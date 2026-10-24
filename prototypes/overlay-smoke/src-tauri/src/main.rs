// PROTOTYPE: throwaway smoke test for "Overlay smoke test on Windows over Nightreign".
// Builds the overlay exactly as the Tauri overlay research recommends, instruments every
// check, and logs to <exe dir>/logs. Not production code: no tests, minimal error handling.
#![windows_subsystem = "windows"]

use serde_json::{json, Value};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicIsize, AtomicU32, Ordering::Relaxed};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tauri::{
    AppHandle, Emitter, Manager, PhysicalPosition, PhysicalSize, WebviewUrl, WebviewWindowBuilder,
    WindowEvent,
};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

// Every action is Ctrl+Shift+<key>; the VK code lets the low-level hook count the same presses.
const ACTIONS: [(&str, Code, u32); 6] = [
    ("toggle_hud", Code::F6, 0x75),
    ("native_reshow", Code::F7, 0x76),
    ("interactive_10s", Code::F8, 0x77),
    ("toggle_stress", Code::F9, 0x78),
    ("toggle_reassert_topmost", Code::F10, 0x79),
    ("mark_issue", Code::F11, 0x7A),
];
const OVERLAY_DELAY_S: u64 = 20;

static RH: [AtomicU32; 6] = [const { AtomicU32::new(0) }; 6]; // RegisterHotKey deliveries
static LL: [AtomicU32; 6] = [const { AtomicU32::new(0) }; 6]; // WH_KEYBOARD_LL observations
static LL_DOWN: [AtomicBool; 6] = [const { AtomicBool::new(false) }; 6];
static OVERLAY: AtomicIsize = AtomicIsize::new(0);
static COMPANION: AtomicIsize = AtomicIsize::new(0);
static GAME: AtomicIsize = AtomicIsize::new(0);
static PREV_FG: AtomicIsize = AtomicIsize::new(0);
static FG_CHANGES: AtomicU32 = AtomicU32::new(0);
static OVERLAY_FG: AtomicU32 = AtomicU32::new(0);
static GAME_LOST_TO_US: AtomicU32 = AtomicU32::new(0);
static Z_SAMPLES: AtomicU32 = AtomicU32::new(0);
static Z_BAD: AtomicU32 = AtomicU32::new(0);
static REASSERT: AtomicBool = AtomicBool::new(false);
static REASSERTS: AtomicU32 = AtomicU32::new(0);
static HUD_ON: AtomicBool = AtomicBool::new(true);
static STRESS: AtomicBool = AtomicBool::new(false);
static INTERACTIVE: AtomicBool = AtomicBool::new(false);
static START: OnceLock<Instant> = OnceLock::new();
static EVENTS: OnceLock<Mutex<File>> = OnceLock::new();
static CSV: OnceLock<Mutex<File>> = OnceLock::new();
static APP: OnceLock<AppHandle> = OnceLock::new();
static LAST_ACTION: Mutex<Option<(String, Instant)>> = Mutex::new(None);

fn secs() -> f64 {
    START.get().map(|s| s.elapsed().as_secs_f64()).unwrap_or(0.0)
}

fn log_event(kind: &str, mut data: Value) {
    let wall = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    data["t"] = json!((secs() * 1000.0).round() / 1000.0);
    data["wall"] = json!(wall);
    data["kind"] = json!(kind);
    if let Some(f) = EVENTS.get() {
        let _ = writeln!(f.lock().unwrap(), "{data}");
    }
    if let Some(app) = APP.get() {
        let _ = app.emit("event", data);
    }
}

fn emit_state() {
    if let Some(app) = APP.get() {
        let _ = app.emit(
            "state",
            json!({
                "hud": HUD_ON.load(Relaxed),
                "stress": STRESS.load(Relaxed),
                "interactive": INTERACTIVE.load(Relaxed),
                "reassert": REASSERT.load(Relaxed),
            }),
        );
    }
}

fn win_info(hwnd: isize) -> Value {
    if hwnd == 0 {
        return Value::Null;
    }
    let h = win::h(hwnd);
    let pid = win::pid_of(h);
    json!({ "hwnd": hwnd, "title": win::title(h), "exe": win::exe_of(pid), "ours": pid == std::process::id() })
}

// Called from the WinEvent hook on every foreground change.
fn on_foreground(fg: isize) {
    FG_CHANGES.fetch_add(1, Relaxed);
    let prev = PREV_FG.swap(fg, Relaxed);
    let (ov, comp) = (OVERLAY.load(Relaxed), COMPANION.load(Relaxed));
    if GAME.load(Relaxed) == 0 {
        GAME.store(win::find_game(), Relaxed);
    }
    let game = GAME.load(Relaxed);
    if ov != 0 && fg == ov {
        OVERLAY_FG.fetch_add(1, Relaxed);
    }
    if game != 0 && prev == game && (fg == ov || fg == comp) {
        GAME_LOST_TO_US.fetch_add(1, Relaxed);
    }
    if REASSERT.load(Relaxed) && ov != 0 && fg == game {
        win::reassert_topmost(ov);
        REASSERTS.fetch_add(1, Relaxed);
    }
    let last = LAST_ACTION.lock().unwrap().clone();
    log_event(
        "foreground",
        json!({
            "to": win_info(fg),
            "from_was_game": game != 0 && prev == game,
            "is_game": game != 0 && fg == game,
            "is_overlay": ov != 0 && fg == ov,
            "is_companion": fg == comp,
            "last_action": last.as_ref().map(|l| &l.0),
            "ms_since_action": last.map(|l| l.1.elapsed().as_millis() as u64),
        }),
    );
}

fn run_action(app: &AppHandle, i: usize, source: &str) {
    let name = ACTIONS[i].0;
    *LAST_ACTION.lock().unwrap() = Some((name.to_string(), Instant::now()));
    let fg_before = win::foreground();
    match name {
        "toggle_hud" => {
            HUD_ON.fetch_xor(true, Relaxed);
        }
        "native_reshow" => {
            let ov = OVERLAY.load(Relaxed);
            if ov != 0 {
                std::thread::spawn(move || {
                    win::show(ov, false);
                    std::thread::sleep(Duration::from_secs(1));
                    win::show(ov, true); // SW_SHOWNA, the research's re-show path
                    std::thread::sleep(Duration::from_millis(300));
                    log_event(
                        "native_reshow_done",
                        json!({ "fg_before": win_info(fg_before), "fg_after": win_info(win::foreground()) }),
                    );
                });
            }
        }
        "interactive_10s" => {
            if let Some(ov) = app.get_webview_window("overlay") {
                if !INTERACTIVE.swap(true, Relaxed) {
                    let _ = ov.set_ignore_cursor_events(false);
                    std::thread::spawn(move || {
                        std::thread::sleep(Duration::from_secs(10));
                        let _ = ov.set_ignore_cursor_events(true);
                        INTERACTIVE.store(false, Relaxed);
                        emit_state();
                        log_event("interactive_end", json!({ "fg": win_info(win::foreground()) }));
                    });
                }
            }
        }
        "toggle_stress" => {
            STRESS.fetch_xor(true, Relaxed);
        }
        "toggle_reassert_topmost" => {
            REASSERT.fetch_xor(true, Relaxed);
        }
        _ => {} // mark_issue: the log line below is the marker
    }
    emit_state();
    log_event(
        "action",
        json!({ "action": name, "source": source, "fg": win_info(fg_before), "snapshot": telemetry() }),
    );
}

#[tauri::command]
fn action(app: AppHandle, name: String) {
    if let Some(i) = ACTIONS.iter().position(|a| a.0 == name) {
        run_action(&app, i, "button");
    }
}

#[tauri::command]
fn page_report(kind: String, data: Value) {
    log_event(&format!("page_{kind}"), data);
}

fn telemetry() -> Value {
    let me = std::process::id();
    let mem = win::tree_mem(me);
    let (ov, game) = (OVERLAY.load(Relaxed), GAME.load(Relaxed));
    let game_alive = game != 0 && win::is_window(game);
    if !game_alive {
        GAME.store(win::find_game(), Relaxed);
    }
    let game = GAME.load(Relaxed);
    let above = if ov != 0 && game != 0 {
        let a = win::above(ov, game);
        Z_SAMPLES.fetch_add(1, Relaxed);
        if !a {
            Z_BAD.fetch_add(1, Relaxed);
        }
        Some(a)
    } else {
        None
    };
    json!({
        "t": (secs() * 10.0).round() / 10.0,
        "hotkeys": ACTIONS.iter().enumerate().map(|(i, a)| json!({
            "name": a.0, "rh": RH[i].load(Relaxed), "ll": LL[i].load(Relaxed)
        })).collect::<Vec<_>>(),
        "foreground": win_info(win::foreground()),
        "fg_changes": FG_CHANGES.load(Relaxed),
        "overlay_became_fg": OVERLAY_FG.load(Relaxed),
        "game_lost_fg_to_us": GAME_LOST_TO_US.load(Relaxed),
        "game": if game != 0 { win::window_report(game) } else { Value::Null },
        "overlay": if ov != 0 { win::window_report(ov) } else { Value::Null },
        "overlay_above_game": above,
        "z_samples": Z_SAMPLES.load(Relaxed),
        "z_bad": Z_BAD.load(Relaxed),
        "reasserts": REASSERTS.load(Relaxed),
        "mem": mem,
        "state": { "hud": HUD_ON.load(Relaxed), "stress": STRESS.load(Relaxed),
                   "interactive": INTERACTIVE.load(Relaxed), "reassert": REASSERT.load(Relaxed) },
    })
}

fn create_overlay(app: &AppHandle) -> tauri::Result<()> {
    log_event("ram_before_overlay", json!({ "mem": win::tree_mem(std::process::id()) }));
    let game = win::find_game();
    GAME.store(game, Relaxed);
    let monitors = app.available_monitors()?;
    let primary = app.primary_monitor()?;
    let (x, y, w, h) = if game != 0 {
        win::monitor_rect(game)
    } else {
        let m = primary.as_ref().expect("no primary monitor");
        (m.position().x, m.position().y, m.size().width as i32, m.size().height as i32)
    };
    let scale = monitors
        .iter()
        .find(|m| m.position().x == x && m.position().y == y)
        .map(|m| m.scale_factor())
        .unwrap_or(1.0);
    let ov = WebviewWindowBuilder::new(app, "overlay", WebviewUrl::App("overlay.html".into()))
        .title("TT overlay (PROTOTYPE)")
        .transparent(true)
        .no_redirection_bitmap(true)
        .decorations(false)
        .shadow(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .focused(false)
        .focusable(false)
        .resizable(false)
        .content_protected(true)
        .position(x as f64 / scale, y as f64 / scale)
        .inner_size(w as f64 / scale, h as f64 / scale)
        .build()?;
    ov.set_position(PhysicalPosition::new(x, y))?;
    ov.set_size(PhysicalSize::new(w as u32, h as u32))?;
    ov.set_ignore_cursor_events(true)?;
    OVERLAY.store(ov.hwnd()?.0 as isize, Relaxed);
    let ov2 = ov.clone();
    ov.on_window_event(move |e| {
        if let WindowEvent::ScaleFactorChanged { scale_factor, .. } = e {
            let _ = ov2.set_size(PhysicalSize::new(w as u32, h as u32));
            log_event("overlay_scale_changed", json!({ "scale": scale_factor }));
        }
    });
    log_event(
        "overlay_created",
        json!({
            "rect": [x, y, w, h], "scale": scale, "on_game_monitor": game != 0,
            "game": win_info(game), "fg": win_info(win::foreground()),
        }),
    );
    let mut csv = CSV.get().unwrap().lock().unwrap();
    let _ = writeln!(csv, "{:.1},overlay_created,,,,,,,,,,", secs());
    Ok(())
}

fn csv_row(t: &Value) -> String {
    let hk = |k: &str| {
        t["hotkeys"]
            .as_array()
            .unwrap()
            .iter()
            .map(|h| h[k].to_string())
            .collect::<Vec<_>>()
            .join("/")
    };
    format!(
        "{},tick,{},{},{},{},{},{},{},{},{},{}",
        t["t"],
        t["mem"]["working_set_mb"],
        t["mem"]["private_mb"],
        t["mem"]["processes"],
        hk("rh"),
        hk("ll"),
        t["overlay_became_fg"],
        t["game_lost_fg_to_us"],
        t["z_bad"],
        t["z_samples"],
        !t["game"].is_null(),
    )
}

fn main() {
    START.set(Instant::now()).unwrap();
    let dir = std::env::current_exe().unwrap().parent().unwrap().join("logs");
    std::fs::create_dir_all(&dir).unwrap();
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let open = |name: String| {
        OpenOptions::new().create(true).append(true).open(dir.join(name)).unwrap()
    };
    EVENTS.set(Mutex::new(open(format!("events-{stamp}.jsonl")))).unwrap();
    let mut csv = open(format!("telemetry-{stamp}.csv"));
    let _ = writeln!(csv, "t,kind,ws_mb,private_mb,procs,rh_f6..f11,ll_f6..f11,overlay_fg,game_lost_fg,z_bad,z_samples,game_found");
    CSV.set(Mutex::new(csv)).unwrap();

    tauri::Builder::default()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, sc, ev| {
                    if ev.state() != ShortcutState::Pressed {
                        return;
                    }
                    if let Some(i) = ACTIONS
                        .iter()
                        .position(|a| sc.matches(Modifiers::CONTROL | Modifiers::SHIFT, a.1))
                    {
                        RH[i].fetch_add(1, Relaxed);
                        run_action(app, i, "hotkey");
                    }
                })
                .build(),
        )
        .invoke_handler(tauri::generate_handler![action, page_report])
        .setup(move |app| {
            let handle = app.handle().clone();
            APP.set(handle.clone()).unwrap();
            if let Some(c) = app.get_webview_window("companion") {
                COMPANION.store(c.hwnd()?.0 as isize, Relaxed);
            }
            log_event("start", json!({ "pid": std::process::id(), "log_dir": dir.display().to_string() }));
            for a in ACTIONS {
                let sc = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), a.1);
                let r = app.global_shortcut().register(sc);
                log_event("hotkey_register", json!({ "action": a.0, "ok": r.is_ok(), "err": r.err().map(|e| e.to_string()) }));
            }
            win::start_hook_thread();

            let h = handle.clone();
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_secs(OVERLAY_DELAY_S));
                if let Err(e) = create_overlay(&h) {
                    log_event("overlay_error", json!({ "err": e.to_string() }));
                }
                std::thread::sleep(Duration::from_secs(20));
                log_event("ram_after_overlay_20s", json!({ "mem": win::tree_mem(std::process::id()) }));
            });

            std::thread::spawn(move || {
                let mut n = 0u64;
                loop {
                    let t = telemetry();
                    if n % 10 == 0 {
                        let _ = writeln!(CSV.get().unwrap().lock().unwrap(), "{}", csv_row(&t));
                    }
                    let _ = handle.emit("telemetry", t);
                    n += 1;
                    std::thread::sleep(Duration::from_secs(1));
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

mod win {
    use serde_json::{json, Value};
    use std::sync::atomic::Ordering::Relaxed;
    use windows::core::{BOOL, PWSTR};
    use windows::Win32::Foundation::*;
    use windows::Win32::Graphics::Gdi::*;
    use windows::Win32::System::Diagnostics::ToolHelp::*;
    use windows::Win32::System::LibraryLoader::GetModuleHandleW;
    use windows::Win32::System::ProcessStatus::*;
    use windows::Win32::System::Threading::*;
    use windows::Win32::UI::Accessibility::*;
    use windows::Win32::UI::Input::KeyboardAndMouse::*;
    use windows::Win32::UI::WindowsAndMessaging::*;

    pub fn h(v: isize) -> HWND {
        HWND(v as *mut _)
    }

    pub fn foreground() -> isize {
        unsafe { GetForegroundWindow().0 as isize }
    }

    pub fn is_window(hwnd: isize) -> bool {
        unsafe { IsWindow(Some(h(hwnd))).as_bool() }
    }

    pub fn title(hwnd: HWND) -> String {
        let mut buf = [0u16; 256];
        let n = unsafe { GetWindowTextW(hwnd, &mut buf) };
        String::from_utf16_lossy(&buf[..n.max(0) as usize])
    }

    pub fn pid_of(hwnd: HWND) -> u32 {
        let mut pid = 0;
        unsafe { GetWindowThreadProcessId(hwnd, Some(&mut pid)) };
        pid
    }

    pub fn exe_of(pid: u32) -> String {
        unsafe {
            let Ok(hp) = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) else {
                return String::new();
            };
            let mut buf = [0u16; 512];
            let mut len = buf.len() as u32;
            let ok = QueryFullProcessImageNameW(hp, PROCESS_NAME_WIN32, PWSTR(buf.as_mut_ptr()), &mut len).is_ok();
            let _ = CloseHandle(hp);
            if !ok {
                return String::new();
            }
            let full = String::from_utf16_lossy(&buf[..len as usize]);
            full.rsplit('\\').next().unwrap_or("").to_string()
        }
    }

    // Matches by exe name first; the exact title is a fallback so a browser tab about the game never matches.
    pub fn find_game() -> isize {
        unsafe extern "system" fn cb(hwnd: HWND, lp: LPARAM) -> BOOL {
            unsafe {
                if !IsWindowVisible(hwnd).as_bool() {
                    return true.into();
                }
                let pid = pid_of(hwnd);
                if pid == std::process::id() {
                    return true.into();
                }
                let exe = exe_of(pid).to_lowercase();
                let t = title(hwnd).trim().to_uppercase();
                if exe == "nightreign.exe" || t == "ELDEN RING NIGHTREIGN" || t == "NIGHTREIGN" {
                    *(lp.0 as *mut isize) = hwnd.0 as isize;
                    return false.into();
                }
                true.into()
            }
        }
        let mut found: isize = 0;
        unsafe {
            let _ = EnumWindows(Some(cb), LPARAM(&mut found as *mut isize as isize));
        }
        found
    }

    // True when `top` is above `other` in the z-order (walking down from `top` reaches `other`).
    pub fn above(top: isize, other: isize) -> bool {
        let mut cur = h(top);
        for _ in 0..20000 {
            match unsafe { GetWindow(cur, GW_HWNDNEXT) } {
                Ok(n) if !n.is_invalid() => {
                    if n.0 as isize == other {
                        return true;
                    }
                    cur = n;
                }
                _ => return false,
            }
        }
        false
    }

    pub fn reassert_topmost(hwnd: isize) {
        unsafe {
            let _ = SetWindowPos(
                h(hwnd),
                Some(HWND_TOPMOST),
                0,
                0,
                0,
                0,
                SWP_NOACTIVATE | SWP_NOMOVE | SWP_NOSIZE,
            );
        }
    }

    pub fn show(hwnd: isize, visible: bool) {
        unsafe {
            let _ = ShowWindow(h(hwnd), if visible { SW_SHOWNA } else { SW_HIDE });
        }
    }

    pub fn monitor_rect(hwnd: isize) -> (i32, i32, i32, i32) {
        unsafe {
            let hm = MonitorFromWindow(h(hwnd), MONITOR_DEFAULTTONEAREST);
            let mut mi = MONITORINFO { cbSize: size_of::<MONITORINFO>() as u32, ..Default::default() };
            let _ = GetMonitorInfoW(hm, &mut mi);
            let r = mi.rcMonitor;
            (r.left, r.top, r.right - r.left, r.bottom - r.top)
        }
    }

    pub fn window_report(hwnd: isize) -> Value {
        unsafe {
            let w = h(hwnd);
            let style = GetWindowLongPtrW(w, GWL_STYLE) as u32;
            let ex = GetWindowLongPtrW(w, GWL_EXSTYLE) as u32;
            let mut r = RECT::default();
            let _ = GetWindowRect(w, &mut r);
            let (mx, my, mw, mh) = monitor_rect(hwnd);
            let mut aff = 0u32;
            let _ = GetWindowDisplayAffinity(w, &mut aff);
            let covers_monitor = r.left == mx && r.top == my && r.right - r.left == mw && r.bottom - r.top == mh;
            json!({
                "title": title(w),
                "exe": exe_of(pid_of(w)),
                "rect": [r.left, r.top, r.right - r.left, r.bottom - r.top],
                "monitor": [mx, my, mw, mh],
                "mode_guess": if style & WS_CAPTION.0 == WS_CAPTION.0 { "windowed" }
                              else if covers_monitor { "borderless_or_fullscreen" } else { "popup" },
                "visible": IsWindowVisible(w).as_bool(),
                "ex": {
                    "topmost": ex & WS_EX_TOPMOST.0 != 0,
                    "noactivate": ex & WS_EX_NOACTIVATE.0 != 0,
                    "transparent": ex & WS_EX_TRANSPARENT.0 != 0,
                    "layered": ex & WS_EX_LAYERED.0 != 0,
                    "noredirectionbitmap": ex & WS_EX_NOREDIRECTIONBITMAP.0 != 0,
                },
                "display_affinity": format!("{aff:#x}"),
            })
        }
    }

    // Working set and private bytes summed over our process and every descendant (WebView2 browser, GPU, renderers).
    pub fn tree_mem(root: u32) -> Value {
        let mut procs: Vec<(u32, u32, String)> = Vec::new();
        unsafe {
            if let Ok(snap) = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) {
                let mut e = PROCESSENTRY32W { dwSize: size_of::<PROCESSENTRY32W>() as u32, ..Default::default() };
                if Process32FirstW(snap, &mut e).is_ok() {
                    loop {
                        let n = e.szExeFile.iter().position(|&c| c == 0).unwrap_or(0);
                        procs.push((e.th32ProcessID, e.th32ParentProcessID, String::from_utf16_lossy(&e.szExeFile[..n])));
                        if Process32NextW(snap, &mut e).is_err() {
                            break;
                        }
                    }
                }
                let _ = CloseHandle(snap);
            }
        }
        let mut tree = vec![root];
        let mut i = 0;
        while i < tree.len() {
            let p = tree[i];
            for (pid, ppid, _) in &procs {
                if *ppid == p && *pid != p && !tree.contains(pid) {
                    tree.push(*pid);
                }
            }
            i += 1;
        }
        let (mut ws, mut private) = (0u64, 0u64);
        let mut detail = Vec::new();
        for pid in &tree {
            unsafe {
                let Ok(hp) = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION | PROCESS_VM_READ, false, *pid) else {
                    continue;
                };
                let mut pmc = PROCESS_MEMORY_COUNTERS_EX { cb: size_of::<PROCESS_MEMORY_COUNTERS_EX>() as u32, ..Default::default() };
                if GetProcessMemoryInfo(hp, &mut pmc as *mut _ as *mut PROCESS_MEMORY_COUNTERS, pmc.cb).is_ok() {
                    ws += pmc.WorkingSetSize as u64;
                    private += pmc.PrivateUsage as u64;
                    let name = procs.iter().find(|p| p.0 == *pid).map(|p| p.2.clone()).unwrap_or_default();
                    detail.push(json!({ "pid": pid, "exe": name, "ws_mb": pmc.WorkingSetSize / 1_048_576, "private_mb": pmc.PrivateUsage / 1_048_576 }));
                }
                let _ = CloseHandle(hp);
            }
        }
        json!({
            "working_set_mb": ws / 1_048_576,
            "private_mb": private / 1_048_576,
            "processes": tree.len(),
            "detail": detail,
        })
    }

    unsafe extern "system" fn kb_proc(code: i32, wp: WPARAM, lp: LPARAM) -> LRESULT {
        unsafe {
            if code >= 0 {
                let k = &*(lp.0 as *const KBDLLHOOKSTRUCT);
                let msg = wp.0 as u32;
                if let Some(i) = super::ACTIONS.iter().position(|a| a.2 == k.vkCode) {
                    if msg == WM_KEYDOWN || msg == WM_SYSKEYDOWN {
                        if !super::LL_DOWN[i].swap(true, Relaxed)
                            && GetAsyncKeyState(VK_CONTROL.0 as i32) < 0
                            && GetAsyncKeyState(VK_SHIFT.0 as i32) < 0
                        {
                            super::LL[i].fetch_add(1, Relaxed);
                        }
                    } else if msg == WM_KEYUP || msg == WM_SYSKEYUP {
                        super::LL_DOWN[i].store(false, Relaxed);
                    }
                }
            }
            CallNextHookEx(None, code, wp, lp)
        }
    }

    unsafe extern "system" fn fg_proc(_: HWINEVENTHOOK, _: u32, hwnd: HWND, _: i32, _: i32, _: u32, _: u32) {
        super::on_foreground(hwnd.0 as isize);
    }

    // Passive observers only: a WH_KEYBOARD_LL hook (the hotkey fallback candidate) and a foreground WinEvent hook.
    pub fn start_hook_thread() {
        std::thread::spawn(|| unsafe {
            let hinst = GetModuleHandleW(None).ok().map(|m| HINSTANCE(m.0));
            let kb = SetWindowsHookExW(WH_KEYBOARD_LL, Some(kb_proc), hinst, 0);
            let ev = SetWinEventHook(EVENT_SYSTEM_FOREGROUND, EVENT_SYSTEM_FOREGROUND, None, Some(fg_proc), 0, 0, WINEVENT_OUTOFCONTEXT);
            super::log_event("hooks", json!({ "keyboard_ll_ok": kb.is_ok(), "foreground_winevent_ok": !ev.is_invalid() }));
            let mut msg = MSG::default();
            while GetMessageW(&mut msg, None, 0, 0).as_bool() {
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        });
    }
}
