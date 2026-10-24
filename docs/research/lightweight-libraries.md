# Lightweight libraries around the settled stack

Research for ticket [#8](https://github.com/AbdulmajeedAlOmari/tarnished-tactician/issues/8). Researched 2026-10-24. Versions are the npm `latest` / crates.io `max_stable` as of that date.

**Question.** Which lightweight libraries should v1 use for the cross-cutting concerns around the settled stack (Tauri 2 + React + TypeScript + SQLite + Zod)? Compare bundle/binary size, maintenance and Tauri 2 compatibility, and recommend one choice per concern.

## Summary

| Concern | Recommendation (pinned) | Size impact (measured, min+gzip) | One-line rationale |
|---|---|---|---|
| SQLite access + migrations | Custom Tauri commands over **`rusqlite` 0.40.2** (`bundled`) + **`rusqlite_migration` 2.6.0**, raw SQL in Rust. No TS query builder. | TS: about 0.1 kB (`invoke` only). Rust: +1.10 MB vs +1.36 MB for the sqlx stack behind `tauri-plugin-sql` | `tauri-plugin-sql` can't run a multi-statement transaction from JS, and Drizzle and Kysely on top of it inherit that. |
| Frontend state | **`zustand` 5.0.16** | 0.47 kB | Smallest real store. The vanilla store can be fed from Tauri `listen()` outside React. |
| Routing | Overlay: **none** (its own HTML entry). Companion: **`wouter` 3.13.0** with `useHashLocation` | 3.1 kB (vs React Router 13.2 kB, TanStack 28.4 kB) | Each Tauri window loads its own URL, and the companion has only a handful of screens. |
| Styling / UI primitives | **Tailwind CSS 4.3.3** (`@tailwindcss/vite`) + **native platform primitives** (`<dialog>`, `popover`, CSS anchor positioning). Per-component `@radix-ui/react-*` only as an escape hatch. | 0 kB runtime JS (Radix dialog+popover would add 23.5 kB) | Build-time CSS only. WebView2 is evergreen Chromium, so native primitives cover most needs. |
| Share-code compression | **Native `CompressionStream('deflate-raw')`** (fallback: `fflate` 0.8.3) | 0.27 kB of glue (fflate: 4.6 kB) | The platform does it for free, and output matches fflate's to within a few characters. |
| Share-code alphabet | **base64url without padding** (RFC 4648 §5) via native `Uint8Array.prototype.toBase64` with a `btoa` fallback. Not base85. | 0 kB (`@scure/base`: 1.4 kB) | Base85 saves only about 6%, and its alphabet breaks URLs and Discord markdown. |
| Fuzzy matching (TS side) | **`@leeoniya/ufuzzy` 1.0.19** for companion search only. OCR-to-name matching stays with #4. | 4.0 kB (Fuse.js 9.3 kB) | Small and maintained, with highlight ranges. Built for list filtering and autocomplete. |
| neverthrow | **Don't adopt.** Use a local `Result` discriminated union where needed. | 0 kB (neverthrow: 2.0 kB) | Tauri already rejects with typed `{kind, message}` errors and Zod's `safeParse` already returns a result. No release since Feb 2025. |
| Zod flavour | **`zod` 4.6.7, classic API** (`import * as z from "zod"`). Not `zod/mini`. | 24.0 kB for a 5-field object schema (mini: 6.1 kB) | The saving is irrelevant for assets loaded from local disk, and mini has worse DX and no default English error messages. |

Context for the numbers: React 19.3.1 + `react-dom/client` alone is **67.8 kB** min+gzip with the same method. The whole recommended set adds roughly **32 kB** on top (Zod 24 + wouter 3.1 + uFuzzy 4.0 + Zustand 0.5 + glue), and only the companion carries most of it.

## Method

- **Measured bundle sizes**: one entry file per library exercising a realistic minimal API surface (listed in [Appendix](#appendix-benchmark-entries)). Each was bundled with **Rolldown 1.2.15** (the bundler in Vite 8.3.5) and cross-checked with **esbuild 0.28.2**: minified ESM, browser platform, React externalised (except the React baseline), `NODE_ENV=production`, then `gzip -9`. The table quotes Rolldown. esbuild agreed within about 10%. These are tree-shaken numbers, so they're more realistic than whole-package figures.
- **Whole-package sizes**: from the bundlephobia API (`https://bundlephobia.com/api/size?package=<name>`, fetched 2026-10-24). This is the size of the full entry point with no tree shaking, an upper bound. Per-export sizes come from `https://bundlephobia.com/api/exports-sizes?package=<name>`.
- **Rust binary deltas**: three minimal Linux x86_64 binaries (rustc 1.95.0, tokio + serde_json baseline) built with the release profile [Tauri recommends](https://v2.tauri.app/concept/size/) (`codegen-units=1`, `lto=true`, `opt-level="s"`, `panic="abort"`, `strip=true`). Windows deltas will differ somewhat, but the ordering should hold.
- **Versions, release dates and dependency lists**: from the npm registry (`https://registry.npmjs.org/<name>`) and the crates.io API (`https://crates.io/api/v1/crates/<name>`). Repo activity (`pushed_at`) comes from the GitHub REST API.
- **Browser support**: from [MDN browser-compat-data](https://github.com/mdn/browser-compat-data) (raw JSON, `main` branch). Tauri on Windows uses WebView2, which "is based on Microsoft Edge and therefore Chromium" and "can update itself, you are guaranteed a relatively recent chromium build" ([Tauri webview versions](https://v2.tauri.app/reference/webview-versions/)).
- Tauri compresses embedded frontend assets by default (the `compression` feature, "enabled by default: Enables asset compression": [tauri `lib.rs` @ v2.12.5](https://github.com/tauri-apps/tauri/blob/tauri-v2.12.5/crates/tauri/src/lib.rs), [`Cargo.toml` default features](https://github.com/tauri-apps/tauri/blob/tauri-v2.12.5/crates/tauri/Cargo.toml)). Frontend kilobytes therefore cost installer size and parse time, not network time. That is why the size thresholds below are relaxed compared with a website.

## 1. SQLite access and migrations

### Options

**A. `tauri-plugin-sql` 2.6.0** (crate and `@tauri-apps/plugin-sql` 2.6.0, both released 2026-10-14; actively maintained in [plugins-workspace](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/sql), last commit 2026-10-15).
- Built on sqlx: "The plugin uses sqlx… enabled by a Cargo feature" ([Tauri SQL docs](https://v2.tauri.app/plugin/sql/)). Its [`Cargo.toml`](https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/sql/Cargo.toml) pins `sqlx = "0.8"` with `json, time, uuid, rust_decimal`, plus `sqlite = ["sqlx/sqlite", "sqlx/runtime-tokio"]`. It is still on sqlx 0.8 although sqlx 0.9.0 shipped 2026-05-21 (crates.io).
- Migrations: Rust-side `Migration { version, description, sql, kind }` registered with `add_migrations`, "executed within a transaction" and applied on `preload` or `Database.load()` ([docs](https://v2.tauri.app/plugin/sql/)).
- JS API: `execute(query, bindValues)` and `select<T>(query, bindValues)`. `select` returns "each row being an object keyed by column name" (`@tauri-apps/plugin-sql` 2.6.0 `dist-js/index.d.ts`).
- **Transactions don't work across calls from JS.** [plugins-workspace#886 "[sql] Add support for transactions"](https://github.com/tauri-apps/plugins-workspace/issues/886) has been open since 2024-01-10 (last updated 2026-10-01). Issuing `BEGIN` / `ROLLBACK` as separate `execute` calls doesn't roll back. A maintainer closed [#1817](https://github.com/tauri-apps/plugins-workspace/issues/1817) as a duplicate: "transactions were never really looked at explicitly". The cause is visible in the source: the plugin opens a `sqlx::Pool` ([`wrapper.rs`](https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/sql/src/wrapper.rs), `Pool::connect(db_url)`), and sqlx's default pool has `max_connections: 10` ([sqlx 0.8.6 `pool/options.rs`](https://github.com/launchbadge/sqlx/blob/v0.8.6/sqlx-core/src/pool/options.rs#L151)). Successive calls can therefore land on different connections.
- Security surface: writing from JS requires granting `sql:allow-execute` ([docs](https://v2.tauri.app/plugin/sql/)), which lets the webview run arbitrary SQL. That's a concern when the app renders imported Builds and Share codes, which are untrusted text.

**B. Query builder in TS on top of A.**
- **Drizzle** (`drizzle-orm` 0.45.4, 2026-10-08; a `1.0.0-rc.5` is on the `rc` dist-tag, per the npm registry). The `drizzle-orm/sqlite-proxy` driver takes a callback `(sql, params, method: 'run'|'all'|'values'|'get') => Promise<{rows}>` (`sqlite-proxy/driver.d.ts`), so it can sit on `plugin-sql`. Its `transaction()` sends `begin`, the body and `commit` as **separate** callback calls (`sqlite-proxy/session.js`), so it inherits #886. Its proxy migrator calls `readMigrationFiles`, which imports `node:fs` (`drizzle-orm/migrator.js`), and that doesn't exist in a webview. Migrations would have to go through the Rust side anyway. Measured **17.0 kB** for one table + insert + select (bundlephobia whole package: 8.2 kB, which only covers the root entry and excludes `sqlite-core` and the driver). Drizzle has 2,049 open issues (GitHub).
- **Kysely** (`kysely` 0.29.7, 2026-10-17). Needs a dialect. The community [`kysely-dialect-tauri`](https://www.npmjs.com/package/kysely-dialect-tauri) 2.0.1 maps onto `db.select` / `db.execute` of `plugin-sql` (from its `dist/index.cjs`), so it has the same transaction gap. Measured **29.5 kB** (bundlephobia whole package: 39.2 kB).

**C. Custom Tauri commands over a Rust SQLite crate.**
- **`rusqlite` 0.40.2** (2026-08-08; 0.41.0 landed 2026-10-24, the day of this research). Very active (repo pushed 2026-10-24, crates.io). With `features = ["bundled"]` it compiles SQLite in (via `libsqlite3-sys` 0.38.x). Real transactions come from `Connection::transaction()` on a single owned connection.
- **`rusqlite_migration` 2.6.0** (2026-05-28; repo pushed 2026-10-22) depends on `rusqlite ^0.40.0` (crates.io dependency API). That is why the pin is 0.40.2, not 0.41.0.
- sqlx directly is also possible (0.9.0), but it brings async pooling that a single-user desktop database doesn't need.
- Errors cross the bridge as a serialisable error type. Tauri documents "If the command returns an error, the promise will reject", with a tagged `{ kind, message }` enum pattern ([Calling Rust](https://v2.tauri.app/develop/calling-rust/)).

### Size

| Variant | Frontend (min+gzip) | Rust binary (stripped, Linux x86_64) |
|---|---|---|
| Baseline (tokio + serde_json) | n/a | 411,256 B |
| C: `rusqlite` 0.40.2 bundled + `rusqlite_migration` 2.6.0 | `invoke` only: 0.1 kB | 1,509,416 B (**+1.10 MB**) |
| A: sqlx 0.8.6 with the plugin's feature set + migrate | `plugin-sql` JS: 0.4 kB | 1,774,568 B (**+1.36 MB**, before the plugin's own code) |
| A + Drizzle proxy | +17.0 kB | as A |
| A + Kysely + tauri dialect | +29.5 kB | as A |

Both Rust variants embed SQLite itself, which accounts for most of the delta.

### Recommendation

**C: domain-level Tauri commands (`save_build`, `list_builds`, `log_run`, …) over `rusqlite` 0.40.2 (`bundled`) with `rusqlite_migration` 2.6.0. Write raw SQL in Rust and don't put a query builder in TS.**

- **Rationale**: v1 needs atomic multi-statement writes (a Run log with its picks; a data-pack upgrade remapping Build references). Option A can't provide them from JS (#886), and Drizzle and Kysely only add weight on top of that gap. Keeping SQL in Rust also avoids granting the webview `sql:allow-execute` while it displays untrusted imported text. Zod still validates anything imported before it reaches a command.
- **Size impact**: about 0.1 kB frontend. Rust is about 0.26 MB smaller than the sqlx stack in this measurement.
- **Note**: this decides that SQLite access lives in Rust, which overlaps #17 (see Implications).

## 2. Frontend state management

| Library | Version (date) | Measured min+gzip | Notes |
|---|---|---|---|
| `zustand` | 5.0.16 (2026-10-22) | **0.47 kB** (hook); vanilla `createStore` 0.28 kB (esbuild) | 0 deps (npm). Repo pushed 2026-10-22, 4 open issues (GitHub). |
| `jotai` | 3.0.1 (2026-09-29) | 3.1 kB (`atom` + `useAtom`) | v3.0.0 shipped 2026-09-08 as "mostly backward-compatible", and 3.0.1 "fixes a regression in `useAtomValue` introduced in v3.0.0" ([releases](https://github.com/pmndrs/jotai/releases)). |
| `valtio` | 2.3.2 (2026-05-01) | 2.6 kB (bundlephobia) | Proxy-based. Last release 6 months ago. |
| Plain React (`useSyncExternalStore` + context) | React 19.3.1 | about 0.2 kB of glue | No dependency, but you hand-roll selectors and equality. |

Constraint that shapes the choice: the overlay and the companion are separate webviews (separate JS heaps). No in-memory store spans them. Shared state such as the current Run must live on the Rust side and reach each window through events. Tauri: "Global events are delivered to all listeners", and payloads are JSON with "no strong type support"; channels are for ordered or high-throughput streams ([Calling the frontend](https://v2.tauri.app/develop/calling-frontend/)). Zustand's `createStore` "lets you create a vanilla store" outside React ([docs](https://github.com/pmndrs/zustand/blob/main/docs/reference/apis/create-store.md)), so a `listen('run-state', e => store.setState(e.payload))` adapter can live outside the component tree.

**Recommendation: `zustand` 5.0.16, one store per window, fed by Tauri events or commands, with no persistence middleware (SQLite is the source of truth).**

- **Rationale**: it's the smallest store with selector-based subscriptions, and its vanilla core fits an event-fed store. It is very active, and it has no fresh major release like Jotai's 3.0.
- **Size impact**: about 0.5 kB per window.

## 3. Routing

- Each Tauri window has its own `url`, "the path portion of an app URL" ([Tauri config schema, `WindowConfig.url` / `WebviewUrl`, v2.12.5](https://github.com/tauri-apps/tauri/blob/tauri-v2.12.5/crates/tauri-schema-generator/schemas/config.schema.json)). Vite supports multiple `.html` entry points ([Vite multi-page app](https://vite.dev/guide/build#multi-page-app)). The overlay can therefore be `overlay.html` with its own small bundle and no router.
- The companion has a handful of screens (build library, build editor, live Run view, settings; map "Not yet specified").

| Library | Version (date) | Measured min+gzip | Notes |
|---|---|---|---|
| `wouter` | 3.13.0 (2026-09-30) | **3.1 kB** (with `useHashLocation`) | README: "only 2.1 KB gzipped vs 18.7KB React Router". Hash routing via `wouter/use-hash-location`, plus `nest` for nested routes ([README](https://github.com/molefrog/wouter)). Licence: Unlicense (npm), compatible with MIT. Repo pushed 2026-10-05. |
| `react-router` | 8.5.0 (2026-10-19) | 13.2 kB (declarative `HashRouter`) | Bundlephobia whole package: 63.7 kB. |
| `@tanstack/react-router` | 1.170.46 (2026-10-22) | 28.4 kB (esbuild) | Type-safe, but heavy for 4 screens. |
| None (state switch) | n/a | 0 kB | No params or back stack for `builds/:id`. |

**Recommendation: no router in the overlay (separate HTML entry). `wouter` 3.13.0 with `useHashLocation` in the companion.**

- **Rationale**: real params and nesting for the build editor at about a quarter of React Router's size. Hash URLs don't depend on how Tauri's asset protocol resolves deep paths on reload.
- **Size impact**: +3.1 kB, companion only.

## 4. Styling and UI primitives (transparent overlay + companion)

- Transparency is a window flag (`transparent`; on Windows, "Using `noRedirectionBitmap` can help avoid a white flash when creating a transparent window": [config schema v2.12.5](https://github.com/tauri-apps/tauri/blob/tauri-v2.12.5/crates/tauri-schema-generator/schemas/config.schema.json)). The page must also keep `html`/`body` backgrounds transparent. Overlay window behaviour is #3's subject. The styling layer only needs to avoid forcing an opaque background or a runtime style engine.
- **Tailwind CSS 4.3.3** (2026-07-16; repo pushed 2026-10-19). "Tailwind works by scanning your project for utility classes, then generating all of the necessary CSS based on the classes you've actually used… This makes sure your CSS is as small as possible" ([docs](https://tailwindcss.com/docs/detecting-classes-in-source-files)). It is a build step only and ships no runtime JS. Shared `@theme` tokens give both windows one palette, which helps the colour-blind-safe verdict colours (map: Accessibility).
- **Native primitives available in WebView2 (Chromium)**, from MDN BCD: `<dialog>` (Chrome 37), the `popover` attribute (Chrome 114), CSS anchor positioning (`anchor-name`, Chrome 125) and customisable `<select>` via `appearance: base-select` (Chrome 135). With an evergreen runtime these cover modal dialogs, menus, tooltips and selects with 0 kB JS.
- Headless React kits, measured as `Dialog` + `Popover`:
  - **Radix** (`@radix-ui/react-dialog` 1.3.1 + `@radix-ui/react-popover`; repo pushed 2026-10-14; 247 open issues): **23.5 kB**.
  - **Base UI** (`@base-ui/react` 1.9.0, 2026-10-09; the old `@base-ui-components/react` name is deprecated as "renamed to @base-ui/react", per npm): **37.2 kB**.
  - **Ark UI** (`@ark-ui/react` 5.39.3) depends on about 70 `@zag-js/*` packages (npm), and **React Aria Components** 1.22.1 on `react-aria` + `react-stately` (npm). Both are heavier ecosystems; neither was measured.

**Recommendation: Tailwind CSS 4.3.3 via `@tailwindcss/vite`, plus native HTML/CSS primitives. Add individual `@radix-ui/react-*` packages to the companion only when a native primitive falls short (e.g. a complex combobox). Use no headless kit in the overlay.**

- **Rationale**: no runtime cost in the overlay, which has a CPU budget. One token set covers both windows. Evergreen Chromium makes native dialog, popover and anchor positioning reliable.
- **Size impact**: 0 kB JS. The CSS is limited to the classes used. Each Radix escape hatch costs about 10–13 kB.

## 5. Share-code encoding: compression and alphabet

The format itself (fields, positional vs object JSON, version prefix) belongs to #16. This section picks the codec.

**Compression candidates**

| Option | Version (date) | Measured min+gzip | Maintenance |
|---|---|---|---|
| Native `CompressionStream` / `DecompressionStream` (`deflate-raw`) | Chromium 80 (API) / 103 (`deflate-raw`) per MDN BCD | **0.27 kB** glue (incl. base64url) | Platform. `brotli` and `zstd` formats are **not** in Chromium yet (BCD `version_added: false`). |
| `fflate` | 0.8.3 (2026-05-16) | 4.6 kB (`deflateSync` + `inflateSync` + `strToU8`/`strFromU8`). Inflate-only: 2.2 kB (esbuild). | Repo pushed 2026-05-16. Bundlephobia per-export: `deflateSync` 3.1 kB, `inflateSync` 2.0 kB. |
| `lz-string` | 1.5.0 (2023-03-04) | 1.5 kB | No release in 3.5 years. Repo pushed 2025-08-11 with 57 open issues (GitHub). |
| `pako` | 3.0.2 | 13.5 kB (bundlephobia) | Larger than fflate for the same DEFLATE. |

**Encoded length on illustrative Build payloads** (Node 24.14.1; not the real format):

| Payload | Raw JSON | base64url(raw) | `CompressionStream` deflate-raw → base64url | fflate deflate L9 → base64url | lz-string `compressToEncodedURIComponent` |
|---|---|---|---|---|---|
| Object JSON with string IDs | 482 | 643 | **388** | 392 | 452 |
| Positional array with numeric IDs | 108 | 144 | **94** | 96 | 97 |

Deflate beats lz-string on the larger payload and ties on the small one. Native and fflate output are within a few characters of each other.

**Alphabet**

- **base64url** ([RFC 4648 §5](https://www.rfc-editor.org/rfc/rfc4648#section-5)): `A–Z a–z 0–9 - _`. It's safe in URLs, filenames and chat. Native `Uint8Array.prototype.toBase64({ alphabet: 'base64url', omitPadding: true })` and `Uint8Array.fromBase64` are available from Chrome 140 per MDN BCD.
- **Base85 / Z85** ([ZeroMQ RFC 32](https://rfc.zeromq.org/spec/32/)) packs four bytes into five characters, against base64's three into four: about 6% shorter. Its alphabet includes `.-:+=^!/*?&<>()[]{}@%$#`. Characters like `&`, `?`, `#`, `%` and `+` are URL-special, and `*` is Discord markdown. The spec targets source code and the command line, not URLs. Base85 is also not native, so it needs `base-x` (0.9 kB, bundlephobia) or custom code.
- `@scure/base` 2.4.0 offers base64url in 1.4 kB if a library is wanted.

**Recommendation: native `CompressionStream('deflate-raw')`, then native base64url (no padding). Keep a roughly 10-line `btoa`/`atob` fallback for runtimes older than Chromium 140. Use `fflate` 0.8.3 only if #16 needs a synchronous API or byte-identical encoder output.**

- **Rationale**: about 35% shorter than raw base64url on the sample payloads, at zero dependency cost. base64url survives every target channel.
- **Caveat**: encoder output isn't guaranteed byte-identical across zlib builds, which is irrelevant unless codes are compared as strings. #16 should compare decoded Builds.
- **Size impact**: about 0.3 kB (fflate fallback: +4.6 kB).

## 6. Fuzzy string matching (TS side)

Scope: OCR output to item-name matching is in #4 (the "Fuzzy matching OCR output to known item names" bullet) and likely runs next to OCR in Rust. On the TS side the need is companion search and pickers over a few hundred to a few thousand reference items.

| Library | Version (date) | Measured min+gzip | Notes |
|---|---|---|---|
| `@leeoniya/ufuzzy` | 1.0.19 (2025-08-22) | **4.0 kB** | Designed for "list filtering, autocomplete/suggest" over short phrases, with an optional `intraMode: 1` allowing "a single error in each term" and `uFuzzy.highlight()` ranges ([README](https://github.com/leeoniya/uFuzzy)). Repo pushed 2026-09-08, 15 open issues. |
| `@nozbe/microfuzz` | 1.0.0 (2023-07-18) | 1.6 kB | Smallest, but no release in more than 3 years. Repo pushed 2025-05-22. |
| `match-sorter` | 8.3.0 (2026-04-15) | 3.6 kB (esbuild) | Depends on `@babel/runtime` and `remove-accents` (npm). |
| `fuzzysort` | 4.0.2 (2026-08-13) | 8.4 kB (esbuild) | |
| `fuse.js` | 7.5.0 (2026-07-13) | 9.3 kB | Bitap-based, largest. |

**Recommendation: `@leeoniya/ufuzzy` 1.0.19 for companion search and pickers. Don't use a TS library for OCR matching; #4 owns it.**

- **Rationale**: built for exactly this use, with typo tolerance and highlight ranges for the UI. It is maintained and about 40% the size of Fuse.
- **Size impact**: +4.0 kB, companion only.

## 7. neverthrow at the I/O boundaries

- `neverthrow` 8.2.0, released 2025-02-21. That is still the latest release about 20 months later ([releases](https://github.com/supermacro/neverthrow/releases)). The last commits (2026-02) are README changes, and there are 83 open issues (GitHub).
- Measured **2.0 kB** for `ok`/`err`/`ResultAsync.fromPromise`/`Result.combine`. It's effectively all-or-nothing: bundlephobia reports every single export at about 1.95 kB.
- The boundaries are already result-shaped without it:
  - Tauri commands reject with a serialisable typed error, documented as a tagged `{ kind: 'io' | 'utf8', message }` enum ([Calling Rust](https://v2.tauri.app/develop/calling-rust/)).
  - Zod's `safeParse` returns a discriminated success/error object.
  - The Recommendation engine is pure TypeScript with no I/O (map Notes).

**Recommendation: don't adopt neverthrow in v1. Use `try`/`catch` around `invoke`, Zod `safeParse` for imports, and, where a function must return failure as a value (e.g. Share code decode), a local `type Result<T, E> = { ok: true; value: T } | { ok: false; error: E }`.**

- **Rationale**: it adds no capability the boundaries lack, there are no recent releases, and the map's bar is "only if explicit Result types prove useful". #17 can revisit if the IPC layer grows.
- **Size impact**: 0 kB (saves 2.0 kB).

## 8. Zod 4 vs `zod/mini`: does it change the size story?

- `zod` 4.6.7 (2026-10-12) is `latest`, and the package still ships the v3 API at `zod/v3` (npm).
- Zod's own figures: `z.boolean().parse()` is 2.12 kb (mini) vs 5.91 kb (classic) gzip, and an object schema is 4.0 kb vs 13.1 kb. They advise: "In general you should probably use regular Zod unless you have uncommonly strict constraints around bundle size." Mini loads no default locale, so messages default to "Invalid input" unless you call `z.config(en())` ([Zod Mini docs](https://zod.dev/packages/mini)).
- Measured here (Rolldown):

| Schema | `zod` classic | `zod/mini` | `zod/v3` |
|---|---|---|---|
| `z.boolean().parse` | 14.9 kB | 3.0 kB | 14.3 kB (esbuild) |
| 5-field object (string, int ≥ 0, string[], enum, optional) + `safeParse` | 24.0 kB | 6.1 kB | 13.0 kB |

My absolute numbers are higher than Zod's published ones. My classic object schema uses more schema kinds, and Zod doesn't state its bundler. The ratio, mini at about 25% of classic, matches. Bundlephobia puts the whole package at 88.9 kB min+gzip, an upper bound that tree shaking avoids.

**Recommendation: `zod` 4.6.7 with the classic API (`import * as z from "zod"`) in the companion, where imports (data packs, Build imports, Share codes) are validated. Keep Zod out of the overlay bundle unless the overlay ends up validating something itself.**

- **Rationale**: the about 18 kB saved by mini is noise for assets loaded from local disk next to the 68 kB React baseline. Classic's chained API and default English messages matter for the user-facing decode errors #16 must design.
- **Size impact**: +24 kB in the companion only. So Zod 4 doesn't change the stack decision, only where Zod is loaded.

## Implications for the map

1. **#17 (architecture)**: this research recommends that **SQLite access lives in Rust**, as domain-level commands over `rusqlite`, rather than in TS through `tauri-plugin-sql`. #17 should ratify or reject that as an ADR. If it rejects it, it must accept no multi-statement transactions from JS (plugins-workspace#886) and the `sql:allow-execute` grant.
2. **#18 (SQLite schema and migrations)**: the migration tool becomes `rusqlite_migration` (it tracks state in SQLite's `user_version` field instead of a table, per its [README](https://github.com/cljoly/rusqlite_migration)), not `tauri-plugin-sql` migrations or `drizzle-kit`. Note the pin: `rusqlite` 0.40.x until `rusqlite_migration` supports 0.41.
3. **#16 (Share code format)**: the codec is deflate-raw + base64url (native). #16 still owns payload shape, versioning and length targets. In the samples here, a positional array compressed to 94 characters vs 388 for object JSON, which argues for positional.
4. **#17 / overlay**: two Vite HTML entries (overlay and companion) with separate bundles. Cross-window state is Rust state plus Tauri events, with one Zustand store per window.
5. **Watch item, not a blocker**: **Tauri 3.0 is in alpha** (`tauri` 3.0.0-alpha.5, 2026-10-14; `tauri-plugin-sql` 3.0.0-alpha.3; stable is 2.12.5, all per crates.io). Pin `tauri` 2.12.x / `@tauri-apps/api` 2.12.4 for v1. Also for #17: `tauri-specta` stable is 1.0.2, and the Tauri 2 line is still `2.0.0-rc.26` (crates.io). Typed IPC bindings would depend on an RC.

## Appendix: benchmark entries

Each entry imports the library and uses a minimal realistic surface. React, `react-dom` and `react/jsx-runtime` are external except in the baseline.

- **baseline**: `createRoot(...).render(<div/>)`.
- **zustand**: `create` + selector hook. **jotai**: `atom` + `useAtom`. **plain**: `useSyncExternalStore` counter.
- **wouter**: `Router hook={useHashLocation}` + `Switch` + 2 `Route` (one with `:id`) + `Link`. **react-router**: `HashRouter` + `Routes` + 2 `Route` + `Link` + `useParams`. **tanstack**: root + 1 route + `createHashHistory` + `RouterProvider`.
- **tauri-sql**: `Database.load` + `execute` + `select`. **tauri-invoke**: `invoke` from `@tauri-apps/api/core`.
- **drizzle-proxy**: `sqliteTable` with 3 columns + `drizzle(callback)` + insert + select/where `eq`. **kysely**: `Kysely` with SQLite adapter, compiler and introspector plus a stub driver + insert + select/where.
- **fflate**: `deflateSync(strToU8)` + `strFromU8(inflateSync)`. **lz-string**: `compressToEncodedURIComponent` + `decompressFromEncodedURIComponent`. **scure**: `base64urlnopad.encode/decode`. **native**: `CompressionStream('deflate-raw')` + `toBase64({alphabet:'base64url'})` and their inverses.
- **fuzzy**: one search call per library over `{name}` objects (uFuzzy over `string[]`).
- **neverthrow**: `ResultAsync.fromPromise(...).andThen(ok/err)` + `Result.combine`.
- **radix**: `@radix-ui/react-dialog` (Root/Trigger/Portal/Overlay/Content/Title/Close) + `@radix-ui/react-popover` (Root/Trigger/Portal/Content). **baseui**: the `@base-ui/react/dialog` + `/popover` equivalents.
- **zod**: as described in §8.
