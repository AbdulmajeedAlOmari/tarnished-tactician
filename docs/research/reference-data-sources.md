# Game reference data sources and their licences

Research for [#6](https://github.com/AbdulmajeedAlOmari/tarnished-tactician/issues/6). Researched 2026-10-24.

**This is not legal advice.** It is an engineering summary of published terms and licences. Where the law is unsettled, or the publisher's terms read broadly, this note says so. Get advice from someone qualified before the first public release if the risk matters.

## Question

Where can Nightreign game reference data (items, passives, relics, Nightfarers, numbers) come from for versioned JSON, and under what licences? For each source: completeness, structure, how fast it updates after a patch, whether it can be redistributed in an MIT-licensed, non-commercial app, and whether item icons can be redistributed or only used locally. Recommend a primary source and an extraction approach, and say what licence the shipped data packs should carry.

## Answer

- **Primary source: the game's own param and text files**, exported by maintainers with Smithbox (MIT) using its Nightreign paramdefs (MIT). Nothing else is as complete or as fast after a patch, and every row has a stable numeric ID, which suits locale-keyed JSON. Wikis are for cross-checking only.
- **Extraction happens on maintainer machines, never in the app.** Raw exports (CSV/FMG) stay out of the public repo. The repo holds only the transformed, minimal JSON plus the transform script.
- **No source gives a licence that covers the game's content.** FromSoftware's Terms of Service and the Bandai Namco EULA reserve all rights in game information, images and text. Their clauses on reverse engineering and redistribution are broad enough to cover datamining and data packs if read literally. Every Nightreign wiki and tool already works in that zone. Treat it as a residual, unlicensed risk and keep the shipped data minimal.
- **Icons: don't redistribute them.** Ship no game icons in the app or the data packs. If detection needs icon templates, build them on the user's machine from the user's own screen (opt-in), or rely on OCR of names. This goes back to the detection ticket.
- **Data-pack licence:** project-authored content (tags, weights, synergy notes, schema, our own selection and arrangement) under **CC BY 4.0**. Game-derived names, text and numbers carry a notice that they are © FromSoftware, Inc. / Bandai Namco, included for interoperability and not licensed by us. Don't copy wiki prose (CC BY-SA would force share-alike on the whole pack) or Fextralife content (its terms forbid it).

## Comparison

| Source | Completeness | Structure | Update cadence after a patch | Licence / redistribution | Icons |
|---|---|---|---|---|---|
| **Game files via Smithbox export** (maintainer-run) | Complete and authoritative: every relic (`EquipParamAntique`), relic/weapon effect (`AttachEffectParam`, `AttachEffectTableParam`), weapon, talisman, item and Nightfarer param, plus English names from FMG text. Some fields are still `unknown_*`. | Tabular rows keyed by stable numeric IDs; CSV export of all rows or all params; text exported separately. | Same day, if the paramdefs still match the new regulation. Smithbox shipped "NR Param Upgrade" and "NR Meta" commits in May to July 2026. | The tool is MIT. The data is FromSoftware's: ToS Art. 6(2) and EULA §3–4 reserve all rights. No licence is available. | Icons are reachable through `iconId` and the texture viewer, but they are copyrighted game art and must not be redistributed. |
| **Paramdex** (`soulsmods/Paramdex`, `NR/`) | NR defs and row names only; no data. | XML paramdefs and text row-name lists. | NR folder last changed 2025-07-17, so it is stale against the DLC and 1.03.x. | **No licence file** (GitHub returns 404 for the licence), so all rights are reserved by default. Use Smithbox's MIT copies instead. | n/a |
| **Community GitHub datasets** (e.g. `ip1259/…Legal-Relic-Generator`, `slavone/…relic_calculator`, `cetusk/ERN_RelicForge`, `tonydavis629/nightreign-relic-vault`) | Relics and relic effects only. | Raw Smithbox CSV + FMG XML (ip1259), or ad-hoc JSON. | Hobbyist and irregular. ip1259 was last pushed 2025-12-25, before 1.03.2, and its `AttachEffectParam.csv` has no Scholar/Undertaker flags. | GPL-3.0, MIT or none, depending on the repo. Any of these licences covers only the author's own code and arrangement, never the game data inside. | None shipped as datasets. |
| **xxiixi/NightreignQuickRef** | Broad: relics, Nightfarer stats, mechanics, DLC. | App-embedded data, Chinese-first. | Active; last pushed 2026-04. | GPL-3.0. Compiled from Tieba and Bilibili posts and the Fextralife wiki, so its provenance is mixed and partly third-party. | Unclear. |
| **Eldenpedia** (`eldenring.wiki.gg`, `Nightreign:` namespace) | Good: more than 500 Nightreign pages; edits continue into Oct 2026. | Wikitext with templates; scraping would mean parsing pages. | Days to weeks; 16 active users site-wide. | Text under **CC BY-SA 4.0**. Share-alike applies to adaptations. Game text quoted on the wiki is not the wiki's to license. | Images are not covered by the text licence; assume they are copyrighted game assets. |
| **Fandom Elden Ring Wiki** (`eldenring.fandom.com`) | Partial; the "Nightreign Effects" page was last edited 2025-12-05. | Wikitext. | Slow for Nightreign. | Text under **CC BY-SA** (Fandom default 3.0, which adaptations may move to 4.0). | Images rely on fair use, not CC. |
| **Fextralife Nightreign wiki** (Valnet) | Broad (relics, effects, weapons, Nightfarers, DLC). | HTML pages. | Fast. | Valnet ToS **forbids scraping, data-mining and redistribution**, and images may not be republished. The wiki uses a "Custom License". **Unusable.** | Unusable. |
| **Commercial guide sites** (Mobalytics, Maxroll) | Broad. | Web UI only; no public API found. | Fast. | No open licence found. Mobalytics' terms returned 403 to the fetcher, so treat it as all rights reserved. **Unusable.** | Unusable. |
| **Public Nightreign API / open dataset** | None found. | – | – | – | – |

## Findings

### 1. Game files are the only complete, fast source

- Smithbox is MIT-licensed and supports "Elden Ring: Nightreign". Its Param Editor edits "parameters, which contain most of the data that defines each game", its Text Editor edits localisation text, and its Texture Viewer is "a simple to use viewer for looking and extracting textures". It needs no prior unpacking ("You no longer need to unpack a game for any of the editors"). [Smithbox README][smithbox-readme], [LICENSE][smithbox-license]
- Smithbox's Param Editor has "Export CSV" with "All Rows", "Selected Rows", "Modified Rows" and "All Params" options. The Text Editor has an exporter modal. [ParamDataExportMenu.cs][smithbox-export], [TextExporterModal.cs][smithbox-textexport]
- Smithbox keeps Nightreign paramdefs, meta and English row names under `src/Smithbox.Data/Assets/PARAM/NR/` (MIT, same repo). Recent commits touching it: "NR Param Upgrade" (2026-05-11), "NR: Update Aggro System Parameter Names" (2026-06-21), "NR Meta" (2026-07-03). The latest release is 2.2.6 (2026-09-20). [Smithbox NR assets][smithbox-nr], [releases][smithbox-releases]
- The NR defs model exactly what the Recommendation engine needs:
  - `AttachEffectParam` (relic and weapon effects) has `compatibilityId`, `attachTextId` (FMG name), `isDebuff`, per-Nightfarer flags (`allowWylder` … `allowScholar`, `allowUndertaker`), `statusIconId` and `exclusivityId`.
  - `EquipParamAntique` (relics) has `relicColor`, `isDeepRelic`, `iconId`, three `attachEffectTableId_*` slots and three curse slots.
  - There are also `HeroParam` / `HeroStatusParam` (Nightfarers), `EquipParamWeapon`, `EquipParamAccessory` (talismans) and `EquipParamGoods`.

  [AttachEffectParam.xml][def-attach], [EquipParamAntique.xml][def-antique]
- A real export (from a third-party repo, inspected read-only) has a header row `ID,Name,onHitSpEffect,…` and integer rows keyed by ID. The English names are FMG XML (`<text id="310000">Increased Maximum HP</text>`), with separate base and `_dlc01` files per locale. Because the same text ID exists in every locale, this gives the stable-ID, locale-keyed shape the map asks for. [ip1259 AttachEffectParam.csv][ip-csv], [ip1259 en/AttachEffectName.fmg.xml][ip-fmg]
- Effect magnitudes live behind SpEffect IDs (`passiveSpEffectId_*`, `onHitSpEffect`) and some fields are still `unknown_*`. Turning raw rows into human-meaningful numbers takes extra work, and wikis help cross-check it. (Inference from the defs above.)
- Patches separate **App Ver.** from **Regulation Ver.**: for example, 1.03 shipped as "App Ver. 1.03 / Regulation Ver. 1.03.1", and consoles download regulation files from the server. Data packs should record the **regulation version** they were built from. [Patch notes 1.03][pn-103]. 1.03.2 (2026-01-15) reported "Regulation Ver. 1.03.4" [(secondary: mp1st)][pn-1032].
- Lower-level alternatives: SoulsFormatsNEXT (GPL-3.0) reads the formats, WitchyBND (GPL-3.0) serialises PARAM and FMG to XML and has "basic initial support for ELDEN RING NIGHTREIGN", and Nuxe (GPL-3.0) unpacks "Elden Ring Nightreign (PC)" archives. [SoulsFormatsNEXT][sfn], [WitchyBND README][witchy], [Nuxe README][nuxe]. Per the GPL FAQ, a program's output "inherits that of the input it was generated from", so using a GPL tool does not make the exported data GPL; the data stays FromSoftware's. A maintainer script that links SoulsFormatsNEXT would itself be GPL-3.0, so keep it outside the MIT app code (a separate repo or folder). [GPL FAQ #GPLOutput][gplfaq]

### 2. Paramdex has no licence and its NR folder is stale

- Paramdex is "a collection of resources for working with params via SoulsFormats". It holds `NR/Defs` and `NR/Names`, but no `Tdfs` for NR. [Paramdex README][paramdex]
- The repository has no licence (the GitHub licence API returns 404). The last commits under `NR/` date from 2025-07-12 to 07-17, and one of them is "Add defs and TDFs from Smithbox", so Smithbox is upstream. Use Smithbox's MIT copy. [Paramdex NR commits][paramdex-commits]

### 3. Community datasets don't solve licensing

- ip1259's repo (GPL-3.0, last pushed 2025-12-25) commits raw `AttachEffectParam.csv`, `AttachEffectTableParam.csv`, `EquipParamAntique.csv` and FMG XML for `en` and `tcht`. In other words, it redistributes game data under a code licence the author has no power to grant for that data. [ip1259 repo][ip-repo]
- slavone's calculator (no licence) builds its JSON from the ip1259 files. [slavone README][slavone]. Other relic tools are MIT or unlicensed: `metinc/nightreign-relic-browser` MIT, `tonydavis629/nightreign-relic-vault` MIT, `cetusk/ERN_RelicForge` MIT, `Y4rd13/nightreign-relic-optimizer` none, `petersquall/…relic-scanner` none. (Licences read from GitHub repository metadata.)
- `xxiixi/NightreignQuickRef` is GPL-3.0 and cites Tieba threads, Bilibili videos and the Fextralife wiki as its sources. [QuickRef README][quickref]
- Precedent for our app category: `NeuraXmy/nightreign-overlay-helper` (an AGPL-3.0 desktop overlay) lists notices for its code dependencies and none for game data or assets. [overlay-helper notices][overlay-notices]. This shows community norms, not permission.

### 4. Wikis: usable for cross-checking, risky to copy

- **Eldenpedia (wiki.gg):** the MediaWiki rights info is "Creative Commons Attribution-ShareAlike 4.0 License". The Nightreign namespace (3000) has more than 500 pages, and there were recent edits on 2026-10-10 and 2026-10-14. Site statistics show 16 active users. [wiki.gg siteinfo API][wikigg-api], [recent changes][wikigg-rc]
- **Fandom:** rights info is "CC-BY-SA", linking to fandom.com/licensing. Its "Nightreign Effects" page was last edited 2025-12-05. [Fandom siteinfo API][fandom-api], [search API][fandom-search]. Fandom's licensing page could not be fetched directly (HTTP 402). Per search excerpts, it puts text under CC BY-SA 3.0 by default, and its community creation policy relies on fair use for game screenshots. [fandom.com/Licensing][fandom-lic], [Community creation policy][fandom-ccp]. CC BY-SA 3.0 adaptations may be distributed under "a later version of this License with the same License Elements". [CC BY-SA 3.0 §4(b)][ccbysa3]
- **Share-alike would spread to the whole pack.** CC BY-SA 4.0 §3(b) requires an adapter's licence to be BY-SA (same or later) or BY-SA Compatible, and §4 treats a database built from "all or a substantial portion" of a BY-SA database as Adapted Material. Copying wiki tables into a pack would therefore force the pack under BY-SA. [CC BY-SA 4.0][ccbysa4]
- A CC licence covers only rights "the Licensor has authority to license". Item descriptions or icons that a wiki copied from the game are not licensed by the wiki. [CC BY 4.0 §1, §2][ccby4]
- Facts themselves are not copyrightable in the US: "Facts, whether alone or as part of a compilation, are not original and therefore may not be copyrighted". A compilation's copyright is "thin", covering only its selection and arrangement. Checking a number against a wiki and recording it in our own structure does not copy the wiki. [Feist v. Rural, 499 U.S. 340][feist]
- **Fextralife** is "part of the Valnet Publishing Group". Its contributions are under a "Custom License", and fextralife.com/terms-of-use redirects to Valnet's terms. Those terms (updated 2026-01-12) prohibit users to "Frame, scrape, automatically download, data-mine, extract, collect, or harvest any content", forbid republishing images, and limit reproduction to non-commercial extracts with credit. Exclude Fextralife as a source. [Fextralife Nightreign wiki footer][fextra], [Valnet Terms][valnet]

### 5. Publisher terms: everything here is unlicensed

- **FromSoftware Nightreign ToS:**
  - Art. 6(1): all rights in the Game belong to the Company.
  - Art. 6(2): "Players shall not be entitled to alter, adapt, copy, edit, reproduce, distribute, transmit or publish all pieces of information included in the Game (any and all kinds of information including, but not limited to images, pictures, characters, …) without the prior approval of the Company."
  - Art. 10(2)(3) prohibits "reverse engineering the Game, decompiling the Game, disassembling the Game and/or extracting the Game's source code".
  - Art. 10(2)(2) prohibits altering game data or using altered data.

  [Nightreign Terms of Service][tos]
- **Bandai Namco EULA (Steam):**
  - §3(a) grants "a personal, non-exclusive, non-transferable, limited license to use the Game for noncommercial use".
  - §3(b) bars reverse engineering, decompiling and disassembling, as well as distributing or copying the Game and creating derivative works.
  - §4 says the Game and its content are Bandai Namco's or its licensors' property.
  - §10(i) bars "unauthorized third party tools".

  [Nightreign EULA][eula]
- **FromSoftware video/image guideline** (last updated 2022-03-15):
  - It covers only individuals resident in Japan posting videos and screenshots.
  - It explicitly does not license "MODs or derivative works".
  - It prohibits implying official endorsement.
  - It does not address apps, tools, datasets or icons.

  [FromSoftware guideline][fs-guide]. **No Bandai Namco or FromSoftware fan-content or fan-tool policy was found.** Searches turned up only Bandai Namco's corporate website-download terms, which allow non-commercial personal use only. [Bandai Namco terms][bn-terms]
- **What this means (flagged uncertainty):**
  - Read literally, ToS Art. 6(2) covers *any* redistribution of in-game information, including names and numbers, and every wiki and tool listed here would breach it. It is a contractual term binding players, separate from copyright.
  - US copyright likely doesn't protect bare facts and numbers (Feist), but item descriptions, flavour text and icons are creative works and stay protected.
  - The EU sui generis database right protects a maker's "substantial investment" against extraction of a substantial part, but only for makers who are EU nationals or residents, or companies established in the EU. FromSoftware is Japanese; whether Bandai Namco Entertainment Europe could claim the right is unknown. [Directive 96/9/EC Art. 7][eu-art7], [Art. 11][eu-art11]
  - Japan protects a database as a work only "by reason of the selection or systematic construction". [Japan Copyright Act Art. 12-2][jp-cr]
  - Whether reading param files with community paramdefs counts as "reverse engineering" under the ToS is untested here.
  - The practical stance: free and non-commercial, no branding, no game art, no flavour text, the minimum data needed, attribution to FromSoftware, and willingness to respond to a takedown request. This reduces risk; it does not remove it.

### 6. Icons

- Relics and effects carry `iconId` / `statusIconId`, and Smithbox can extract textures. Technically, maintainers could export every icon. [AttachEffectParam.xml][def-attach], [Smithbox README][smithbox-readme]
- But icons are images, the clearest case under ToS Art. 6(2) ("images, pictures"). The EULA §4 says content is Bandai Namco's. Valnet forbids republishing images, and Fandom images rest on fair use, not a licence. No source licenses icon redistribution. [ToS][tos], [EULA][eula], [Valnet][valnet], [Fandom CCP][fandom-ccp]
- **Local-only options**, best first (choosing between them is a detection-ticket decision):
  1. OCR the relic and item **names** and match them to IDs; no icons needed.
  2. **User-side template capture:** an opt-in calibration that crops icons from the user's own screen and stores them locally.
  3. Read icons from the user's own install at runtime. This needs archive decryption and format parsing in-app, which pulls in GPL code, sits closest to the ToS "reverse engineering" clause, and is not recommended.

## Recommended pipeline (for the data-pack publishing ticket)

1. A maintainer updates Smithbox after a patch and confirms the NR defs load the new regulation without errors.
2. Export "All Rows" CSV for the params the schema uses (`EquipParamAntique`, `AttachEffectParam`, `AttachEffectTableParam`, `EquipParamWeapon`, `EquipParamAccessory`, `EquipParamGoods`, `HeroParam`, `HeroStatusParam`, plus the SpEffect rows they reference). Export the English FMG text for the matching names (base + `_dlc01`).
3. Run a committed transform script (TypeScript, project code, MIT) that reads those exports from a **git-ignored** input folder. It writes minimal JSON keyed by param row ID: only the fields the Recommendation engine uses, names only, no flavour or description text. The JSON is then Zod-validated.
4. Record `regulationVersion`, `appVersion`, the Smithbox version and the source param list in the pack manifest.
5. Diff against the previous pack to produce patch-day release notes. Cross-check changed numbers against Eldenpedia and patch notes by hand; don't copy text.
6. If automation is wanted later, write a separate GPL-3.0 helper built on SoulsFormatsNEXT, kept out of the MIT app code.

## Data-pack licence

- **Project-authored layer: CC BY 4.0.** This covers tags, Recommendation weights, synergy notes, our schema, and our selection and arrangement of data. CC BY suits data better than MIT and keeps attribution. CC0 is the alternative if maximum reuse matters more than credit.
- **Game-derived content: not licensed by us.** Each pack carries a `NOTICE`: "Game names, values and identifiers are from ELDEN RING NIGHTREIGN, © FromSoftware, Inc. / Bandai Namco Entertainment. They are included for interoperability in a free, non-commercial fan tool, are not covered by this pack's licence, and this project is not affiliated with or endorsed by FromSoftware or Bandai Namco."
- **Contribution rule:** no text copied from wikis (to avoid BY-SA spreading to the pack) and nothing from Fextralife or Mobalytics/Maxroll.

## Implications for the map

- **Detection spike / OCR ticket:** treat icon templates as unavailable for shipping. Prefer name-OCR. User-side icon capture conflicts with "frames are never stored" unless it becomes an explicit opt-in exception, so it needs a decision.
- **Data-pack publishing workflow** (Not yet specified): has a concrete starting point (pipeline above). Key packs by regulation version.
- **Mobalytics Build import:** Mobalytics' terms could not be read (403), so that ticket should check its terms directly.
- **Release readiness:** consider a short legal sanity check, or simply reaching out to Bandai Namco, before the first public release, given ToS Art. 6(2).
- The data packs carry a licence different from the code: a repo-level `LICENSE` (MIT) plus `data/LICENSE` (CC BY 4.0) and `data/NOTICE`.

## Sources

[smithbox-readme]: https://github.com/vawser/Smithbox/blob/main/README.md
[smithbox-license]: https://github.com/vawser/Smithbox/blob/main/LICENSE
[smithbox-export]: https://github.com/vawser/Smithbox/blob/main/src/Smithbox.Program/Editors/Param%20Editor/Tools/Data%20Transfer/ParamDataExportMenu.cs
[smithbox-textexport]: https://github.com/vawser/Smithbox/blob/main/src/Smithbox.Program/Editors/Text%20Editor/GUI/TextExporterModal.cs
[smithbox-nr]: https://github.com/vawser/Smithbox/tree/main/src/Smithbox.Data/Assets/PARAM/NR
[smithbox-releases]: https://github.com/vawser/Smithbox/releases
[def-attach]: https://github.com/vawser/Smithbox/blob/main/src/Smithbox.Data/Assets/PARAM/NR/Defs/AttachEffectParam.xml
[def-antique]: https://github.com/vawser/Smithbox/blob/main/src/Smithbox.Data/Assets/PARAM/NR/Defs/EquipParamAntique.xml
[paramdex]: https://github.com/soulsmods/Paramdex/blob/master/README.md
[paramdex-commits]: https://github.com/soulsmods/Paramdex/commits/master/NR
[sfn]: https://github.com/soulsmods/SoulsFormatsNEXT
[witchy]: https://github.com/ividyon/WitchyBND/blob/main/README.md
[nuxe]: https://github.com/JKAnderson/Nuxe/blob/main/README.md
[gplfaq]: https://www.gnu.org/licenses/gpl-faq.html#GPLOutput
[ip-repo]: https://github.com/ip1259/Elden-Ring-Nightreign-Legal-Relic-Generator
[ip-csv]: https://github.com/ip1259/Elden-Ring-Nightreign-Legal-Relic-Generator/blob/main/AttachEffectParam.csv
[ip-fmg]: https://github.com/ip1259/Elden-Ring-Nightreign-Legal-Relic-Generator/blob/main/en/AttachEffectName.fmg.xml
[slavone]: https://github.com/slavone/nighreign_relic_calculator
[quickref]: https://github.com/xxiixi/NightreignQuickRef
[overlay-notices]: https://github.com/NeuraXmy/nightreign-overlay-helper/blob/main/src/nightreign_overlay/resources/licenses/THIRD-PARTY-NOTICES.md
[wikigg-api]: https://eldenring.wiki.gg/api.php?action=query&meta=siteinfo&siprop=rightsinfo|namespaces|statistics
[wikigg-rc]: https://eldenring.wiki.gg/wiki/Special:RecentChanges?namespace=3000
[fandom-api]: https://eldenring.fandom.com/api.php?action=query&meta=siteinfo&siprop=rightsinfo
[fandom-search]: https://eldenring.fandom.com/wiki/Nightreign_Effects
[fandom-lic]: https://www.fandom.com/Licensing
[fandom-ccp]: https://www.fandom.com/community-creation-policy
[ccbysa3]: https://creativecommons.org/licenses/by-sa/3.0/legalcode
[ccbysa4]: https://creativecommons.org/licenses/by-sa/4.0/legalcode.en
[ccby4]: https://creativecommons.org/licenses/by/4.0/legalcode.en
[feist]: https://www.law.cornell.edu/supremecourt/text/499/340
[fextra]: https://eldenringnightreign.wiki.fextralife.com/Elden+Ring+Nightreign+Wiki
[valnet]: https://www.valnetinc.com/en/terms-of-use
[tos]: https://nightreign.eldenring.jp/termsofservice_en.html
[eula]: https://store.steampowered.com/eula/2622380_eula_0
[fs-guide]: https://www.fromsoftware.jp/jp/guidelines_video.html
[bn-terms]: https://www.bandainamco.co.jp/en/terms/
[pn-103]: https://www.bandainamcoent.com/news/elden-ring-nightreign-patch-notes-version-1-03
[pn-1032]: https://mp1st.com/title-updates-and-patches/elden-ring-nightreign-version-1-03-2-balance-adjustments-update-1-003-002-1-22
[eu-art7]: https://www.legislation.gov.uk/eudr/1996/9/article/7
[eu-art11]: https://www.legislation.gov.uk/eudr/1996/9/article/11
[jp-cr]: https://www.cric.or.jp/english/clj/cl2.html

- Smithbox: [README][smithbox-readme] · [LICENSE (MIT)][smithbox-license] · [CSV export code][smithbox-export] · [text exporter][smithbox-textexport] · [NR param assets][smithbox-nr] · [releases][smithbox-releases] · [AttachEffectParam def][def-attach] · [EquipParamAntique def][def-antique]
- Paramdex: [README][paramdex] · [NR commit history][paramdex-commits]
- Format tools: [SoulsFormatsNEXT (GPL-3.0)][sfn] · [WitchyBND README (GPL-3.0)][witchy] · [Nuxe README (GPL-3.0)][nuxe] · [GPL FAQ: output][gplfaq]
- Community datasets: [ip1259 Legal Relic Generator][ip-repo] · [its CSV][ip-csv] · [its FMG][ip-fmg] · [slavone calculator][slavone] · [NightreignQuickRef][quickref] · [overlay-helper notices][overlay-notices]
- Wikis: [Eldenpedia siteinfo][wikigg-api] · [Eldenpedia NR recent changes][wikigg-rc] · [Fandom siteinfo][fandom-api] · [Fandom Nightreign Effects][fandom-search] · [Fandom licensing][fandom-lic] · [Fandom community creation policy][fandom-ccp] · [Fextralife NR wiki][fextra] · [Valnet Terms of Use][valnet]
- Licences and law: [CC BY-SA 3.0][ccbysa3] · [CC BY-SA 4.0][ccbysa4] · [CC BY 4.0][ccby4] · [Feist v. Rural][feist] · [EU Database Directive Art. 7][eu-art7] · [Art. 11][eu-art11] · [Japan Copyright Act (CRIC translation)][jp-cr]
- Publisher: [Nightreign Terms of Service][tos] · [Nightreign Steam EULA][eula] · [FromSoftware video/image guideline][fs-guide] · [Bandai Namco website terms][bn-terms] · [Patch notes 1.03][pn-103] · [1.03.2 (secondary)][pn-1032]
