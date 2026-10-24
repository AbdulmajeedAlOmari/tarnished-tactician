# Tauri 2 overlay window capabilities on Windows

Research for ticket [#3](https://github.com/AbdulmajeedAlOmari/tarnished-tactician/issues/3). Researched 2026-10-24.

**Question:** can Tauri 2 on Windows give us the overlay window we need? With which APIs, plugins and caveats? The window must be transparent, frameless and always on top over a borderless game. Click-through must toggle at runtime. It must never steal focus. Global hotkeys must work while the game has focus. Placement must be correct on multi-monitor and mixed-DPI setups. A second WebView2 window must cost little when idle.

## Versions pinned (crates.io / npm, as of 2026-10-24)

| Package | Version | Notes |
| --- | --- | --- |
| `tauri` | 2.12.5 | depends on `tauri-runtime-wry ~2.12.3`, `tauri-utils ~2.10.2`, `windows ^0.62`, `webview2-com ^0.39` |
| `tao` | 0.37.2 | windowing backend (Win32) |
| `wry` | 0.57.1 | WebView2 backend |
| `tauri-utils` | 2.10.2 | `tauri.conf.json` window schema |
| `tauri-plugin-global-shortcut` | 2.4.1 (JS `@tauri-apps/plugin-global-shortcut` 2.4.1) | depends on `global-hotkey 0.8` (resolves to 0.8.0; 0.9.0 exists, not yet adopted) |
| `tauri-plugin-window-state` | 2.5.0 | |
| `@tauri-apps/api` | 2.12.4 | |
| `windows` | 0.62.2 | same major as Tauri's own dependency, so no duplicate crate |
| `webview2-com` | 0.39.1 | only if we touch WebView2 COM directly |

Sources: the crates.io API (`/api/v1/crates/<name>` and `/<name>/<version>/dependencies`) and the npm registry. Every source statement below was checked against the published crate tarballs for these exact versions. For each one, `docs.rs/crate/<name>/<version>/source/<path>` shows the same file.

## Requirement-by-requirement findings

### 1. Transparent, frameless, always-on-top

All of this is supported by Tauri config or builder flags. No extra code is needed.

- The `WindowConfig` fields `transparent`, `decorations`, `alwaysOnTop`, `skipTaskbar`, `shadow`, `focus`, `focusable`, `contentProtected` and `noRedirectionBitmap` all exist in [`tauri-utils` 2.10.2 `src/config.rs`](https://docs.rs/crate/tauri-utils/2.10.2/source/src/config.rs). `WebviewWindowBuilder` has a matching builder method for each one ([tauri 2.12.5 `src/webview/webview_window.rs`](https://docs.rs/crate/tauri/2.12.5/source/src/webview/webview_window.rs)).
- `alwaysOnTop` maps to `WS_EX_TOPMOST` / `HWND_TOPMOST` ([tao 0.37.2 `platform_impl/windows/window_state.rs`](https://docs.rs/crate/tao/0.37.2/source/src/platform_impl/windows/window_state.rs)).
- **How transparency works on Windows.** By default tao makes a transparent window by calling `DwmEnableBlurBehindWindow` with an empty region ([tao `window.rs` `on_create`](https://docs.rs/crate/tao/0.37.2/source/src/platform_impl/windows/window.rs)). wry then sets the WebView2 default background to `(0,0,0,0)` through `ICoreWebView2ControllerOptions3::SetDefaultBackgroundColor` / `set_background_color` ([wry 0.57.1 `src/webview2/mod.rs`](https://docs.rs/crate/wry/0.57.1/source/src/webview2/mod.rs)).
- **New in tauri 2.12: `noRedirectionBitmap`.** It sets `WS_EX_NOREDIRECTIONBITMAP`, and tao then skips the blur-behind hack (`if transparent && !no_redirection_bitmap`). PR [tauri#15410](https://github.com/tauri-apps/tauri/pull/15410) merged it on 2026-07-03, to fix the white flash on the first frame of transparent windows. Microsoft's definition of the style: "does not render to a redirection surface" ([Extended Window Styles](https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles)).
- **The WebView2 background alpha is all or nothing.** Per the `background_color` docs in `tauri-utils`, the webview layer ignores any alpha that is not 0. wry forces any non-zero alpha to 255. So the page background is either fully transparent or opaque. Semi-transparent panels must be drawn with CSS `rgba()` inside the page, which works because the page composites over a transparent base.
- Set `shadow: false`. With `shadow: true`, an undecorated Windows window gets a 1px white border and, on Windows 11, rounded corners (`tauri-utils` `shadow` docs).

### 2. Click-through, toggled at runtime

This is supported, but only for the whole window.

- `WebviewWindow::set_ignore_cursor_events(bool)` (JS `getCurrentWindow().setIgnoreCursorEvents()`, permission `core:window:allow-set-ignore-cursor-events`) toggles `WS_EX_TRANSPARENT | WS_EX_LAYERED` at runtime (tao `window_state.rs`, `IGNORE_CURSOR_EVENT`).
- Microsoft's documentation of this behaviour: "if the layered window has the **WS_EX_TRANSPARENT** extended window style, the shape of the layered window will be ignored and the mouse events will be passed to other windows underneath" ([Window Features: layered windows](https://learn.microsoft.com/en-us/windows/win32/winmsg/window-features)).
- **There is no per-pixel or per-region click-through.** The WebView2 surface does not pass clicks through transparent pixels. The Electron-style `forward` option, which would let the page receive mouse-move events while ignoring clicks, is still an open request ([tauri#6164](https://github.com/tauri-apps/tauri/issues/6164)). So does a webview-level version of the API ([tauri#10564](https://github.com/tauri-apps/tauri/issues/10564)). Apps that want "interactive panel, click-through elsewhere" poll the cursor and toggle the flag. That pattern is associated with a known Windows 10 composition race, covered under Caveats ([tauri#15947](https://github.com/tauri-apps/tauri/issues/15947)).

### 3. No focus stealing

This is supported, with one important trap.

- `focusable(false)` / config `focusable: false` adds `WS_EX_NOACTIVATE` (tao `window_state.rs`: `if !FOCUSABLE { style_ex |= WS_EX_NOACTIVATE }`). Runtime `set_focusable(bool)` exists too (`core:window:allow-set-focusable`).
  - Microsoft: a window with this style "does not become the foreground window when the user clicks it… should not be activated through programmatic access" ([Extended Window Styles](https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles)).
  - This replaces hand-written Win32 for `WS_EX_NOACTIVATE`. The feature landed for [tauri#11130](https://github.com/tauri-apps/tauri/issues/11130) and [tauri#13546](https://github.com/tauri-apps/tauri/issues/13546).
- `focused(false)` on `WebviewWindowBuilder` sets the window and the webview to unfocused. wry then skips `controller.MoveFocus` at creation (`if attributes.focused`). On the window side, it shows the window with `SW_SHOWNOACTIVATE` the first time.
- **Trap: later `show()` calls use `SW_SHOW`.** tao clears `MARKER_DONT_FOCUS` once creation finishes ([tao `window.rs`](https://docs.rs/crate/tao/0.37.2/source/src/platform_impl/windows/window.rs), end of `init`), so every later `show()` uses `ShowWindow(SW_SHOW)`. Microsoft documents `SW_SHOW` as "Activates the window and displays it" ([ShowWindow](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)).
  - `WS_EX_NOACTIVATE` should block that activation. That is my reading of the style docs, not a tested result.
  - The safer design avoids the question entirely: **create the overlay once, keep it shown, and "hide" it by rendering nothing.** With click-through on, an empty transparent window is invisible and inert.
  - If we ever do need to hide and re-show the native window, call `ShowWindow(hwnd, SW_SHOWNA)` through the `windows` crate (`WebviewWindow::hwnd()`) instead of `show()`.
- **Trap: `tauri-plugin-window-state` calls `set_focus()`.** When the plugin restores `StateFlags::VISIBLE`, it calls `self.show()` and then `self.set_focus()` ([plugin 2.5.0 `src/lib.rs`, `restore_state`](https://docs.rs/crate/tauri-plugin-window-state/2.5.0/source/src/lib.rs)). Put the overlay on its denylist (`Builder::with_denylist(&["overlay"])`) and use the plugin only for the companion window.
- **Do not use `set_fullscreen` for the overlay.** tao force-activates fullscreen windows ("for fullscreen windows they must be activated (i.e. focused) so that they appear on top of the taskbar"; `force_window_active` after `set_fullscreen`). Size the window to the monitor rect instead.
- **Side effect of `focusable: false`: the page gets no keyboard input** ([tauri#14386](https://github.com/tauri-apps/tauri/issues/14386), open). Hotkeys come from the global-shortcut plugin, so the overlay doesn't need keyboard focus. That suits us.

### 4. Global hotkeys while the game has focus

Supported, but must be validated against Nightreign.

- `tauri-plugin-global-shortcut` 2.4.1 uses `global-hotkey` 0.8.0. On Windows that crate calls `RegisterHotKey(hwnd, id, mods | MOD_NOREPEAT, vk)` on a hidden message-only window and handles `WM_HOTKEY` ([global-hotkey 0.8.0 `src/platform_impl/windows/mod.rs`](https://docs.rs/crate/global-hotkey/0.8.0/source/src/platform_impl/windows/mod.rs)). That is a system-wide hotkey, so it fires whichever window has focus ([RegisterHotKey](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerhotkey)). It doesn't touch the game process.
- Register from Rust (`app.global_shortcut().on_shortcut(...)`) so the webview needs no `global-shortcut:*` permissions. The plugin grants none by default ([plugin docs](https://v2.tauri.app/plugin/global-shortcut/), `permissions/default.toml`).
- Microsoft constraints: registration fails if another app already holds the combination, and F12 is reserved and must not be registered ([RegisterHotKey](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerhotkey)). The settings UI needs to report registration failures.
- **Evidence from games:** [tauri#7318](https://github.com/tauri-apps/tauri/issues/7318) (open) reports global shortcuts **working in Elden Ring**, Apex and Trackmania, but **not in Dark Souls 2/3** in fullscreen. The reporter suspects exclusive fullscreen or DirectInput. Nightreign runs on the Elden Ring engine, so this is encouraging but not proof. It needs a hands-on check.
- Fallback if `RegisterHotKey` doesn't fire in Nightreign: a passive `WH_KEYBOARD_LL` hook (for example via `SetWindowsHookExW` in the `windows` crate). It only observes input and injects nothing, so it stays within our "no automated input" constraint. Keep it as plan B.

### 5. Multi-monitor and mixed-DPI placement

Supported, with one known behaviour to handle.

- tao makes the process per-monitor DPI aware v2 (`SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2)`, falling back to v1; [tao `dpi.rs`](https://docs.rs/crate/tao/0.37.2/source/src/platform_impl/windows/dpi.rs)).
- Tauri exposes `available_monitors()`, `current_monitor()`, `monitor_from_point()` and `scale_factor()`, plus `set_position` / `set_size` that take `PhysicalPosition` / `PhysicalSize` ([tauri 2.12.5 `src/window/mod.rs`](https://docs.rs/crate/tauri/2.12.5/source/src/window/mod.rs)). The overlay should cover the game's monitor, using that monitor's physical `position()` / `size()`.
- **Behaviour to handle:** when a window crosses onto a monitor with a different scale, tao's `WM_DPICHANGED` handler keeps the window's *logical* size, so its physical size changes ([tao `event_loop.rs`, `WM_DPICHANGED`](https://docs.rs/crate/tao/0.37.2/source/src/platform_impl/windows/event_loop.rs)). To place the overlay:
  1. `set_position(target.position())`.
  2. On `WindowEvent::ScaleFactorChanged` (or right away if the scale is unchanged), `set_size(target.size())`.
  3. Check with `outer_position()` / `outer_size()`.
- Re-run placement when the monitor setup changes or the game window moves.
- Finding which monitor the game is on is a separate decision for the "first-run setup & calibration" ticket. One option: enumerate top-level windows and call `MonitorFromWindow` on the game's HWND. That reads no game memory.

### 6. Idle cost of a second WebView2 window

Acceptable, but must be measured.

- tauri-runtime-wry keeps one `WebContext` per data directory and shares it between webviews ([tauri-runtime-wry 2.12.3 `src/lib.rs`, `WebContextStore`](https://docs.rs/crate/tauri-runtime-wry/2.12.3/source/src/lib.rs)). wry builds each environment with the same options and data folder. Microsoft: environments "configured the same way (including sharing the same user data folder)… represent the same user data folder and the same associated collection of processes" ([WebView2 process model](https://learn.microsoft.com/en-us/microsoft-edge/webview2/concepts/process-model)). So the overlay **shares the companion's browser and GPU processes**, and at most adds a renderer. Both pages are same-origin app assets, so they may even share one renderer, depending on site isolation.
- Memory controls: wry implements `SetMemoryUsageTargetLevel` (`ICoreWebView2_19`), but Tauri 2.12.5 has no public API for it. We can reach it through `WebviewWindow::with_webview(|w| w.controller())` plus `webview2-com`. Microsoft recommends `Low` for inactive WebViews, and `TrySuspend` for long idle periods ([WebView2 performance best practices](https://learn.microsoft.com/en-us/microsoft-edge/webview2/concepts/performance)). Apply this to the **companion** window while a Run is active, not to the overlay.
- `backgroundThrottling` is unsupported on Windows (`tauri-utils` docs), so don't count on it.
- Keep the overlay page light: no polling, CSS animations only, and an update only when a Recommendation changes.
- No figures exist yet. Measure the second window's private bytes in the spike against the ~300 MB RAM target.

### 7. Composing over the game, and over our own screen capture

- Overlays only work over **borderless or windowed** games, or exclusive-fullscreen games that Windows "Fullscreen Optimizations" turns into a composed window. Microsoft: in fullscreen exclusive, an outside overlay "would have to step into and intercept the rendering process". With FSO, "when an overlay appears, DWM takes control back", at "a slight performance overhead" ([Demystifying Fullscreen Optimizations](https://devblogs.microsoft.com/directx/demystifying-full-screen-optimizations/)). Injection is ruled out, so the overlay requires borderless or windowed mode. When the game can't be overlaid, the companion window is the fallback.
- **Interaction with screen-capture OCR.** If detection captures the whole monitor, it will see our own overlay. `contentProtected: true` / `set_content_protected(true)` calls `SetWindowDisplayAffinity(WDA_EXCLUDEFROMCAPTURE)` (tao `window.rs`). That removes the overlay from captures on Windows 10 2004 and later ([SetWindowDisplayAffinity](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowdisplayaffinity)).
  - The downside: it also hides the overlay from the user's own recordings and streams (OBS, Game Bar).
  - Alternatively, detection can capture only the game's window rather than the monitor.

## Minimal recommended approach

Create the overlay from Rust once at startup. Don't define it in `tauri.conf.json`, so its flags stay together in one place.

```rust
// src-tauri: tauri 2.12.5
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

let overlay = WebviewWindowBuilder::new(app, "overlay", WebviewUrl::App("overlay.html".into()))
    .title("Tarnished Tactician overlay")
    .transparent(true)
    .no_redirection_bitmap(true) // WS_EX_NOREDIRECTIONBITMAP; skips blur-behind hack (2.12+)
    .decorations(false)
    .shadow(false)
    .always_on_top(true)         // WS_EX_TOPMOST
    .skip_taskbar(true)
    .resizable(false)
    .focused(false)              // first show uses SW_SHOWNOACTIVATE; webview not focused
    .focusable(false)            // WS_EX_NOACTIVATE
    .content_protected(true)     // WDA_EXCLUDEFROMCAPTURE (make this a user setting)
    .visible(false)
    .build()?;
overlay.set_ignore_cursor_events(true)?; // WS_EX_LAYERED | WS_EX_TRANSPARENT
// place on the game's monitor: set_position(PhysicalPosition), then set_size(PhysicalSize)
// after any ScaleFactorChanged; then show() ONCE and never hide()/show() again.
```

```css
/* overlay.html */
html, body { background: transparent; margin: 0; }
.panel { background: rgba(10, 10, 14, 0.72); } /* translucency via CSS, not window alpha */
```

- **Hotkeys:** register with `tauri-plugin-global-shortcut` 2.4.1 from Rust: toggle overlay visibility (in the page's render state), dismiss/undo detection, and toggle interactive mode.
- **Interactive mode (rare, user-initiated):** `set_ignore_cursor_events(false)`. Optionally also `set_focusable(true)` plus `set_focus()` when the user really wants keyboard input. That deliberately takes focus from the game. Turn both back off on exit. Don't toggle click-through on every cursor move.
- **Window state:** `tauri-plugin-window-state` 2.5.0 for the companion only, `.with_denylist(&["overlay"])`.
- **Direct Win32 via `windows` 0.62:** only `ShowWindow(SW_SHOWNA)` if we ever re-show the native window. Optionally `SetWindowPos(HWND_TOPMOST, …, SWP_NOACTIVATE|SWP_NOMOVE|SWP_NOSIZE)` to put the overlay back on top if the game ever covers it (see caveats). Optionally `SetMemoryUsageTargetLevel` on the companion through `with_webview`. Nothing else needs hand-written Win32. Tauri already exposes `WS_EX_NOACTIVATE`, `WS_EX_TRANSPARENT`/`WS_EX_LAYERED`, `WS_EX_TOPMOST`, `WS_EX_NOREDIRECTIONBITMAP` and display affinity.

## Caveats and open issues

1. **Transparency corruption over long sessions** (open):
   - [tauri#15947](https://github.com/tauri-apps/tauri/issues/15947): on Windows 10, transparent areas or the wallpaper occasionally turn black when `set_ignore_cursor_events` is toggled often or the window is minimized. Blamed on the blur-behind and layered toggling, on tao 0.34.8.
   - [tauri#16341](https://github.com/tauri-apps/tauri/issues/16341): on Windows 11, a long-lived transparent window develops a black background, possibly after display changes. That report is on tauri 2.3.1 and has no reproduction.
   - Mitigations: `noRedirectionBitmap: true` (which skips the blur-behind path #15947 blames), rare click-through toggles, never minimizing the overlay, and recreating the overlay window on display-change events if the spike reproduces the problem.
2. **Clicks in interactive mode may reach the game.** `WS_EX_NOACTIVATE` keeps the game in the foreground. By default, raw input goes to the window that "has the window focus" ([RAWINPUTDEVICE](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-rawinputdevice)). So a game using raw mouse input may also act on clicks meant for the overlay. This is my inference, not tested. **Design the overlay as display-only plus hotkeys during play.** Mouse interaction belongs to the companion window, or to an explicit focus-taking edit mode.
3. **Z-order against the game:** both windows can be topmost. Among topmost windows, the most recently activated one can come to the front. If the spike shows the game covering the overlay, re-assert `HWND_TOPMOST` with `SWP_NOACTIVATE` on foreground changes (for example via `SetWinEventHook(EVENT_SYSTEM_FOREGROUND)`). This is an inference, not tested.
4. **Hotkeys:** `RegisterHotKey` is not proven against Nightreign specifically ([tauri#7318](https://github.com/tauri-apps/tauri/issues/7318) has Elden Ring working and DS2/DS3 failing). Conflicts with other apps make registration fail. A registered combination is probably consumed and never reaches the game (inference), so defaults should avoid the game's bindings. That belongs to the "Settings & hotkeys" ticket.
5. **`focusable: false` means no keyboard input in the overlay page** ([tauri#14386](https://github.com/tauri-apps/tauri/issues/14386)). Fine for us, but text inputs belong in the companion.
6. **Exclusive fullscreen:** if the game is in exclusive fullscreen without FSO, the overlay won't show. First-run setup should check the window mode and fall back to the companion.
7. **No public Tauri API for WebView2 memory levels.** `with_webview` plus `webview2-com` 0.39 is the escape hatch, and it is unsafe COM code.
8. **The `content_protected` trade-off:** it keeps the overlay out of our OCR, but also out of the user's captures and streams. Expose it as a setting, or capture only the game window.

## Verdict

**No concrete blocker.** Tauri 2.12.5 covers every requirement through built-in builder or config flags:

- transparency, plus `noRedirectionBitmap`
- `alwaysOnTop`
- `focusable: false` → `WS_EX_NOACTIVATE`
- `set_ignore_cursor_events` → `WS_EX_TRANSPARENT | WS_EX_LAYERED`
- `contentProtected` → `WDA_EXCLUDEFROMCAPTURE`
- monitor and DPI APIs

Global hotkeys come from `tauri-plugin-global-shortcut` 2.4.1 (`RegisterHotKey`). Hand-written Win32 shrinks to at most `ShowWindow(SW_SHOWNA)` and an optional topmost re-assert. The settled stack stands.

The open risks are things only a hands-on test can settle, not missing APIs:

- hotkey delivery inside Nightreign
- whether clicks on the overlay leak to the game in interactive mode
- z-order against the game's borderless window
- transparency stability over long sessions on Windows 10 and 11
- the measured RAM of the second window

All of these belong in a short overlay smoke-test spike on a real Windows machine with Nightreign running in borderless mode.

## Implications for the map

- **Suggested new prototype/spike ticket: "Overlay smoke test on Windows + Nightreign".** Validate hotkey delivery, no focus stealing, click leakage in interactive mode, z-order, `noRedirectionBitmap` transparency over a 2h session, and the second window's RAM. This also feeds the "Testing strategy" overlay smoke tests.
- **Overlay UX (wherever it's decided):** the overlay is display-only plus hotkeys during play. Mouse interaction goes in the companion, or in an explicit focus-taking edit mode. No text input in the overlay.
- **Detection spike / capture ticket:** decide between monitor capture with `contentProtected` on the overlay (which hides the overlay from user recordings) and capturing only the game window, so OCR never sees our own overlay.
- **First-run setup & calibration:** must detect borderless/windowed vs exclusive fullscreen, and must find the game's monitor for overlay placement.
- **Settings & hotkeys:** handle `RegisterHotKey` failures (combination already taken, F12 reserved), avoid game-bound keys, and consider a `WH_KEYBOARD_LL` fallback.
