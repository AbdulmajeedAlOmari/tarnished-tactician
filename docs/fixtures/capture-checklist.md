# Nightreign UI fixture corpus: capture checklist

This is the checklist for [Capture a Nightreign UI fixture corpus](https://github.com/AbdulmajeedAlOmari/tarnished-tactician/issues/9). It gives the [Detection spike against the fixture corpus](https://github.com/AbdulmajeedAlOmari/tarnished-tactician/issues/10) real frames to test against. The surfaces come from the [itemization research](https://github.com/AbdulmajeedAlOmari/tarnished-tactician/blob/research/nightreign-itemization-and-ui/docs/research/nightreign-itemization-and-ui.md) §4. The open questions come from §6 of the same document.

**Corpus location:** `C:\tt-fixtures` (`/mnt/c/tt-fixtures` from WSL). It stays **outside git**. It contains game art and other players' names, so it is never published.

**Scope for now:** 1920×1080 and 2560×1440, Borderless Windowed, base game, normal expeditions. Deep of Night and *The Forsaken Hollows* surfaces are listed under [Later](#later-when-available). Capture them when you can, then add them to the corpus.

---

## 0. One-time setup

- [ ] **Steam**: Settings → In Game → Screenshots. Set the hotkey to `F12` (the default) and tick **Save an uncompressed copy**. Point it at `C:\tt-fixtures\_inbox`. You'll sort files from `_inbox` after each session.
- [ ] **OBS**:
  - Use a **Game Capture** source on Nightreign.
  - Settings → Video: Base and Output resolution = the game resolution, no downscaling.
  - Settings → Output (Simple): Recording Quality = **Lossless Quality, Tremendously Large File Size**. Recording path = `C:\tt-fixtures\_inbox`.
  - Bind a start/stop recording hotkey.
- [ ] **Quiet the desktop**: turn off Discord, Steam friend and Windows notification pop-ups, except where a hard case below asks for them.
- [ ] **HDR check**: open Windows Settings → System → Display. Note whether a **Use HDR** toggle exists. If it does, leave HDR **off** for every section except §4.
- [ ] Note your Windows **display scale** (e.g. 100%, 125%) from the same page.

## 1. Per session: record settings in `notes.md`

Do this at the start of every play session, and again whenever any of these change:

- [ ] Game version: App Ver. / Regulation Ver., shown on the title screen.
- [ ] In-game Screen Mode (must be **Borderless Windowed**) and Resolution.
- [ ] Any in-game display/HUD options that affect the UI (HUD on/auto, brightness, and any UI or HUD scale setting). If the game has no UI scale option, write "no UI scale option".
- [ ] Windows display scale and HDR on/off.
- [ ] Solo or co-op.

## 2. Surfaces at 1920×1080 (full set)

Aim for the counts given. Take **distinct items** across shots so OCR is tested on different strings, not one string repeated. Screenshots are `F12`. A clip is a 5–20 s OBS recording.

| ✓ | Surface ID | What to capture | Count |
|---|---|---|---|
| [ ] | `dormant-power` | Reward-cache selection screen (from bosses, Trolls, Shifting Earth events). Take **one shot per option highlighted** (cursor on option 1, 2 and 3), so the description panel for each option is captured. Mix in armament, passive-buff, talisman and upgrade options. **Clip**: open the cache and move through all the options. | ≥5 caches, ≥1 clip |
| [ ] | `item-acquired` | The popup right after picking something up: armaments, a talisman and a consumable. **Clip**: from walking up to the item until the popup fades, to time how long it stays. | ≥5 shots, ≥2 clips |
| [ ] | `pickup-prompt` | The in-world action prompt near loot: a chest, a weapon rack, an enemy drop and a merchant. Try to catch the rarity glow on a dropped armament. **Clip**: walk up to loot. | ≥5 shots, ≥1 clip |
| [ ] | `equipment-details` | Equipment menu with an armament's detail panel open. Capture **each rarity** (Common/Uncommon/Rare/Legendary), one with a **hand-icon** passive, one with an **affinity** (e.g. Magic, Fire), and one with a **status ailment**. If the passives need scrolling, take a top shot **and** a bottom shot. Also capture a talisman's detail panel. | ≥8 |
| [ ] | `status-effects` | Status menu → Special Effects & Passive Effects list. Take it early in a run with few effects and late in a run with many (scroll if needed). | ≥3 |
| [ ] | `shop` | Merchant list, plus **Detailed Description** open on an armament, a throwable and an Item Pouch. Include the Spirit Shelter shop if you reach it. | ≥4 |
| [ ] | `talisman-pick` | A 1-of-3 talisman choice: the Castle basement chest or the Scale-Bearing Merchant. Capture it opportunistically. | ≥1 if seen |
| [ ] | `smithing` | Smithing Table menu: the armament list and the confirm step. | ≥2 |
| [ ] | `relic-rites` | Roundtable → Relic Rites: the vessel view with slots filled, the relic list, and a relic detail with 2–3 effects. Include one **`[Nightfarer]`-prefixed** effect, one clash with the **red "!"**, the **dimmed** incompatible effects, filter-by-effect, and the preset list. **Clip**: scroll the whole relic list slowly. This tests whether the inventory can be read by scrolling. | ≥8 shots, ≥1 clip |
| [ ] | `results` | The post-expedition Results/rewards screen: relics received, the sell/favourite prompts, and the passives/special-effects summary. Capture a win **and** a loss if possible. | ≥3 |
| [ ] | `character-select` | Character Selection with the equipped relics shown (effect toggle on and off). | ≥2 |
| [ ] | `expedition` | Expedition screen with the Nightlord portrait and its weakness icon. Use different Nightlords. | ≥3 |
| [ ] | `negative` | Frames with **nothing to detect**: plain HUD while exploring, combat, the map screen, a loading screen, the Roundtable hub. These test for false positives. | ≥6 |

### Hard cases (1080p)

| ✓ | Surface ID | What to capture | Count |
|---|---|---|---|
| [ ] | `hard-combat` | A pickup prompt or item-acquired popup **during combat**, with spell, fire or particle effects nearby. | ≥3 |
| [ ] | `hard-occluded` | A prompt or popup partly covered: a bright background, rain or fog, the night circle's edge, or a boss health bar overlapping. | ≥3 |
| [ ] | `hard-coop` | Co-op: the teammate's "indicated equipment" ping message, teammates' item pings, and a crowded three-player HUD while a cache screen is open. | ≥3 |
| [ ] | `hard-popup` | A Steam or Discord notification overlapping a game surface. Allow this one on purpose. | ≥1 |

## 3. Surfaces at 2560×1440 (layout check)

One instance of each surface is enough. The goal is to see how the UI scales with resolution, not to repeat the 1080p corpus.

- [ ] `dormant-power`, `item-acquired`, `pickup-prompt`, `equipment-details`, `status-effects`, `shop`, `smithing`, `relic-rites`, `results`, `character-select`, `expedition`: at least one each.
- [ ] `negative`: at least two.
- [ ] One clip: `dormant-power` or `relic-rites` scrolling.

## 4. HDR sample (only if your display has HDR)

- [ ] With HDR **on**, at 1080p, take one `dormant-power`, one `relic-rites` and one `negative` shot. Steam may save these tone-mapped or washed out. That's fine: the point is to see what the capture actually contains. If the display has no HDR, write "no HDR display" in `notes.md` and skip this section.

## 5. Open questions to answer while capturing

Write the answers in `notes.md`, citing the file that shows each one:

1. Dormant Power screen: is a description panel visible for the highlighted option? Does it show an armament's skill, affinity and passives **without extra input**?
2. Does the item-acquired popup show the armament's name? How long does it stay on screen? (Time it from the clip.)
3. Is affinity shown as a name prefix, an icon, or only in the stats block?
4. Does the in-world pickup prompt show anything beyond the verb (a name, a rarity colour)?
5. Which string does the English client show for the reward cache: "Dormant Power", "Untold Power", or something else?

## 6. Filing

Sort `_inbox` after each session:

```
C:\tt-fixtures\
  notes.md              per-session settings + answers to §5
  manifest.csv          one row per file
  1080p\<surface-id>\   e.g. 1080p\dormant-power\dormant-power__axe-cache-opt2__01.png
  1440p\<surface-id>\
  hdr\<surface-id>\
  _inbox\               Steam/OBS drop zone; empty it each session
```

File names follow `<surface-id>__<short-variant>__<nn>.<ext>`. Use lowercase with hyphens, and no item names that contain spaces.

`manifest.csv` columns:

```
file,resolution,surface,variant,game_version,hdr,solo_coop,truth
```

`truth` is optional. Fill it in **only** when the frame hides something the image alone can't show. Examples: "occluded text reads *Great Club of the Night*", or "this is the armament I actually picked up". Otherwise leave it blank: ground-truth labels for visible text are transcribed later by the detection spike.

When you're done, say so on the ticket. The agent then runs the inventory, checks the corpus against this list, and closes the ticket.

---

## Later (when available)

These are **in v1 scope**. They're missing only because the player doesn't have access yet. They aren't needed to close the ticket, but detection for these surfaces stays unvalidated until they're captured. Add them to the same corpus once you have access:

- **Deep of Night**: Depth Relics and deep slots in Relic Rites, the curse lines on armaments, the DoN Expedition screen (target hidden).
- **The Forsaken Hollows**: DLC Nightfarers in Character Selection and Relic Rites, DLC vessels, Blacksmith Village skill reforge.
- **More resolutions**: 3840×2160 or ultrawide, if v1 later supports them.
