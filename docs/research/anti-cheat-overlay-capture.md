# Anti-cheat, ToS and window-mode constraints for an external overlay and screen capture

Research for wayfinder ticket #2. Researched 2026-10-24.

**Question.** Can an external, always-on-top overlay window plus screen capture of *Elden Ring Nightreign* coexist with its anti-cheat and terms? Which game window modes make it work?

**Evidence grades used below**
- **[P]** Primary source: the vendor's own docs, EULA, store page or source code.
- **[C]** Community source: wikis, forums, tool READMEs. Useful, but it is someone's report, not a guarantee.
- **[I]** My own inference from the sources. It is not stated anywhere.

---

## 1. Which anti-cheat Nightreign ships

- The Steam store page lists "Uses Kernel Level Anti-Cheat" and "Easy Anti-Cheat - Requires manual removal after game uninstall". It also lists "Incorporates 3rd-party DRM: GuardIT" (anti-tamper) and "Requires agreement to a 3rd-party EULA". **[P]** [Steam store page, app 2622380](https://store.steampowered.com/app/2622380/ELDEN_RING_NIGHTREIGN/)
- PCGamingWiki lists Easy Anti-Cheat (EAC) as the middleware. It says: "The embedded Easy Anti-Cheat component prevents unknown third-party tools from injecting themselves in the game… Online play will be disabled when Easy Anti-Cheat is disabled." **[C]** [PCGamingWiki: Elden Ring Nightreign](https://www.pcgamingwiki.com/wiki/Elden_Ring_Nightreign)
- The game uses Direct3D 12 only. **[C]** (PCGamingWiki, API section)

## 2. What EAC says it does about overlays, capture, hotkeys and companion apps

The player FAQ at easy.ac is EAC's own statement. **[P]** [Easy Anti-Cheat player support articles](https://www.easy.ac/en-us/support/articles)

- **What gets blocked:** "EAC blocks software that interacts with games in ways that look identical to cheating techniques. On rare occasions, this could include legitimate software whose behavior relies on special access to the game process or system - such as overlays, hardware monitoring and performance tools, or automation software."
- **Its suggested fixes point at process access:** "Check if the software has a 'game mode' that disables memory injection features" and "Add exceptions in the software's settings to exclude the game process". Both assume the conflicting tool is touching the game process.
- **Modding tools:** "Many modding tools modify game files or inject code into the running game process. Because these same methods can also be used to create cheat software, anti-cheat systems may block them in multiplayer environments."
- **Vulnerable drivers:** EAC may ask you to close software "that use[s] outdated kernel drivers", most often hardware-monitoring or RGB tools.
- **Who decides on a ban:** "When EAC flags an account for cheating, the game publisher may subsequently decide to suspend or ban the account." EAC does not disclose which detection triggered a ban.
- **Lifetime:** "EAC does not run at boot or in the background. It only starts when a protected game is launched and stops when the protected game is closed."
- **Game-specific requirements:** games can impose "additional security requirements" (Secure Boot, TPM, IOMMU). None of these affect a separate user-mode app. **[I]**

**What EAC does *not* say anywhere I could find:** nothing about separate top-most or transparent windows, Windows Graphics Capture, DXGI Desktop Duplication, or `RegisterHotKey`. Every category the FAQ names as at risk depends on "special access to the game process or system": injection, memory access, vulnerable kernel drivers, or automation. **[P]** for what it lists; **[I]** for the conclusion that an app with none of these behaviours is outside the stated risk.

Epic's own help article on "Game Security Violation Detected" is consistent with this. It blames "outdated or insecure background third-party software" and tells you to close it. I could only see the search-indexed summary because the page returned 403. **[P, partial]** [Epic Games help: Game Security Violation Detected](https://www.epicgames.com/help/en-US/c-202300000001639/c-202300000001736/easy-anti-cheat-error-game-security-violation-detected-a202300000015740)

## 3. Precedents: external overlays and OCR tools already used with Nightreign

Several community tools already do what Tarnished Tactician plans to do: an external window plus screen OCR, with EAC on, online. **All [C].** These are author claims, not independent verification.

| Tool | What it does | Anti-cheat claim | Window mode |
|---|---|---|---|
| [Nightreign Armament Helper](https://github.com/nextfinish/NightreignArmamentHelper) (also Nexus mod 193, "Online-safe") | OCR reads menus and armament names, then draws recommendation icons on top. **Closest analogue to this project.** | "does not interact with the game's memory. It simply reads the screen"; "safe to use ONLINE and with EAC ENABLED" | "requires the game to be running in BORDERLESS WINDOWED mode" |
| [nightlord-detector](https://github.com/SplenectomY/nightlord-detector) | `mss` desktop capture of one rectangle at 2 Hz, Tesseract OCR, top-most click-through window | "never injects into `nightreign.exe`, never reads game memory, and never sends input… lowest-risk design against Easy Anti-Cheat… still not a FromSoftware blessing" | "borderless-windowed Nightreign (exclusive fullscreen fights the overlay)" |
| [Map Labels / Map Overlay & Seed Finder](https://www.nexusmods.com/eldenringnightreign/mods/419) (Nexus 419, 659) | Transparent window over the game plus global hotkeys | "does not inject, patch, or read game memory… designed to be online-safe and compatible with Easy Anti-Cheat" | Borderless Window |
| Nightreign Auto-Timer, NAST (Nexus 139, 196) | OCR of on-screen "DAY" text to start storm timers | No EAC statement | not stated |

The Nexus pages returned HTTP 403 to direct fetches. Their claims above are quoted from search-engine extracts of those pages.

**No report found of a ban caused by a screen-reading, external-window tool.** The Nightreign ban reports I found involve file or memory mods used online, such as Seamless Co-op connected to official servers. **[C]** [Steam thread "I got banned"](https://steamcommunity.com/app/2622380/discussions/0/597402319036942977/). Absence of evidence is weak evidence, and EAC does not publish its detections.

**Counter-examples worth noting.** OCR *bots* also exist, for example relic-farming bots that send input ([Pulgo relic bot README](https://github.com/PulgoMaster/Pulgo-Elden-Ring-Nightreign-Relic-Farming-Bot/blob/master/README.md)). Those cross into "automation software" (EAC FAQ) and "bots" (EULA §10(i)). The project's no-automated-input constraint keeps it on the right side of that line. **[I]**

## 4. EULA and ToS clauses

Source: Bandai Namco Entertainment End User License Agreement, "Last Updated: 2024-09-17", which the Steam page links as the Nightreign EULA. **[P]** [ELDEN RING NIGHTREIGN EULA on Steam](https://store.steampowered.com/eula/2622380_eula_0)

- **§10(i)** prohibits: "use or exploit any bugs, errors, or design flaws to obtain unauthorized access to the Game, to gain an unfair advantage over other players, or to cheat… including but not limited to… creating, developing, distributing, or using any bots, emulators, or other unauthorized third party tools."
- **§10(l):** "circumvent, or attempt to circumvent, any security measures in the Game."
- **§3(b):** no reverse engineering or decompiling, and no "modify, alter, or create any derivative works of the Game."
- **§23** (anti-cheat): "if you remove or disable such software or features at any time, your license to the Game terminates immediately."
- **§10, preamble:** violations may lead to "temporary or permanent suspension to your account or access to the Game."
- **Not addressed:** the EULA says nothing explicit about overlays, screen reading, OCR, recording or streaming. "Unauthorized third party tools" is never defined.

**Reading [I]:** the only clause a passive overlay could plausibly fall under is §10(i), "unauthorized third party tools". In context, that clause is about exploits, unfair advantage and cheating. There is no explicit permission for companion tools either, so legal risk is not zero but is undefined and unenforced as far as I can find. A screen-reading tool does not modify, decompile, or circumvent anything, so §3(b) and §10(l) do not apply as long as the app never touches game files, memory or EAC. §23 means the app must never ask users to disable EAC, use an "EAC toggler", or launch `nightreign.exe` directly. PCGamingWiki documents both of these as common mod workarounds. I found no FromSoftware or Bandai Namco statement on third-party companion tools.

## 5. Display modes, overlay visibility and capture

### 5.1 What Nightreign offers

According to PCGamingWiki **[C]**:
- Windowed, Borderless Windowed and Fullscreen are all available.
- 60 FPS lock, VSync always on, ultrawide pillarboxed to 16:9, D3D12.
- **HDR:** "Game needs to be set to Fullscreen before HDR can be enabled."
- "Mute on focus lost" is "Functional only when using Fullscreen mode". This suggests the game treats focus loss specially in Fullscreen. **[I]**

Elden Ring, on the same engine, shows the same HDR behaviour in player reports: "you need to be in Fullscreen otherwise it greys out." **[C]** [Steam thread, Elden Ring HDR](https://steamcommunity.com/app/1245620/discussions/0/3279194170704993566)

### 5.2 Can a separate top-most window draw over each mode?

- **Borderless windowed and windowed:** yes. Every community overlay above runs this way. The DWM (Desktop Window Manager) composes top-most windows over flip-model windows, and can keep the game on independent flip by using multiplane overlays (MPO) when other content sits on top. **[P]** [DirectX dev blog: DXGI flip model](https://devblogs.microsoft.com/directx/dxgi-flip-model/), **[C]** the tool READMEs.
- **Fullscreen:** **unreliable or unverified.** D3D12 is flip-model only (same blog). Under Fullscreen Optimizations the game "believes that it is running in Fullscreen Exclusive" while the DWM still composes it. "When an overlay such as the Game Bar is present, the DWM reassumes control of the display." **[P]** [DirectX dev blog: Demystifying Full Screen Optimizations](https://devblogs.microsoft.com/directx/demystifying-full-screen-optimizations/). So a top-most window can in principle appear. However, community tools consistently report "exclusive fullscreen fights the overlay". There are also Elden Ring reports of alt-tab problems and black screens in Fullscreen. **[C]** Treat Fullscreen as unsupported for the overlay until the detection spike tests it.

### 5.3 Capture APIs

- **Windows Graphics Capture (WGC)**
  - `IGraphicsCaptureItemInterop::CreateForWindow` captures a single HWND and needs Windows 10 1903 or later. **[P]** [CreateForWindow](https://learn.microsoft.com/en-us/windows/win32/api/windows.graphics.capture.interop/nf-windows-graphics-capture-interop-igraphicscaptureiteminterop-createforwindow)
  - Window capture records the target window's own content. Our overlay is a separate window, so it is not in the frames, which avoids a feedback loop. **[I]**, based on WGC capturing one "application window" ([Screen capture docs](https://learn.microsoft.com/en-us/windows/apps/develop/media-authoring-processing/screen-capture)); verify in the spike.
  - **Yellow border:** "a yellow notification border is drawn by the system around the actively captured item." It can be turned off with `GraphicsCaptureSession.IsBorderRequired = false` (Windows build 20348 or later), but only after user consent through `GraphicsCaptureAccess.RequestAccessAsync(Borderless)`. That call requires "the **graphicsCaptureWithoutBorder** capability in your app's package manifest". **[P]** [IsBorderRequired](https://learn.microsoft.com/en-us/uwp/api/windows.graphics.capture.graphicscapturesession.isborderrequired), [RequestAccessAsync](https://learn.microsoft.com/en-us/uwp/api/windows.graphics.capture.graphicscaptureaccess.requestaccessasync)
  - A Tauri app is unpackaged by default. Whether an unpackaged app can drop the border is not documented. One developer reports the setter works without the consent call ([handmade.network wcap thread](https://handmade.network/forums/t/8701-wcap/2)). **[C]**, open risk.
  - **HDR:** "On systems with Windows HD color enabled… To avoid pixel overclipping (i.e. the captured content looks washed out)… consider using DXGI_FORMAT_R16G16B16A16_FLOAT… HDR-to-SDR tone mapping might be required." **[P]** [Screen capture docs](https://learn.microsoft.com/en-us/windows/apps/develop/media-authoring-processing/screen-capture)
- **DXGI Desktop Duplication (DDA)**
  - Captures a whole output, meaning the monitor, *including* our overlay. There is no capture border.
  - `DuplicateOutput` always returns BGRA8, converting "the fullscreen surface to a 32-bit BGRA format". `DuplicateOutput1` can return the game's original format, such as R10G10B10A2 or FP16, for HDR.
  - It fails with `DXGI_ERROR_UNSUPPORTED` in "non-DWM desktop modes" and on desktop switches, and with `E_ACCESSDENIED` on the secure desktop. There is a limit of four concurrent duplicating processes per session.
  - **[P]** [DuplicateOutput](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/nf-dxgi1_2-idxgioutput1-duplicateoutput), [DuplicateOutput1](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_5/nf-dxgi1_5-idxgioutput5-duplicateoutput1), [Desktop Duplication API](https://learn.microsoft.com/en-us/windows/win32/direct3ddxgi/desktop-dup-api)
- **Keeping our overlay out of monitor capture**
  - `SetWindowDisplayAffinity(hwnd, WDA_EXCLUDEFROMCAPTURE)`: "The window is displayed only on a monitor. Everywhere else, the window does not appear at all." Requires Windows 10 2004 or later, and works only while the DWM is composing. **[P]** [SetWindowDisplayAffinity](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowdisplayaffinity)
  - Tauri exposes this as `contentProtected`. tao's Windows code calls exactly `SetWindowDisplayAffinity(..., WDA_EXCLUDEFROMCAPTURE)`. **[P]** [tao `window.rs`](https://github.com/tauri-apps/tao/blob/dev/src/platform_impl/windows/window.rs)
  - Side effect **[I]**: it also hides the overlay from the player's own OBS or Discord streams. Some users will want that, others won't.
- **Does EAC block capture of the game?** No primary statement either way. Nightreign is widely streamed and recorded, and the community OCR tools capture it with EAC on. **[C]** The known problem is OBS *Game Capture*, which hooks or injects into the game; window or display capture is the usual workaround. **[C]** [OBS forum: black screen game capture with EAC](https://obsproject.com/forum/threads/black-screen-game-capture-easyanticheat.113475/post-434298)

### 5.4 Overlay window mechanics in Tauri 2 (all primary: source code)

- **Click-through:** `set_ignore_cursor_events(true)` sets `WS_EX_TRANSPARENT | WS_EX_LAYERED`. [tao `window_state.rs`](https://github.com/tauri-apps/tao/blob/dev/src/platform_impl/windows/window_state.rs)
- **Never steals focus:** `focusable: false` adds `WS_EX_NOACTIVATE`. This matters because Fullscreen reacts to focus loss. [tao `window_state.rs`](https://github.com/tauri-apps/tao/blob/dev/src/platform_impl/windows/window_state.rs)
- **Window config:** Tauri's window config has `transparent`, `alwaysOnTop`, `focus`, `focusable`, `skipTaskbar` and `contentProtected`. [tauri-utils `config.rs`](https://github.com/tauri-apps/tauri/blob/dev/crates/tauri-utils/src/config.rs)
- **Global hotkeys:** Tauri's `global-hotkey` crate uses `RegisterHotKey` on Windows, not a low-level keyboard hook. [global-hotkey Windows impl](https://github.com/tauri-apps/global-hotkey/blob/dev/src/platform_impl/windows/mod.rs)
  - `RegisterHotKey` "fails if the keystrokes… have already been registered for another hot key".
  - F12 "is reserved for use by the debugger at all times".
  - `MOD_WIN` shortcuts "are reserved for use by the operating system".
  - [RegisterHotKey](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerhotkey)
  - Neither EAC nor the EULA mentions hotkeys. A hotkey that only toggles our own UI is not "automation" because it sends nothing to the game. **[I]**

### 5.5 Matrix

| Game mode | Overlay visible over game | Capture works | HDR | Status |
|---|---|---|---|---|
| Borderless windowed (SDR) | Yes [C, multiple tools] | WGC window or DDA [P/C] | Not available in-game [C] | **Supported v1 overlay mode** |
| Windowed (SDR) | Yes [I/C] | Yes; regions shift with window position [C] | No | Supported, with recalibration on move or resize |
| Fullscreen (SDR) | Unreliable or unverified [C] | Probably, through WGC or DDA [I] | — | Companion-only until the spike proves otherwise |
| Fullscreen + HDR | Unreliable or unverified [C] | Needs FP16 capture plus tone-mapping for OCR [P] | Yes, only mode with game HDR [C] | Companion-only; HDR capture is a spike item |

## Verdict

**No concrete blocker.** Nothing in EAC's published guidance, the Nightreign EULA or Windows capture APIs prohibits or technically prevents a separate-process, top-most, click-through overlay that reads the screen. A near-identical tool, Nightreign Armament Helper (OCR plus recommendation icons drawn over the game), already runs this way with EAC on, online. The settled stack (Tauri 2 overlay, Rust-side capture) needs no change.

**Constraints the spec must impose**
1. **Never touch the game process.** No `OpenProcess` or handles to `nightreign.exe`, no `ReadProcessMemory`, no DLL injection or hooks (including D3D or Present hooks and OBS-style "game capture"). No `SendInput` or synthetic input. No kernel drivers. This aligns with every risk category the EAC FAQ names.
2. **Never touch game files or EAC.** Do not read, modify or replace game files. Never suggest or automate disabling EAC, EAC togglers, or launching `nightreign.exe` directly (EULA §3(b), §10(l), §23). The app works only with EAC on, online.
3. **Overlay requires Borderless Windowed** (Windowed tolerated). First-run setup must detect the mode and guide the user. Fullscreen and HDR users get the **companion window as the fallback**, which can still receive detections if capture works in that mode.
4. **Overlay window flags:** separate top-level window that is transparent, always on top, click-through (`ignoreCursorEvents`), non-focusable (`WS_EX_NOACTIVATE`) and hidden from the taskbar. It must never take focus from the game, so any interactive UI goes in the companion window.
5. **Capture:** WGC window capture of the game HWND is the primary path, because it excludes our own overlay. DDA monitor capture is the fallback, with the overlay hidden from capture through `contentProtected` or a region choice that avoids overlap. Capture only regions of interest, in memory (already a standing constraint).
6. **Minimum OS:** Windows 10 2004 (build 19041) or later, for `WDA_EXCLUDEFROMCAPTURE`. Borderless WGC needs build 20348 or later in practice, so Windows 11.
7. **Hotkeys through `RegisterHotKey`** (Tauri global-shortcut). No low-level keyboard hooks. Avoid F12, `Win+` combinations and Nightreign's default bindings. Handle registration failure.
8. **Positioning and disclaimer:** an advisory, read-only, unofficial, not-endorsed tool. Do not imply FromSoftware or Bandai Namco permission (consistent with the no-branding constraint).

## Open risks

- **No first-party blessing.** EULA §10(i), "unauthorized third party tools", is undefined. Bandai Namco could at any time declare companion overlays prohibited, or EAC could start flagging some window or capture behaviour. EAC never discloses detections. Mitigation: the companion-only mode keeps the product useful if the overlay must be dropped.
- **WGC yellow border for unpackaged apps.** `IsBorderRequired = false` officially needs a package-manifest capability. Tauri builds are unpackaged NSIS or MSI installers. If the border cannot be removed, options are: accept the border, use DDA plus `contentProtected`, or package as MSIX (sparse package). **Spike item.**
- **Fullscreen and HDR.** HDR players must choose between game HDR and the overlay. HDR capture needs FP16 and tone-mapping before OCR, which may conflict with the "no GPU compute beyond capture" budget. **Spike item.**
- **`contentProtected` side effect.** It hides the overlay from players' own streams and recordings. It should be a user setting with a sensible default, not hard-coded.
- **Community evidence only for "EAC-safe".** Every precedent's safety claim is self-reported. There have been no reports of bans, but there is also no attestation.
- **Patch drift.** EAC configuration or the game's window-mode behaviour can change with any patch. The detection self-test in first-run setup should re-verify the mode and capture each session.

## Implications for the map

- The **detection spike** must cover: WGC window capture vs DDA on Nightreign in Borderless and in Fullscreen, overlay visibility in Fullscreen, borderless WGC from an unpackaged Tauri app, and HDR (FP16) capture plus tone-mapping cost.
- **First-run setup & calibration** must include window-mode detection and guidance toward Borderless Windowed, with a companion-only path for Fullscreen and HDR.
- **Settings & hotkeys:** use `RegisterHotKey` semantics, a reserved-key list (F12, `Win+`), conflict detection, and a "hide overlay from screen capture/streams" toggle.
- **Display support matrix:** v1 HDR detection is at best companion-only; ultrawide is pillarboxed to 16:9 in-game, which simplifies regions of interest.
- **Code signing & SmartScreen / distribution:** packaging (MSIX or sparse package vs NSIS) may be forced by the WGC border question, which links it to the spike.
- **Minimum OS:** Windows 10 2004 or later; Windows 11 recommended.
