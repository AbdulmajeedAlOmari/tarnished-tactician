# PROTOTYPE: overlay smoke test (throwaway)

Answers [Overlay smoke test on Windows over Nightreign](https://github.com/AbdulmajeedAlOmari/tarnished-tactician/issues/19): does a minimal Tauri 2 overlay behave correctly over Nightreign on real Windows hardware? It lives only on the `prototype/overlay-smoke-test` branch and is never merged.

The overlay is built as [Tauri 2 overlay window capabilities on Windows](https://github.com/AbdulmajeedAlOmari/tarnished-tactician/issues/3) recommends: transparent, `noRedirectionBitmap`, decorations and shadow off, always on top, skip taskbar, `focused(false)`, `focusable(false)`, `set_ignore_cursor_events(true)`, `contentProtected`. It is created once and never `show()`n again. Hotkeys come from `tauri-plugin-global-shortcut` (`RegisterHotKey`).

## Build (WSL, cross-compiled)

```sh
# clang/lld/llvm 22.1.3 unpacked from the Arch archive into ~/.local/opt/llvm (no root)
export PATH=$HOME/.local/opt/llvm/usr/bin:$PATH LD_LIBRARY_PATH=$HOME/.local/opt/llvm/usr/lib
cd prototypes/overlay-smoke/src-tauri
cargo xwin build --release --target x86_64-pc-windows-msvc
cp target/x86_64-pc-windows-msvc/release/tt-overlay-smoke.exe /mnt/c/tt-smoke/
```

## What it measures

| Check | How |
| --- | --- |
| Hotkey delivery with the game focused | Ctrl+Shift+F6…F11 are counted twice: by `RegisterHotKey` (RH) and by a passive `WH_KEYBOARD_LL` hook (LL). If LL counts a press and RH doesn't, `RegisterHotKey` failed and the hook is a working fallback. |
| No focus stealing | A `EVENT_SYSTEM_FOREGROUND` WinEvent hook logs every foreground change, with the action just before it. "overlay became fg" and "game lost fg to us" should stay 0. |
| Click-through | The overlay page counts every pointer/wheel event it gets. This should stay 0. F8 turns click-through off for 10 s to test whether clicks also reach the game. |
| Game never over the overlay | Once a second it walks the z-order to check the overlay is above the game window. F10 turns on putting the overlay back on top (`HWND_TOPMOST` + `SWP_NOACTIVATE`) on every foreground change. |
| Transparency over a long session | Seen by eye: a magenta frame plus a translucent HUD. F9 repaints at 60 fps. F11 marks the moment something looks wrong. |
| RAM of the second WebView2 window | Working set + private bytes summed over the process tree. Logged before the overlay exists, at creation, 20 s later and every 10 s. |
| Real window styles | The overlay's actual `WS_EX_*` flags and display affinity (`0x11` = excluded from capture). |

Logs: `C:\tt-smoke\logs\events-<ts>.jsonl` (every event) and `telemetry-<ts>.csv` (every 10 s).

## Test script (Nightreign in Borderless Windowed on the primary monitor)

1. Start Nightreign (Borderless Windowed) and reach the Roundtable Hold.
2. Run `C:\tt-smoke\tt-overlay-smoke.exe`, then **click straight back into the game**. The overlay appears 20 s later. The game should keep focus: camera and movement keep responding, with no stutter or minimise.
3. **Hotkeys:** with the game focused, press each of Ctrl+Shift+F6…F11 once. Check that RH equals LL on the HUD, and note whether the game also reacted to any of the keys.
4. **Focus on update:** press F6 (HUD off/on) a few times, then F7 (native hide → `SW_SHOWNA` re-show), then F9 (stress) and play one fight with it on. The game must never lose focus.
5. **Click-through:** attack or click with the mouse over the HUD panel and over the frame. The game must react, and "mouse events seen by overlay" must stay 0.
6. **Interactive leak:** press F8, then click the red panel's button. Does the game *also* react (attack, camera)? Does it keep focus?
7. **Z-order:** alt-tab out and back, open the Steam overlay (Shift+Tab), open game menus, and go through loading screens and an expedition start/end. The HUD must stay visible over the game; watch "z-order bad". If it goes above 0, press F10 and repeat.
8. **Capture exclusion:** take a Steam F12 screenshot and a Win+Shift+S snip. The overlay must be absent from both.
9. **Long session:** keep it running through normal play, at least 1 h and ideally 2. Also lock the screen (Win+L) and come back once. Press **F11 the moment** the transparent area turns black, flickers or the overlay vanishes.
10. Close the companion window to quit. Then report what you saw in steps 2, 3, 6, 7, 8 and 9.
