# Build import paths: Mobalytics and other Nightreign build tools

Research for [#7](https://github.com/AbdulmajeedAlOmari/tarnished-tactician/issues/7). Checked on 2026-10-24.

**Question.** Which supported, permitted ways exist to bring a Build (Nightfarer + relic loadout/vessel + weapon/passive/talisman priorities) from Mobalytics or a similar Nightreign build tool into a free, open-source, local-first desktop app with no backend?

## Answer

- **Mobalytics has no public or partner API, and no export format.** Its Terms of Service forbid automated access by anything other than a normal web browser, and they forbid framing its pages. Any automated Mobalytics import is therefore **forbidden**. What remains is a person reading the page in their own browser and entering the Build by hand.
- **Two third-party planners put the whole build inside a share link** (nightreignbuilds.com and nightreignwiki.com). The app can decode such a link offline, without contacting the site. **relics.pro** offers user-initiated JSON export and import. These are the only paths I found that are clearly permitted and also yield structured data.
- **Recommended v1:** the manual "transcribe from a link" flow in the build editor, plus a source-URL field and an optional offline "paste text, match names" helper. Per-site import adapters (relics.pro JSON first, then nightreignbuilds.com links) can be added later as small, Zod-validated adapters once the Build model is settled.

## Mobalytics

### Terms of Service

These are the Mobalytics Terms of Service (Gamers Net, Inc.), "Last Updated: July 23, 2026". Mobalytics returns HTTP 403 to non-browser clients (both `curl` and an automated fetcher got 403 on `/terms/` and on the build pages), so I read the Wayback Machine capture from 2026-10-02 ([snapshot](https://web.archive.org/web/20261002154407/https://mobalytics.gg/terms/), live URL https://mobalytics.gg/terms/).

- **§4, Who May Use the Services.** "Gamers Net furnishes the Services for your personal, non-commercial use and enjoyment … a non-exclusive limited license to use the Services … for personal, non-commercial purposes only."
- **§7(a), Content Ownership.** "Gamers Net and its licensors exclusively own all right, title and interest in and to the Services and Content". "Content" includes text, graphics and images. Gamers Net "does not claim any ownership rights in any User Content", but by §7(b) a creator grants Gamers Net a licence to that content. Readers get no licence.
- **§8, General Prohibitions.** You agree not to:
  - "Attempt to access or search the Services or Content or download Content from the Services through the use of any engine, software, tool, agent, device or mechanism (including spiders, robots, crawlers, data mining tools or the like) other than the software and/or search agents provided by Gamers Net or other generally available third-party web browsers";
  - "Use, display, mirror or frame the Services or any individual element within the Services, Gamers Net's name (including "Mobalytics") … without Gamers Net's express written consent";
  - "Access, tamper with, or use non-public areas of the Services";
  - "Avoid, bypass … or otherwise circumvent any technological measure implemented by Gamers Net … to protect the Services or Content";
  - "Use the Services or Content … for the benefit of any third party";
  - "Encourage or enable any other individual to do any of the foregoing."

### robots.txt

[robots.txt](https://mobalytics.gg/robots.txt) has `Allow: /` for `*` and disallows only a few paths (`/api/tft`, `/api/valorant`, `/reward-pass`, non-English locale folders, and some Valorant/Deadlock pages). It says nothing about Nightreign. robots.txt is a hint for crawlers and does not override ToS §8.

### API and partner access

- I found no developer portal, public API documentation or partner programme. A web search for a Mobalytics public API or partner programme found only press and careers pages, which describe Mobalytics as a *consumer* of publisher APIs such as Riot's ([TechCrunch 2017](https://techcrunch.com/2017/08/15/mobalytics-launches-open-beta-of-its-gaming-analytics-coach), [careers](https://careers.mobalytics.gg)).
- The ToS mentions "API" only in two senses: a game-account connection flow for Reward Passes (§6), and passing a hashed email to the Overwolf API (§2). Neither is a developer API.
- The build pages' bootstrap config does reference internal GraphQL endpoints, including `https://mobalytics.gg/api/elden-ring-nightreign/v1/graphql/query` (found in the archived [Revenant builds page](https://web.archive.org/web/20260913134819/https://mobalytics.gg/elden-ring-nightreign/revenant-builds)). This is the site's private backend. It has no documentation, and using it from our app breaks §8 ("non-public areas" and access by a tool other than a browser).

### Build page structure

I inspected the archived [Libra Expedition Revenant build](https://web.archive.org/web/20250821124049/https://mobalytics.gg/elden-ring-nightreign/builds/libra-expedition-revenant-build), captured 2025-08-21.

- **URLs.** Guides are at `/elden-ring-nightreign/builds/<slug>`. Community builds have an id suffix, e.g. `/builds/0-rng-revenant-jt5609`. Per-Nightfarer hubs are at `/elden-ring-nightreign/<nightfarer>-builds`, creator profiles at `/elden-ring-nightreign/profile/<user>/builds`, and the editor at `/elden-ring-nightreign/planner/builds`. Mobalytics' own planner page says editing is desktop-only.
- **Structured data.** The page is server-rendered with a `window.__PRELOADED_STATE__` GraphQL cache. The build is an `EldenRingNightreignUserGeneratedDocument` made of widgets. The "Equipment Loadout" widget (`NgfDocumentCmWidgetGameDataCardGridV2`) lists items as `{slug, groupSlug, title, subTitle, imageUrl}`. In the sampled build the `groupSlug`s were `relics`, `weapons`, `weaponPassives`, `dormantPowers`, `talismans`, `spells` and `consumables`, with slugs such as `revenants-cursed-claw` and `green-turtle-talisman`. The Nightfarer is a `class` tag. The day-by-day game plan and tips are rich text. The sampled build had **no vessel field and no relic-to-slot/colour mapping**: relics were listed as items. There is no JSON-LD build schema, only OpenGraph meta tags.
- **Share and export.** The page has a "Share" button (the string `"Share {{documentType}}"`) and a shared-images host (`shared-images.mobalytics.gg`). I found no export, copy-as-code or embed feature.

**What Mobalytics yields against our Build model:** the Nightfarer, relics by name, weapons, weapon passives, dormant powers, talismans and spells, plus the author's prose. It does not give a vessel or slot colours. The data is good for a person to transcribe. The ToS rules out extracting it with software.

## Other Nightreign build tools

| Tool | Licence / terms | How builds leave the tool | Data in the payload |
|---|---|---|---|
| **relics.pro** ([site](https://relics.pro/), [Contact & Legal](https://relics.pro/contact)) | Closed source. ToS: "provided for personal, non-commercial use"; "© 2025 Relics.pro. All rights reserved for original website content". It has no scraping clause. [robots.txt](https://relics.pro/robots.txt) allows `/builds/*`. | The user clicks **Export** to download a `.json` file and **Import** to load one. The importer accepts one build object or an array, up to 1 MB. Shared builds are served as static `/build-data/<id>.json` (from its `BuildsPage` and `builds` JS bundles, read 2026-10-24). | `{name, author, description, recommendedCharacters[], effects{<effectId>: weight}, synergies[{effects[], weight}], required[], forbidden[], …}`. Effects use **numeric game effect IDs**: e.g. `7000001`, which is "Vigor +2" in the MIT [metinc/nightreign-relic-browser](https://github.com/metinc/nightreign-relic-browser) data as vendored by [nightreign-relic-vault](https://github.com/tonydavis629/nightreign-relic-vault/blob/HEAD/public/data/game-data.json). This is a weighted **relic-effect priority** list, not a concrete loadout. It has no weapons or talismans. |
| **nightreignbuilds.com** ([site](https://www.nightreignbuilds.com/), [Steam thread](https://steamcommunity.com/app/2622380/discussions/0/603042323028591841/)) | No ToS, licence or source link on the page. `robots.txt` returns 404. | **Share link** `?s=<base64(JSON)>`. It is built by `btoa(JSON.stringify(state))` and decoded with `JSON.parse(atob(s))` (from its JS bundle `index-jdOXMqdS.js`, read 2026-10-24). Saved builds stay in browser `localStorage`. | `{char, vessel, relics: [relicName\|null …], randoms: [[effectName\|null ×3] …]}`, keyed by **English display names**, with data version `1.03.5`. This gives the Nightfarer, vessel and relics per slot. It has no weapons or talismans. |
| **Nightreign Wiki Relic Build Simulator** ([tool](https://nightreignwiki.com/tools/relic-builder/), [about](https://nightreignwiki.com/about/)) | No licence. The about page asks people to credit "Nightreign Wiki" and link the page. [robots.txt](https://nightreignwiki.com/robots.txt) allows all crawlers. | **Share link** `?char=&boss=&slots=3..5&relics=id1,id2,…` (inline page script `syncUrl()`). | `relics` are about 32 site-invented archetype ids (e.g. `flask-heal`, `status-weapon`), not real relics. This is too coarse for our Build. |
| **nightreignbuilds.fyi** ([site](https://nightreignbuilds.fyi), [repo](https://github.com/tannart/nightreignbuilds)) | Source is public but has **no licence**, so the default is all rights reserved. robots: `Content-Signal: search=yes,ai-train=no,use=reference`. | The share link makes an unlisted server-side build at `/b/<slug>/`. The views are HTML only and there is no JSON route ([`builds/urls.py`](https://github.com/tannart/nightreignbuilds/blob/HEAD/builds/urls.py)). | Nightfarer, vessel, Deep of Night 3 or 6 slots, relics, weapons (per the [README](https://github.com/tannart/nightreignbuilds)). Getting it means fetching and scraping HTML. |
| **BlueWhiteCloud/nightreign-relic-build-tool** ([repo](https://github.com/BlueWhiteCloud/nightreign-relic-build-tool)) | MIT. Chinese UI. | `.json` "导出配置" (export config) and a pasteable "配置码" (config code). Ships preset JSONs per Nightfarer. | Relic-rule conditions (format not inspected in depth). |
| **Save-file tools**: [DankYeeter/Nightreign-Helper](https://github.com/DankYeeter/Nightreign-Helper) (MIT code), [nightreign-relic-vault](https://github.com/tonydavis629/nightreign-relic-vault) (MIT), [metinc/nightreign-relic-browser](https://github.com/metinc/nightreign-relic-browser) (MIT), [calculator.smith-c.com](https://calculator.smith-c.com/) | Varies. | They read the player's `.sl2` save read-only. | The player's **relic inventory** and current loadout. This is not someone else's Build, so it is out of scope here (see Implications). |
| **Maxroll** | [robots.txt](https://maxroll.gg/robots.txt) header: "Use of any robot, crawler, or other tool to scrape … is prohibited without written permission from Ziff Davis." | I found no Nightreign planner or export. | Guides only. Manual transcription only. |

## Paths rated

"Permitted" means nothing in the source's own terms forbids it and our app never touches the source's servers in a way they restrict. "Grey" means no explicit rule against it, but it depends on undocumented behaviour, or the terms are silent or unclear. "Forbidden" means it conflicts with a stated term.

| # | Path | Rating | Why | What it yields for our Build |
|---|---|---|---|---|
| 1 | **Manual transcription.** The user opens a Mobalytics (or any) build in their own browser and fills in our build editor. We store the source URL and author as attribution. | **Permitted** | The user reads the page in "generally available third-party web browsers" (ToS §8). Our app never contacts Mobalytics. We store only game-item selections, which are game facts, not Mobalytics' prose or images. | Everything the user enters: Nightfarer, relics, vessel, weapons, passives, talismans, priorities. |
| 2 | **"Open source link" button.** Hands the stored URL to the system browser (`opener`/`shell.open`). | **Permitted** | It is an ordinary link-out. §9 contemplates links to third parties. | None (convenience). |
| 3 | **Paste text, match names (offline).** The user copies text from any build page and pastes it into the editor. We fuzzy-match it against our own reference-data names and propose selections for the user to confirm. | **Permitted / low-grey** | Our software never accesses the site. The user copies by hand in their browser. The only Mobalytics clause on "extract" is aimed at AI Training (§8). We keep none of the pasted text after matching. It works for any source (Mobalytics, Fextralife, Game8, YouTube descriptions). | Item names that match reference data. The Nightfarer, vessel and slot colours may need confirming. |
| 4 | **relics.pro JSON file import.** The user clicks Export on relics.pro and imports the `.json` into us. | **Permitted** | This uses relics.pro's own user-facing export feature, on the user's own data, offline. relics.pro's ToS has no relevant restriction. | Relic-effect priorities keyed by game effect ID, `recommendedCharacters`, `required`/`forbidden` effects. No weapons or talismans, and no concrete relics. |
| 5 | **nightreignbuilds.com share-link decode (offline).** The user pastes a `?s=` link and we base64-decode the JSON locally. | **Permitted / grey** | Parsing a string the user pasted is not access to the site, and no terms exist. The format is undocumented and may change. Names must be mapped to our IDs. | Nightfarer, vessel, relic per slot, the random effects on each relic. |
| 6 | **Nightreign Wiki share-link decode (offline).** | Permitted, but **not worth building** | Same reasoning as #5. The ids are coarse archetypes, not relics. | Nightfarer, slot count, relic "archetypes". |
| 7 | **relics.pro shared-build fetch** (`/build-data/<id>.json`) from a pasted link. | **Grey** | This is an undocumented internal endpoint. The ToS is silent and robots allows `/builds/*`. It adds a network call to a third party, so it would need to be opt-in under our local-first rule. Better to ask the relics.pro owner first. | Same as #4. |
| 8 | **nightreignbuilds.fyi share-link fetch and HTML scrape.** | **Grey (leaning no)** | The code has no licence, there is no JSON route, and robots signals `use=reference`. Scraping HTML is brittle. | Nightfarer, vessel, relics, weapons. |
| 9 | **Mobalytics page fetch from our app** (HTTP or a hidden webview), parsing `__PRELOADED_STATE__`. | **Forbidden** | ToS §8: access "through the use of any … software, tool, agent" other than a browser. Non-browser clients get 403, so working around that would also breach the §8 clause against circumventing "technological measure[s]". | Would yield the full item list (see above). |
| 10 | **Mobalytics internal GraphQL** (`/api/elden-ring-nightreign/v1/graphql/query`). | **Forbidden** | ToS §8 "non-public areas" plus the same automation clause. Undocumented. | Same as #9. |
| 11 | **Embedding a Mobalytics page in our window** (iframe or webview) to show it beside the editor. | **Forbidden** | ToS §8: "display, mirror or frame the Services … without … express written consent." | Display only. |
| 12 | **A browser extension or bookmarklet that we ship** to pull Mobalytics page state into our app. | **Forbidden** | It runs in the user's browser, but it is still a "tool … mechanism" used to "download Content", and by shipping it we would "encourage or enable" a breach (§8). | Full item list. |
| 13 | **Asking Mobalytics for written permission or a partner feed.** | Permitted to ask | Only express written consent unlocks #9–#12. Nothing suggests such a programme exists. Possible later, not something v1 can depend on. | Unknown. |
| 14 | **Maxroll, Game8, Fextralife automated import.** | **Forbidden** (Maxroll) / not assessed (others) | Maxroll robots header forbids scraping without Ziff Davis's permission. | Guide prose only. |

## Recommendation for v1

1. **Ship path #1 as the primary import:** the build editor with a "Source" field (URL plus author or creator name). Show it as attribution on the Build and give it an "Open source" button (#2). This is the fallback the ticket names, and it is the only path that works for Mobalytics.
2. **Add path #3 (paste text, match names) if it is cheap** once the reference-data name index exists. It works with every source, needs no network and no per-site format, and it reuses the name matching the OCR pipeline already needs to map English item text to stable IDs.
3. **Defer format adapters (#4 and #5) to post-v1, or ship #4 if time allows.** They are permitted and give structured data, but each covers only part of our Build. relics.pro gives effect priorities without weapons. nightreignbuilds.com gives a concrete relic loadout without weapons. Each adapter is a small Zod-validated `parse(external) → Build draft` that opens in the editor for the user to confirm, never a silent import. relics.pro is the better first adapter: its effect IDs are numeric game IDs, so it maps cleanly onto ID-keyed reference data.
4. **Do not build** #9–#12, and never present the app as a "Mobalytics importer". ToS §8 forbids using Mobalytics' name without consent, and the project bans third-party branding anyway.

## Implications for the map

- **Build model ticket:** relics.pro and Mobalytics both describe builds as *prioritised relic effects* (weights, required, forbidden). nightreignbuilds.com, nightreignbuilds.fyi and save-file tools describe a *concrete vessel plus relic-per-slot loadout*. The Build model should decide whether it holds one or both. Importers need a place for each.
- **Reference data ticket:** key relic effects by the game's numeric effect IDs (as relics.pro and the MIT save-file tools do), and keep an English-name index. That makes adapters #4 and #5 and the paste-matcher (#3) nearly free.
- **Possible new ticket, relic-inventory import from the save file:** several MIT tools read `NR0000.sl2` read-only and fully offline. That is the player's *owned relics*, not a Build, but it would let Recommendations suggest a loadout from what the player actually has. It does not read process memory, so it stays inside the standing constraints. It needs its own research (save format, decryption, legality and anti-cheat stance).
- **Curated starter Build set (not yet specified):** do not seed it from Mobalytics or relics.pro presets. Mobalytics content is all rights reserved, and relics.pro claims rights in its original content. Author the starter set ourselves or get explicit permission.

## Method and limits

- Mobalytics blocks non-browser clients, so I read its ToS and two build pages through Wayback Machine captures. The ToS capture (2026-10-02) matches the live "Last Updated: July 23, 2026" text that appears in search results. The sampled build page is from 2025-08 and its schema may have changed since.
- relics.pro and nightreignbuilds.com formats come from reading their shipped JS bundles on 2026-10-24. I made a handful of requests and did no bulk fetching. These formats are undocumented and can change without notice.
- This is not legal advice. The ratings are a plain reading of each site's published terms.
