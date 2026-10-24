# Windows screen capture and OCR callable from Tauri

Research for [#4](https://github.com/AbdulmajeedAlOmari/tarnished-tactician/issues/4). Researched 2026-10-24. Versions are pinned to what crates.io, GitHub and Hugging Face reported that day.

**Question:** what is the lightest stack, callable from a Tauri 2 (Rust) app on Windows, that can capture regions of the Nightreign window and read its UI text and icons, within the map's targets (ROI capture at about 2–4 fps, under about 5% average CPU, under about 300 MB RAM, no GPU compute beyond capture)?

## TL;DR

- **Capture (primary):** Windows Graphics Capture (WGC), per window, through [`windows-capture` 2.0.1](https://crates.io/crates/windows-capture) (MIT). Target the game's HWND directly, so the system picker isn't needed. Crop each ROI on the GPU, ask for no border, and throttle to 2–4 fps.
- **Capture (fallback):** DXGI Desktop Duplication on the game's monitor, from the same crate. It draws no border and runs on Windows 8 and later. The overlay must be hidden from capture with `WDA_EXCLUDEFROMCAPTURE`.
- **OCR (primary):** the built-in `Windows.Media.Ocr`, called through the [`windows` 0.62.2](https://crates.io/crates/windows) crate. It adds no binary size and Windows maintains it. Two caveats: Microsoft documents it as needing package identity, and it needs the en-US OCR language pack.
- **OCR (fallback):** [`ocrs` 0.13.1](https://crates.io/crates/ocrs) on [`rten` 0.27.0](https://crates.io/crates/rten). It is pure Rust and adds about 12 MB of bundled models (CC-BY-SA-4.0). It needs no OS components, runs the same on every machine, and can run fixture tests on Linux CI.
- **Rejected for v1:**
  - Tesseract works, and two Nightreign community tools ship it. It does need a C/C++ toolchain, and you have to pre-process images to dark text on a light background.
  - PaddleOCR through `ort` gives the best general accuracy, but `onnxruntime.dll` alone is 17.3 MB plus about 12 MB of models, and `ort` 2.0 is still a release candidate.
  - `xcap` uses GDI by default, and its WGC mode starts a new session for every screenshot.
- **Matching:** OCR is only a noisy key into a **closed vocabulary**. Normalise the text, limit candidates by ROI context (weapon name, passive, relic effect…), then score with [`strsim` 0.11.1](https://crates.io/crates/strsim) (normalised Levenshtein plus Jaro-Winkler). Accept a match only above a score threshold *and* a margin over the runner-up, and confirm it across two consecutive frames. Return the stable ID, never the raw string.
- **Icons:** read text first. Where an element has no text, use cheap colour or hash features, or [`imageproc` 0.27.0](https://crates.io/crates/imageproc) template matching on fixed slots. Templates are game art, which raises a licensing question (see Implications).
- **Not decidable from documentation:** how accurate each engine is on Nightreign's stylised serif UI font. No public benchmark exists. The detection spike should run a small bake-off of Windows OCR against ocrs on a fixture corpus; it decides which engine is primary.

---

## 1. Capture

### 1.1 Windows Graphics Capture (WGC)

- **Item creation without a picker.** Desktop (Win32) apps can create a `GraphicsCaptureItem` for a specific HWND or HMONITOR through `IGraphicsCaptureItemInterop::CreateForWindow` / `CreateForMonitor`. This needs Windows 10 1903 (build 18362) or later ([MS Learn: IGraphicsCaptureItemInterop](https://learn.microsoft.com/en-us/windows/win32/api/windows.graphics.capture.interop/nn-windows-graphics-capture-interop-igraphicscaptureiteminterop)). The UWP picker flow isn't required.
- **Frames arrive as D3D11 textures.** The frame pool hands out `Direct3D11CaptureFrame` surfaces. Microsoft advises keeping heavy work off the `FrameArrived` thread and copying out a sub-rect using `ContentSize` ([MS Learn: Screen capture](https://learn.microsoft.com/en-us/windows/uwp/audio-video-camera/screen-capture)). This is what makes cropping on the GPU cheap.
- **Per-window capture.** A window item captures only that window's own content. Our overlay is a separate window, so it doesn't appear in the frames. Per-window capture also keeps working when the game window moves between monitors.
- **Yellow border.**
  - The system draws a yellow border around every captured item ([MS Learn: Screen capture](https://learn.microsoft.com/en-us/windows/uwp/audio-video-camera/screen-capture)). With a fullscreen game, that border sits on the edges of the player's screen, which is unacceptable for an overlay product.
  - `GraphicsCaptureSession.IsBorderRequired` (build 20348 / UniversalApiContract v12 and later, i.e. Windows 11) can turn it off. Microsoft documents that the system honours `false` only after the user consents through `GraphicsCaptureAccess.RequestAccessAsync(Borderless)`, and that this needs the `graphicsCaptureWithoutBorder` capability in a package manifest ([MS Learn: IsBorderRequired](https://learn.microsoft.com/en-us/uwp/api/windows.graphics.capture.graphicscapturesession.isborderrequired)).
  - In practice, unpackaged callers do set it. `xcap` calls `SetIsBorderRequired(false)` and treats a failure as non-fatal ([xcap `src/windows/wgc.rs`](https://github.com/nashaofu/xcap/blob/master/src/windows/wgc.rs)). A `windows-capture` user switched to `draw_border=False` to stop the border flashing ([windows-capture #158](https://github.com/NiiightmareXD/windows-capture/issues/158)).
  - **Unverified:** whether an unpackaged Tauri app gets a borderless capture on a stock Windows 11 machine. The detection spike must check this first. If it fails, the fallback is Desktop Duplication, which has no border.
- **Frame-rate control.** `GraphicsCaptureSession.MinUpdateInterval` exists only from build 26100 (Windows 11 24H2) ([MS Learn: MinUpdateInterval](https://learn.microsoft.com/en-us/uwp/api/windows.graphics.capture.graphicscapturesession.minupdateinterval)). On older builds, drop frames in the handler: an unmapped frame costs almost nothing on the CPU. WGC delivers a frame only when content changes. `windows-capture` exposes `MinimumUpdateIntervalSettings::Custom(Duration)`, plus `DirtyRegionSettings` ([docs.rs settings](https://docs.rs/windows-capture/latest/windows_capture/settings/index.html)).
- **HDR.**
  - With Windows HD Color on, captured content may not be BGRA8. Microsoft recommends `R16G16B16A16_FLOAT` throughout the pipeline, plus HDR-to-SDR tone mapping, to avoid washed-out pixels ([MS Learn: Screen capture](https://learn.microsoft.com/en-us/windows/uwp/audio-video-camera/screen-capture)).
  - `windows-capture` offers `ColorFormat::{Rgba16F, Rgba8, Bgra8}` ([docs.rs ColorFormat](https://docs.rs/windows-capture/latest/windows_capture/settings/enum.ColorFormat.html)).
  - Tone-mapping a few small ROIs on the CPU is cheap, but OCR still needs calibrating against it. This report found no primary source on whether Nightreign offers an HDR output mode, so treat HDR as a question for the display support matrix.
- **Minimum OS.** `GraphicsCaptureSession` dates from Windows 10 1803 ([MS Learn](https://learn.microsoft.com/en-us/uwp/api/windows.graphics.capture.graphicscapturesession)). Windows 10 Home/Pro reached end of support on 2025-10-14 ([MS Lifecycle](https://learn.microsoft.com/en-us/lifecycle/products/windows-10-home-and-pro)), so a **Windows 11 baseline** is reasonable. Windows 10 users would get the bordered WGC path or the DXGI fallback.

### 1.2 DXGI Desktop Duplication

- **Per-output only.** It duplicates a whole monitor, never a single window ([MS Learn: Desktop Duplication API](https://learn.microsoft.com/en-us/windows/win32/direct3ddxgi/desktop-dup-api)). Anything on top of the game, our overlay included, ends up in the frame. The overlay must set `SetWindowDisplayAffinity(WDA_EXCLUDEFROMCAPTURE)`, which is supported from Windows 10 2004 ([MS Learn](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowdisplayaffinity)).
- **No border, Windows 8 or later** ([MS Learn: AcquireNextFrame](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/nf-dxgi1_2-idxgioutputduplication-acquirenextframe)).
- **Fragility.** `AcquireNextFrame` returns `DXGI_ERROR_ACCESS_LOST` on a desktop switch, a mode change, or a switch to or from a fullscreen application. The app must then recreate the duplication ([MS Learn: AcquireNextFrame](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/nf-dxgi1_2-idxgioutputduplication-acquirenextframe)). The default limit is four concurrent duplication clients, so OBS and similar tools compete for slots ([MS Learn: DuplicateOutput1](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_5/nf-dxgi1_5-idxgioutput5-duplicateoutput1)).
- **HDR.** `DuplicateOutput` always converts to BGRA8. `DuplicateOutput1` can return the original fullscreen format, for example R10G10B10A2 or FP16 ([MS Learn: DuplicateOutput1](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_5/nf-dxgi1_5-idxgioutput5-duplicateoutput1)).
- **Throttling.** Poll `AcquireNextFrame` with a timeout. The app controls the rate.

### 1.3 Rust crates

| Crate (version, licence) | Backends | Window capture | ROI crop | FPS control | Notes |
|---|---|---|---|---|---|
| [`windows-capture` 2.0.1](https://crates.io/crates/windows-capture) (MIT) | WGC plus DXGI Desktop Duplication ([README](https://github.com/NiiightmareXD/windows-capture)) | Yes. `Window::from_raw_hwnd`, `from_contains_name`, `enumerate` ([docs.rs Window](https://docs.rs/windows-capture/latest/windows_capture/window/struct.Window.html)) | `Frame::buffer_crop(x0,y0,x1,y1)` runs `CopySubresourceRegion` into a staging texture of ROI size, so the crop happens on the GPU ([source `src/frame.rs`](https://github.com/NiiightmareXD/windows-capture/blob/main/src/frame.rs)) | `MinimumUpdateIntervalSettings`, `DirtyRegionSettings`, `DrawBorderSettings::WithoutBorder` ([source `src/settings.rs`](https://github.com/NiiightmareXD/windows-capture/blob/main/src/settings.rs)) | Single maintainer, 521 stars, last push 2026-10-05 (GitHub API). Allocates a new staging texture on every `buffer_crop` call, which is fine at 2–4 fps; pool them if it shows in profiles. |
| [`xcap` 0.9.8](https://crates.io/crates/xcap) (Apache-2.0) | GDI `BitBlt`/`PrintWindow` by default; WGC behind the `wgc` feature ([source `src/windows/capture.rs`](https://github.com/nashaofu/xcap/blob/master/src/windows/capture.rs)) | Yes | `capture_region` on monitors | None for screenshots | The WGC path creates and closes a session for every screenshot ([source `src/windows/wgc.rs`](https://github.com/nashaofu/xcap/blob/master/src/windows/wgc.rs)). That is per-shot setup cost, and the border can flash each time. Good for one-off calibration screenshots, not for a 2–4 fps loop. |
| [`windows` 0.62.2](https://crates.io/crates/windows) (MIT/Apache-2.0) | Raw WinRT/Win32 bindings | Yourself | Yourself | Yourself | Most control and fewest dependencies, but you write about 300–500 lines of D3D11/WinRT plumbing (estimate). `windows-capture` already depends on it, so use it directly for OCR and `SetWindowDisplayAffinity`. |

**Capture recommendation:** use `windows-capture` with WGC per window as primary and its `DxgiDuplicationApi` as fallback. Sketch:

1. Find the game window by process name (to be confirmed in the spike) and build the item with `from_raw_hwnd`.
2. Use `ColorFormat::Bgra8` for SDR, or `Rgba16F` when the display is HDR.
3. Set `DrawBorderSettings::WithoutBorder` and `CursorCaptureSettings::WithoutCursor`.
4. Throttle to 4 fps with `MinimumUpdateIntervalSettings::Custom(250 ms)` on 24H2, or drop frames in the handler.
5. In `on_frame_arrived`, call `buffer_crop` once per ROI. Only those few KB cross to the CPU.
6. Run a cheap per-ROI change check (hash or mean-diff) so OCR runs only when an ROI changes.

## 2. Recognition (OCR and icons)

### 2.1 Candidates

**`Windows.Media.Ocr` (built-in)**
- **API:** `OcrEngine.RecognizeAsync(SoftwareBitmap)` returns lines, then words, then `BoundingRect` and `Text` ([MS Learn: OcrEngine](https://learn.microsoft.com/en-us/uwp/api/windows.media.ocr.ocrengine)). `OcrWord` has **no confidence score** ([MS Learn: OcrWord](https://learn.microsoft.com/en-us/uwp/api/windows.media.ocr.ocrword)). There is no character whitelist. `MaxImageDimension` caps the input size.
- **Rust:** the official `windows-rs` sample calls it from a plain console binary ([windows-rs `crates/samples/windows/ocr`](https://github.com/microsoft/windows-rs/tree/master/crates/samples/windows/ocr)).
- **Package identity:** the namespace docs state the APIs "are only supported for desktop apps with *package identity*" ([MS Learn: Windows.Media.Ocr](https://learn.microsoft.com/en-us/uwp/api/windows.media.ocr)). The unpackaged sample above shows it runs without identity, but Microsoft doesn't support that use. Tauri builds MSI and NSIS installers, not MSIX ([Tauri: Windows installer](https://v2.tauri.app/distribute/windows-installer/)). Identity is possible through packaging with external location, but that needs a **signed identity package from a trusted certificate** registered by the installer ([MS Learn: grant identity](https://learn.microsoft.com/en-us/windows/apps/desktop/modernize/grant-identity-to-nonpackaged-apps)).
- **Language pack:** it recognises only languages whose OCR pack is installed. A user can add en-US with `Get-WindowsCapability -Online | ? Name -like 'Language.OCR*en-US*' | Add-WindowsCapability -Online` (admin, may take minutes) ([MS Learn: PowerToys Text Extractor](https://learn.microsoft.com/en-us/windows/powertoys/text-extractor)). A user with a non-English Windows display language playing the English client may not have it.
- **Windows AI `TextRecognizer`:** the newer Windows App SDK API runs only on devices with an NPU and needs package identity ([MS Learn: text recognition](https://learn.microsoft.com/en-us/windows/ai/apis/text-recognition); [modernize packaged apps](https://learn.microsoft.com/en-us/windows/apps/desktop/modernize/modernize-packaged-apps)). That hardware base is too narrow for v1.

**`ocrs` 0.13.1 + `rten` 0.27.0 (pure Rust)**
- **Models:** detection is a segmentation model and recognition is a CRNN ([ocrs-models](https://github.com/robertknight/ocrs-models)). Latin alphabet only, which is fine for an English client. The README calls it an "early preview" and says to "expect more errors than commercial OCR engines" ([README](https://github.com/robertknight/ocrs)).
- **Model files:** text-detection 2.5 MB and text-recognition 9.7 MB, as `.rten` or `.onnx`. The Hugging Face card licenses the weights **CC-BY-SA-4.0**. They are trained on HierText (CC-BY-SA-4.0) ([HF `robertknight/ocrs`](https://huggingface.co/robertknight/ocrs); [ocrs-models](https://github.com/robertknight/ocrs-models)).
- **Options:** `OcrEngineParams` has `allowed_chars`, and `detection_model` is an `Option` ([docs.rs](https://docs.rs/ocrs/latest/ocrs/struct.OcrEngineParams.html)). `recognize_text` takes caller-supplied line rectangles ([docs.rs OcrEngine](https://docs.rs/ocrs/latest/ocrs/struct.OcrEngine.html)). Because our ROIs are fixed, detection can likely be skipped and only the recognition model loaded. This is inferred from the signatures and still to be confirmed in the spike.

**Tesseract 5.5.3 (C++), via [`tesseract-rs` 0.4.0](https://crates.io/crates/tesseract-rs) (MIT) or [`tesseract` 0.15.2](https://crates.io/crates/tesseract)**
- **Build:** `tesseract-rs` downloads and compiles Tesseract and Leptonica with CMake and a C++ compiler. It can embed traineddata ([README](https://github.com/cafercangundogdu/tesseract-rs)).
- **Data:** `eng.traineddata` is 4.1 MB (tessdata_fast) or 15.4 MB (tessdata_best), Apache-2.0 (GitHub API on [tessdata_fast](https://github.com/tesseract-ocr/tessdata_fast) / [tessdata_best](https://github.com/tesseract-ocr/tessdata_best)).
- **Input requirements:** Tesseract 4 and later wants dark text on a light background. Internal binarisation is weak on uneven backgrounds. It has single-line and single-word modes (PSM 7/8) and character whitelists ([tessdoc ImproveQuality](https://tesseract-ocr.github.io/tessdoc/ImproveQuality.html)). Nightreign's light-on-dark UI would need inversion and thresholding.
- **Field evidence:** two Nightreign community overlays use Tesseract on English text with closed-vocabulary matching:
  - [NightreignArmamentHelper](https://github.com/nextfinish/NightreignArmamentHelper) uses Jaccard similarity and caches "reference points" to skip repeat OCR. It reports correct detection in its testing, with occasional half-second latency.
  - [nightlord-detector](https://github.com/SplenectomY/nightlord-detector) says English nameplates are "the most reliable OCR target".

**PaddleOCR PP-OCRv5 via [`ort` 2.0.0-rc.15](https://crates.io/crates/ort) or [`oar-ocr` 0.10.1](https://crates.io/crates/oar-ocr)**
- **Models:** the en mobile rec model is 7.8 MB and the mobile det model is 4.7 MB (Paddle format, Apache-2.0) ([HF en_PP-OCRv5_mobile_rec](https://huggingface.co/PaddlePaddle/en_PP-OCRv5_mobile_rec); [HF PP-OCRv5_mobile_det](https://huggingface.co/PaddlePaddle/PP-OCRv5_mobile_det)). They need ONNX conversion; `oar-ocr` handles that ([README](https://github.com/greatv/oar-ocr)).
- **Runtime size:** ONNX Runtime 1.31.0's `onnxruntime.dll` is **17.3 MB** uncompressed, measured from the [official win-x64 zip](https://github.com/microsoft/onnxruntime/releases/tag/v1.31.0). `ort` recommends `load-dynamic` linking ([ort linking guide](https://ort.pyke.io/setup/linking)).

**Template / icon matching**
- `imageproc` 0.27.0 provides `match_template` (with mask and parallel variants) ([docs.rs](https://docs.rs/imageproc/latest/imageproc/template_matching/index.html)).
- Small fixed slots matched against a short candidate list cost little on the CPU (estimate).
- Brittle against UI scale and resolution changes unless templates are rescaled at calibration.

### 2.2 Comparison

Accuracy is rated **qualitatively**: no public benchmark of any of these engines on Nightreign's font was found. Sizes are measured from the cited files. CPU cost is measured only where it says so; everything else is an estimate.

| | Windows.Media.Ocr | ocrs + rten | Tesseract 5.5 | PaddleOCR v5 via ort | Template matching (imageproc) |
|---|---|---|---|---|---|
| **Stylised-font accuracy** | Unknown on Nightreign. A general-purpose engine with no whitelist or confidence. Must be bake-off tested. | Unknown. Self-described early preview. Correctly read a synthetic serif light-on-dark ROI ("Uchigatana +2" / "Improved Attack Power at Full HP") in my local test (§2.3). | Proven adequate on English Nightreign names by two community tools (above), with inversion and binarisation. | Generally the strongest open model family. Not tested on Nightreign. | Exact for fixed icons at a known scale. Breaks with UI scale or HDR changes unless recalibrated. |
| **Binary / install size** | **0 MB.** Ships with Windows; the en-US OCR pack may need installing. | About 12.2 MB of models (2.5 + 9.7), or about 9.7 MB if detection is skipped, plus rten code (a few MB, estimate). | 4.1 MB eng_fast traineddata plus the libtesseract/leptonica static libs (several MB, estimate). | 17.3 MB DLL plus about 12.5 MB of models, so **about 30 MB**. | Templates only (KBs), plus `image`/`imageproc` code. |
| **CPU cost per ROI** | Estimate: tens of ms per small ROI, runs in-process on OS code. Measure in the spike. | **Measured (Linux, i9-14900K, CLI cold start, 640×72 ROI, 2 lines):** about 130–145 ms wall and about 0.6 s CPU per process, *including* model load. The detection model graph alone (110 ops) took about 80 ms. Warm, in-process, recognition-only should be well under that (estimate). | Estimate: tens of ms per line ROI with PSM 7. | Estimate: comparable to ocrs or faster (heavily optimised runtime), but a larger resident footprint. | Sub-ms to a few ms per slot (estimate). |
| **RAM** | Small, system-managed (estimate). | Measured CLI peak RSS 162 MB with both ONNX models (upper bound; the library with `.rten` and recognition only should be far lower, estimate). | Tens of MB (estimate). | Tens to 100+ MB (estimate). | Negligible. |
| **Licence** | Windows component, no redistribution. | Code MIT/Apache-2.0. **Weights CC-BY-SA-4.0** (attribution + share-alike on the model files). | Apache-2.0 (code + tessdata). | Apache-2.0 (models, PaddleOCR); ORT MIT; `ort` MIT/Apache-2.0. | MIT. Templates are game art, so licensing needs checking. |
| **Maintenance** | Microsoft, but [described as "legacy"](https://learn.microsoft.com/en-us/windows/ai/apis/text-recognition) next to the NPU API. Officially needs package identity. | Active: ocrs pushed 2026-10-22, 1.9k stars. Single maintainer. | Active upstream (5.5.3, 2026-07-24). Rust bindings are small projects. | PaddleOCR very active (91k stars). `ort` is 2.0 **RC**. | imageproc active (0.27.0, 2026-06). |
| **Build complexity** | `windows` crate features only. | Pure Rust. | C++/CMake toolchain in CI. | Ship a DLL and convert models. | Pure Rust. |

Sources for the stars and push dates: GitHub REST API on 2026-10-24. Crate versions: crates.io API on 2026-10-24.

### 2.3 Local measurement (ocrs)

I ran `ocrs-cli` 0.13.1 on a synthetic 640×72 ROI: FreeSerif, cream text on a dark brown gradient, roughly imitating Nightreign's look. The machine was Linux (WSL2) on an i9-14900K. Over 5 runs it was 128–213 ms wall, about 0.6 s CPU per run and 162 MB peak RSS. The output was exactly right. Each run is a fresh process that loads both ONNX models, so these numbers are an **upper bound** for an in-process engine. Synthetic text isn't the game's font, so this is evidence of *feasibility*, not accuracy. I couldn't measure `Windows.Media.Ocr` (no Windows host available).

### 2.4 Budget estimate against the map targets (estimates)

The arithmetic assumes these fixed parameters:

- 4 fps capture, about 3 ROIs of about 600×80 px each, so roughly 0.6 MB per tick copied GPU→CPU.
- OCR runs only when an ROI changes. Prompts appear a few times per minute, in bursts.
- A burst is about 2 s of OCR at 4 fps on 3 ROIs, so about 24 OCR calls. At about 30–50 ms each that is about 1 s of CPU per burst.

Results:

- **Average CPU:** with about 6 bursts per minute, roughly 6 s of CPU per 60 s on one core. On an 8-core CPU that is under 2% average. Idle cost (frame handling plus change detection) should be well under 1% (estimate).
- **RAM:** Windows OCR adds little. ocrs recognition-only should add tens of MB. Either way it fits under 300 MB alongside Tauri and WebView2 (estimate). Validate in the spike.
- **CPU throttling:** cap OCR to one worker thread so a burst never takes more than one core.

## 3. Matching OCR text to known names

The Recommendation engine needs **stable IDs**, so the OCR output never leaves the detector as raw text. The matching procedure:

1. **Constrain candidates by context.** Each ROI type (for example the reward prompt's title line or a relic effect line) maps to one data-pack category. That keeps the candidate list to hundreds of names, not thousands.
2. **Normalise both sides:**
   - NFKC, casefold, collapse whitespace, and strip punctuation the font renders ambiguously.
   - Fold common OCR confusions (`0↔O`, `1↔l↔I`, `rn↔m`, `vv↔w`).
   - Split off structured suffixes such as `+2` or numbers and parse them separately.
   - Precompute normalised forms in the data pack at build time.
3. **Score** with `strsim::normalized_levenshtein`, plus `jaro_winkler` as a tiebreak for prefix-heavy names ([docs.rs strsim 0.11.1](https://docs.rs/strsim/latest/strsim/)). A few hundred comparisons of short strings per OCR call take microseconds. A second, token-set score (Jaccard over words, as [NightreignArmamentHelper](https://github.com/nextfinish/NightreignArmamentHelper) uses) handles partially occluded lines.
4. **Accept only when confident:**
   - Best score ≥ T, with T around 0.85, to be tuned on fixtures.
   - Best minus second-best ≥ Δ, with Δ around 0.08. This catches near-duplicates such as "+1" and "+2" variants or names that differ by one word.
   - The same ID wins on 2 consecutive frames.
   - Otherwise report "unrecognised". The overlay then shows nothing, or "?", and the one-key correction flow applies.
5. **Cache.** Key the ROI pixels by perceptual hash and map the hash to an ID, so a repeated prompt skips OCR. NightreignArmamentHelper does the same with its learned "reference points".
6. **Where it runs.** Do it in Rust, inside the detection pipeline: the data-pack name lists go in, and `{roi, id, score}` events go out to the TypeScript side. This keeps the IPC small and the Recommendation logic pure TypeScript.

`strsim` is MIT and tiny (14 KB crate). It was last released in 2024-04 but is stable and is used by clap and others (1.2B downloads). [`rapidfuzz` 0.5.0](https://crates.io/crates/rapidfuzz) is faster but has been unreleased since 2023-12, so it isn't worth it at this candidate-set size.

## 4. Recommendation

| Layer | Primary | Fallback | Why |
|---|---|---|---|
| Capture | WGC per window via `windows-capture` 2.0.1, GPU ROI crop, no border, 2–4 fps throttle | DXGI Desktop Duplication (same crate) on the game's monitor, overlay hidden with `WDA_EXCLUDEFROMCAPTURE` | WGC is per window and needs no polling; DXGI has no border and survives where WGC's border can't be removed. |
| Text | `Windows.Media.Ocr` via `windows` 0.62 | `ocrs` 0.13 (recognition-only on fixed ROIs), bundled | WinOCR costs 0 MB. ocrs covers a missing en-US pack and a future identity enforcement, and gives deterministic CI fixtures. |
| Icons / non-text | Colour or hash features, `imageproc` templates on fixed slots | Ask the user (one-key correction) | Text-first; icons only where no text exists. |
| Matching | Context-restricted candidates, normalisation, `strsim` score with threshold, margin and 2-frame vote, then stable ID | "Unrecognised" goes to the correction flow | OCR is a noisy key into a closed vocabulary. |

Put both OCR engines behind one Rust trait (`fn recognize(&self, roi: &RoiImage) -> Vec<Line>`), so the fixture harness can score both. **Flip rule:** if the spike's bake-off shows ocrs at least matching WinOCR accuracy on the Nightreign fixtures at an acceptable warm latency, ship ocrs as the only engine. The roughly 10–12 MB of model weight buys determinism, no OS-component dependency, no package-identity question and testing on Linux CI.

## 5. Open questions for the detection spike

1. Does `IsBorderRequired = false` remove the border for an unpackaged Tauri app on a stock Windows 11 23H2/24H2 machine? If not, DXGI becomes primary.
2. Does WGC window capture of Nightreign work in both its Fullscreen and Borderless Windowed modes? What is the game window's class or title and process name? The game reportedly requires DX12 ([secondary source: optimisation guide](https://www.sportskeeda.com/gaming-tech/best-pc-optimization-guide-elden-ring-nightreign); a primary source is still needed).
3. Accuracy of WinOCR vs ocrs (and optionally Tesseract) on about 100 to 200 real cropped ROIs at 1080p, 1440p and 4K. Report exact-match rate after fuzzy matching, the false-accept rate, and warm latency.
4. Is `Windows.Media.Ocr` usable when the user's Windows display language isn't English (en-US pack missing)? Can we detect that and prompt?
5. Does Nightreign have HDR output? If so, check the Rgba16F capture and tone-map path, or document "HDR off" as a v1 requirement.
6. Does ocrs run recognition-only on caller-supplied line rects without the detection model?

## Sources

Primary docs: the Microsoft Learn pages linked inline (Windows.Graphics.Capture, IGraphicsCaptureItemInterop, Desktop Duplication, DuplicateOutput1, AcquireNextFrame, SetWindowDisplayAffinity, Windows.Media.Ocr, OcrEngine, OcrWord, PowerToys Text Extractor, Windows AI text recognition, package identity, Windows 10 lifecycle). Also Tauri v2 docs, tessdoc and ort.pyke.io.

Source code read: `windows-capture` (`src/frame.rs`, `src/settings.rs`), `xcap` (`src/windows/capture.rs`, `gdi.rs`, `wgc.rs`) and `windows-rs` (`crates/samples/windows/ocr`).

Registry and API data: crates.io API (versions, licences), GitHub REST API (stars, push dates, tessdata sizes, ONNX Runtime release assets) and the Hugging Face API (model file sizes, licences).

Local measurement: §2.3.
