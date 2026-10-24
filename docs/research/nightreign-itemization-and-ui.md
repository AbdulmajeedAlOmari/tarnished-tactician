# Nightreign itemization model and on-screen UI surfaces

Research for [#5](https://github.com/AbdulmajeedAlOmari/tarnished-tactician/issues/5). Researched 2026-10-24.

**What this covers:** the concepts and relationships Tarnished Tactician has to model, the player choices it could recommend on, and where each choice shows up on screen. It is not an item catalogue.

**Confidence tags:**
- **[official]**: Bandai Namco / FromSoftware patch notes, news, the starter manual, or the Bandai-hosted Future Press *Official Starter Guide*.
- **[wiki]**: community wikis or guides, generally reliable.
- **[unverified]**: sources conflict or are thin. Confirm against captured frames.

Sources are listed at the end and cited as `[S#]` (official) or `[W#]` (community).

---

## 1. Current live patch

**Live: App Ver. 1.03.2 / Regulation Ver. 1.03.5**, set by the hotfix of **31 March 2026** [S1] [official]. That hotfix changed regulation only, fixing a multiplayer performance bug. The last client patch, 1.03.2 (regulation 1.03.4), shipped on 15 January 2026 [S3].

Two checks found nothing newer:
- Steam's official announcement feed for app 2622380, pulled on 2026-10-24, has no patch after 1.03.2. Its newest post is a vinyl-merch item dated 2026-04-01 [S2].
- Bandai Namco EU's Nightreign news list ends at hotfix 1.03.5 [S16]. Fextralife's patch page was last edited 7 Sep 2026 and also stops at 1.03.5 [W14].

So the live data has been stable for about seven months. Wikis (unofficial) report rumours of "one or two final updates", so the app should still expect more patches.

### Patch timeline (itemization-relevant highlights)

| Date | Version (App / Reg) | Itemization / UI relevance |
|---|---|---|
| 2025-05-29/30 | 1.01 (launch) | Base game: 8 Nightfarers, relics, vessels, Dormant Power [S11][S13] |
| 2025-06 → 07 | 1.01.1–1.01.5, reg 1.01.6 | Balance and bug fixes [S2] |
| 2025-07-31 | 1.02 / 1.02 | Duo expeditions. Relic Rites **effect filter** (press L3 or `4`), **100 presets**, relics can be **sold/favourited** from the rewards and purchase menus, **"Detailed Description"** in the Limveld shop menu (R3 or `3`), passive and special effects added to the Results menu [S6] |
| 2025-08-12 | 1.02.1 | Balance [W14] |
| 2025-09-10/11 | 1.02.2 + **Deep of Night** mode | DoN adds multi-effect weapons with detrimental effects, **Depth(s) Relics**, and **3 extra deep slots** [S10]. 1.02.2 caps the stacking of two weapon passives and fixes the rarity display of Dormant Power weapons [S7] |
| 2025-09-24 | 1.02.3 | Depth-relic balance. Fixed the "icon color for detrimental passive effects" [S8] |
| 2025-10-09 | 1.02.4 | Fixes to relic display in other players' status menus [S9] |
| 2025-12-03 | 1.03 / 1.03.1 + **The Forsaken Hollows** DLC | Scholar and Undertaker, the Great Hollow Shifting Earth, 2 new Nightlords [S15]. Equipped relics shown in **Character Selection**, **named presets**, relics shown in the **Results** menu. New weapon passives (Improved Melee Attack Power, Dash Attacks, Rolling Attacks). Broad passive and relic retune. Dormant Power selection screen now keeps the HUD visible, plus a rarity-UI fix [S4] |
| 2025-12-17 | 1.03.1 / 1.03.2 | DLC content in DoN. Icons and explanations for active special effects in the **Status** menu. The pin message now shows the item's rarity colour [S5] |
| 2025-12-24 | hotfix 1.03.1 / reg 1.03.3 | Fixes [W14] |
| 2026-01-15 | 1.03.2 / 1.03.4 | Nightfarer, relic and talisman balance. Shop purchase caps on Fire Pots and Throwing Knives. Status-menu icons for more effects [S3] |
| 2026-03-31 | (1.03.2) / **1.03.5** | Multiplayer performance fix [S1] |

**Version source:** the game prints its version at the lower right of the title screen ("App Ver. x / Regulation Ver. y") [S1][S4]. On consoles, a regulation-only update arrives when the player logs in to the server [S1]. Data packs should therefore be keyed on **regulation** version, not only app version.

---

## 2. Entity model

### 2.1 Structure at a glance

```
Nightfarer ──has── fixed attribute growth (letter grades), passive ability, character skill, ultimate art, starting armaments
    │
    ├── Between runs (persistent, Roundtable Hold)
    │     Vessel (per-Nightfarer, or a universal Grail)
    │       ├── 3 standard slots, each coloured red / blue / yellow / green / white(any)
    │       └── 3 deep slots (unlocked after the first Deep of Night run; colours mirror the standard slots)
    │     Relic (persistent inventory item)
    │       ├── colour (Burning=red, Drizzly=blue, Luminous=yellow, Tranquil=green)
    │       ├── size: Delicate(1 effect) / Polished(2) / Grand(3); or a fixed-effect unique relic
    │       ├── kind: normal | Deep (Depth relic: active only in Deep of Night, effects paired with drawbacks)
    │       └── RelicEffect[] (may be Nightfarer-restricted, e.g. "[Wylder] …"; some modify starting weapon affinity/status/skill)
    │     Preset = Nightfarer + Vessel + relic assignment (named, up to 100)
    │
    └── In a run (Expedition: Day 1, Night 1 boss, Day 2, Night 2 boss, Spirit Shelter, Nightlord)
          Level 1–15 (runes spent at a Site of Grace; stats rise automatically per Nightfarer)
          Armaments: 3 right-hand + 3 left-hand slots
            Armament = base weapon (type) + rarity + affinity? + status ailment? + skill + passive(s) [+ curse in DoN]
          Talismans: 2 slots
          Consumables: 4 item slots (+ Item Pouches), per-item stack caps
          Dormant Powers / special effects: run-scoped buffs picked from reward caches
          Flask charges (churches), Wending Grace (solo revive)
```

### 2.2 Nightfarers

- **Roster:** 10. Wylder, Guardian, Ironeye, Duchess, Raider, Revenant, Recluse and Executor (base) [S13], plus Scholar and Undertaker (*The Forsaken Hollows*) [S15][S17].
- **Kit:** each has a passive ability, a character skill, an ultimate art and starting armaments [W6] [wiki].
- **Stats:** fixed growth curves. Levelling up raises attributes **automatically**, so there is no allocation choice [S12][S13] [official].
  - Each Nightfarer has a per-attribute letter grade (S–D) across Vigor, Mind, Endurance, Strength, Dexterity, Intelligence, Faith and Arcane. For example, Wylder is A/C/B/A/B/C/C/C [W6] [wiki].
  - The grades decide weapon fit. Any Nightfarer can wield any armament, but "some characters will inherently utilize specific weapons more effectively … if their stat distribution is compatible with the armament's damage scaling" [S13] [official].
- **Patch sensitivity:** patches change Nightfarer stat gains, e.g. 1.02.2 "Increased Strength and Dexterity gain when leveling up" for Guardian [S7]. Growth tables are therefore **reference data**.

**App model:** `Nightfarer { id, attributeGrades, growthTable(level→stats), passive, skill, ultimate, startingArmaments }`. A Build references one Nightfarer.

### 2.3 Levels and runes

- **Range and cost:** levels run 1–15 [W15]. Runes are spent at a Site of Grace [S12]. Runes from kills are shared by the whole team [S11][S13].
- **Death:** dying costs one level. The runes can be recovered where you died [S12][S13].
- **Rarity gates:** Uncommon armaments need level 3, Rare need 7, and Legendary need 10. Below that level they deal "drastically reduced damage" [S13] [official].
- **Choices:** levelling itself is not a choice. The *when* matters, though, because of rarity gates and shop spending. The Scale-Bearing Merchant trades levels for a weapon ("I desire a powerful weapon") [S6] [official, mentioned in a bug fix].

### 2.4 Armaments (weapons, shields, staves, seals)

**Rolled properties [S13] [official]:**
- Field-generated armaments roll a random **rarity (upgrade level)**, **skill**, **affinity**, **status ailment** and **passive effects**. "Certain pieces also possess fixed properties."
- Rarity tiers are Common, Uncommon, Rare and Legendary, "indicated by a colored border surrounding its icon".
  - [wiki]: white, blue, purple, gold [W3].
- "Virtually all equipment found in the field possesses at least one passive effect which you'll benefit from as long as the armament is in your possession."
- A subgroup, mostly Rare and Legendary, has **unique Skills which never change**. Two otherwise identical armaments with the same fixed skill can still roll different passives.
- **Affinities:** Fire, Magic, Lightning, Holy. **Status ailments:** Poison, Scarlet Rot, Frostbite, Blood Loss, Sleep, Madness, Death Blight.
- There is no weight limit or stat requirement. The only restriction is the rarity level gate.

**Legendary and unique armaments [W3] [wiki]:**
- Legendary armaments carry a fixed unique passive plus a random passive.
- Staves and seals have a fixed light-attack spell plus a random sorcery or incantation.

**Smithing:**
- Smithing Stones raise rarity one step at the Smithing Table next to every merchant [S12][S13] [official].
- Upgrading improves damage or guard boost, **not passives** [W3] [wiki].
- Stone tiers [W12] [wiki]: Stone [1] takes Common to Uncommon, and Stone [2] takes Uncommon to Rare. Legendary needs a special table.

**Passive application [W10][W11] [wiki]:**
- Passives apply while the armament sits in any of the **6 armament slots**, wielded or not.
- The exception is passives marked with a **hand icon**, which work only while that weapon is in active use. These are typically on-hit HP or FP restoration and "Add Magic to Weapon"-type effects.
- Whether passives on carried-but-unslotted weapons apply is **[unverified]** because sources conflict.

**Stacking:** not uniform. 1.02.2 "Set a cap on the cumulative effects of the Power of Vengeance and Power of House Marais passive effects" [S7] [official]. Stacking must therefore be a **per-effect reference-data field**, not a code rule.

**Deep of Night armaments:**
- Officially, DoN weapons have "multiple additional effects" that "also come with detrimental effects … check before looting" [S10] [official].
- [wiki]: red-marked Unique Armaments need level 7 and "always come with two passives and a curse" [W3].
- [wiki]: curses include reduced HP, FP or Stamina, or lowered damage negation after repeated evasion [W3].

**Other sources:**
- Phantom armaments: red death-phantoms of other players drop their armaments, which bear unique passive traits that depend on which Nightfarer carried them and grow stronger on repeated deaths [S13] [official].
- Skill reforge (DLC): Blacksmith Villages appear on Balancers/Dreglord expeditions when the Great Hollow isn't active. Their smithing table, once fed by a forge, lets you **change an armament's skill** [W13] [wiki, single source]. Patch 1.03.1 refers to a "Combat Art workbench" assigning skills [S5] [official mention].

**App model:**
```
ArmamentBase { id, weaponType, scaling, fixedSkill?, fixedPassives[], uniqueFlag }
ArmamentInstance { baseId, rarity, affinity?, statusAilment?, skillId, passiveIds[](+tier), curseIds[], deepFlag }
```

### 2.5 Passive effects, special effects, Dormant Powers: terminology

The game uses overlapping terms, and the app needs one internal vocabulary.

- **Passive Effect:** an effect carried by an armament, or granted by a Dormant Power reward. Named effects are often tiered, e.g. "Improved Skill Attack Power (+1, +2)" [S8] [official].
- **Special Effect:** a run-state buff, for example from Shifting Earth events ("Favor of the Great Hollow") or raid events [S3].
  - The Status menu lists "Special Effects and Passive Effects" (R3) [S12] [official].
  - Since 1.03.1 and 1.03.2, the Status menu shows icons and descriptions for more of these while active [S5][S3].
- **Relic Effect:** an effect on a relic. Many names match passive effects ("Improved Critical Hits", "Physical Attack Up") but have separate values [S4].
- **Detrimental effects / curses:** these have a distinct icon colour [S8] [official].
- **Dormant Power:** the official guide calls the boss-drop reward cache "Dormant power": "All players can choose one weapon or special effect from a selection" [S11] [official].
  - Wikis also use "Dormant Power" for the *passive buffs* inside the cache [W8][W9].
  - Patch notes also mention "Sorcerer's Rise **Untold Power**" and a DoN "**latent power**" [S8][S3]. Treat these as localisation variants of the reward cache. **[unverified]**: confirm the on-screen string in the English client.

### 2.6 Reward caches (Dormant Power) and other in-run rewards

**Official description [S13]:**
- Caches drop from bosses, from certain strong enemies (e.g. Trolls) and from completed Shifting Earth events.
- On the field they appear as "a roughly player-sized wisp of colored haze marked with [an] icon".
- Interacting with one prompts you "to select from up to three different rewards".
- Contents are rolled **per player**, and "your choice has no impact on the remaining options available to your teammates". This suits a local-player-only scope.
- Caches "typically offer two or three choices" and may contain pre-upgraded armaments or run-long passive buffs.

**By source [wiki]:**
- Strong field bosses give 2 weapons and 1 passive [W8 family].
- Night 2 boss rewards are 3 passive Dormant Powers. The third slot has a 5% chance to be a Wending Grace instead [W8].
- Condemned (Nightfarer) enemies give that Nightfarer's +2 starting weapon with a random passive and skill [W8 family].
- Categories are Armaments, Upgrades, Talismans and Wending Graces [W9].

**Tier colours [W9] [wiki]:** blue, purple, gold. The official 1.03 notes confirm rarity UI on "equipment and item icons" in the selection screen [S4].

**Talismans:**
- Two slots [W7] [wiki].
- [official]: talismans come only from Teardrop Scarabs [S13] (launch-era).
- [wiki]: also the Castle basement chest (pick 1 of 3), Sorcerer's Rise caches, the Township merchant, and the Scale-Bearing Merchant [W7]. Treat the official "only" claim as outdated or simplified.

**Shops (ghostly merchants):**
- Stock has a static part (Item Pouches, Smithing Stones) and armaments "rerolled at the beginning of each mission [that] tend to possess attributes which target the weak point(s) of the current Expedition boss" [S13] [official].
- Stock is per player. Rare items deplete across all merchants.
- Since 1.02, the shop menu has a **Detailed Description** view [S6]. Since 1.03.2, Fire Pots and Throwing Knives have purchase caps [S3].

**Consumables:**
- Four item slots, plus Item Pouches. Per-slot caps, e.g. 1 Stonesword Key, 2 Starlight Shards, 4 Kukri, 8 Throwing Knives [S13] [official].
- Items have rarities but no level gate.
- **Worth recommending:** weakness-targeting throwables (e.g. Fire Pots vs a fire-weak Nightlord), Item Pouches, Smithing Stones, Stonesword Keys (Evergaols) and Wending Grace (solo).

**Other run upgrades:**
- Church altars add a flask charge [S13].
- Nightlord weaknesses show on the Expedition screen (an elemental icon at the upper left of the Nightlord portrait). The map's Detailed View shows armament types and affinities at points of interest [S13] [official].
- In Deep of Night the target Nightlord is hidden until the expedition begins, and at Depth 3+ even until the final battle [S10][W5].

### 2.7 Relics

**Basics [S11][S12][S13] [official]:**
- Relics are persistent and equipped in **Relic Rites** at the Roundtable Hold.
- They "come with up to three random effects".
- They apply from the start of each expedition and are never consumed.

**Size [S14] [official]:**
- **Delicate** = 1 effect, **Polished** = 2, **Grand** = 3.
- "Colors mainly affect which chalices can use them, although some colors seem to have a greater affinity for some types of stat bonuses."

**Colour [W4] [wiki]:**
- Red = Burning, blue = Drizzly, yellow = Luminous, green = Tranquil.
- Random relics are named e.g. "Grand Burning Scene" or "Deep Delicate Tranquil Scene", so the **name encodes size, colour and deep-ness**.
- Fixed unique relics have bespoke names, e.g. "The Will of the Balancers" [S3].

**Character-restricted effects [S13][S3] [official]:**
- Prefixed with the Nightfarer in brackets, e.g. "[Wylder] Art activation spreads fire in area".
- On another Nightfarer "these effects are greyed out and will not activate".

**Starting-weapon effects [S13][S14] [official]:** some effects grant your starting armament an affinity or status ailment, or replace its skill.

**Generation and stacking rules [W1][W4] [wiki]:**
- A relic never holds the same effect twice.
- A relic holds at most one each of: character-exclusive effect, starting-weapon affinity effect, starting-weapon skill effect, Attack Power effect, and "Improved [spell type]" effect.
- These generation rules don't apply to fixed relics.
- Across relics: identical effects at the same strength don't stack, but different strengths (+1/+2) can.
- When effects clash, the **left-most relic wins**. Clashing effects get a **red exclamation mark**, and incompatible effects are dimmed.
- One unique case [official]: multiple copies of "The Will of the Balancers" activate only once [S3].

**Acquisition:**
- Expedition rewards: the first clear per Nightlord gives a unique relic, later clears give random relics, and you get Murk either way [S13].
- Small Jar Bazaar: Scenic Flatstones (a random relic) and fixed "scene" relics, bought with Murk [S13][S14].
- Remembrance quests [S14].
- Collector Signboard: Sovereign Sigils from Everdark Sovereigns [W1].
- The "Deep Large Scenic Flatstone" was added to the Collector Signboard in 1.03 [S4].

**Inventory [official]:** relics can be sold or favourited (1.02) [S6]. A large but finite inventory cap applies: 1960, per [W1] [unverified].

**Depth relics [S10] [official]:**
- Obtained by completing Deep of Night expeditions.
- "Can only be activated during Deep of Night expeditions and feature new powers and detrimental effects not found in existing relics."
- They go in "three newly unlocked relic slots".
- [wiki]: deep slots mirror each vessel's standard slot colours [W2]. Effects are stronger versions of existing ones offset by negatives, e.g. a grease-triggered attack boost with lowered Faith and Dexterity [W5].

**App model:**
```
Relic { id?, name, colour, size, deep, fixed, effects: [{effectId, tier}], curses: [{effectId}] }
RelicEffect { id, text_en, category, nightfarer?, tierFamily, stacksAcrossRelics, mode: normal|deep, isCurse }
```

### 2.8 Vessels and presets

**Vessels [S13] [official]:** "Relic Rites are vessels with a series of slots in which you may equip Relics of the corresponding color; certain advanced [vessels] also sport uncolored slots which will accommodate any type of Relic." Vessels are bought at the Small Jar Bazaar or unlocked through progression.

**Per-Nightfarer vessels [W2] [wiki]:** 7 each.
- Urn (starter) and Goblet (Murk).
- Chalice (Remembrance quest; includes a white slot).
- Soot-Covered Urn, Sealed Urn, Decrepit Goblet and Forgotten Goblet (Sovereign Sigils; the last two are DLC).
- All have 3 standard + 3 deep slots.

**Universal Grails [W2] [wiki]:** 4 shared vessels (Sacred Erdtree, Spirit Shelter, Giant's Cradle, Scadutree), each with slots of a single colour.

**Presets [S6][S4] [official]:** up to 100, named since 1.03, reorderable, selectable at Character Selection. Equipped relics are viewable from Character Selection since 1.03 [S4].

**App model:** `Vessel { id, nightfarer | universal, slots[3]: colour, deepSlots[3]: colour }`. A Build's relic loadout is `{ vesselId, standard[3], deep[3] }`. Deep slots only matter for Deep of Night Builds.

### 2.9 Relationships the app must model

1. **Build → Nightfarer** (1). Relic-effect validity, weapon fit and the vessel list all depend on it.
2. **Vessel → slot colours → Relic colour** compatibility, with white as a wildcard. Deep slots accept only deep relics.
3. **Relic → RelicEffect[]**, with character restriction, stacking and clash rules (left-most wins), and mode restriction (deep relics only work in DoN).
4. **Relic effect → starting armament** (affinity, status or skill change). This changes what the run "needs" from pickups.
5. **ArmamentInstance → passives** (slot-wide unless hand-marked), **rarity → level gate**, and **affinity/status vs the Nightlord's weakness**.
6. **Run state** = level, 6 armament slots, 2 talismans, consumables, accumulated Dormant Powers and special effects, the target Nightlord (unknown in DoN), and the mode (normal or DoN depth).
7. **Effect stacking across sources** (weapon passives + relic effects + Dormant Powers + talismans) is per effect, and must be reference data.
8. Everything numeric, plus availability (which passives exist and which skills can roll), is **patch-versioned**.

---

## 3. Player choices the app could recommend on

| # | Choice | When | Recommendable? |
|---|---|---|---|
| C1 | Nightfarer for this Build | pre-run | Fixed by the Build. Not a live recommendation. |
| C2 | **Vessel + relic assignment** (incl. deep slots) | between runs, Relic Rites | **Yes.** This is the core between-run optimisation: a constrained assignment (colours, clashes, character restrictions). It needs the player's relic inventory. |
| C3 | Relic keep/sell/favourite | after a run (rewards screen), Relic Rites | Yes, optionally. "Is this new relic useful to any of my Builds?" |
| C4 | Scenic Flatstone / vessel purchases (Murk, Sigils) | between runs | Low priority. |
| C5 | Expedition target / normal vs DoN | pre-run | Mostly out of scope. The weakness feeds C7. |
| C6 | **Reward cache pick** (1 of 2–3: armament / passive buff / talisman / upgrade / Wending Grace) | in run, after bosses and events | **Yes.** This is the highest-value in-run decision: discrete, time-boxed, and fully on screen. |
| C7 | **Pick up / keep / drop an armament** (chests, racks, drops, phantoms) and **which 6 to slot** | in run | **Yes.** Judged on weapon type vs Build, affinity/status vs Nightlord, passives, and (in DoN) curses. |
| C8 | Talisman pick (Castle chest 1-of-3, Scale-Bearing Merchant 1-of-3) and which 2 to wear | in run | Yes |
| C9 | **Shop purchases** (armaments, throwables, pouches, stones, Wending Grace) | in run, merchants, Spirit Shelter | Yes |
| C10 | Which armament to smith (stone use) | in run, smithing table | Yes. Simple rule: upgrade the main weapon to the next rarity within the level gate. |
| C11 | Skill reforge (DLC Blacksmith Village) | in run | Yes, but rare. |
| C12 | Scale-Bearing Merchant deals (levels for a weapon, etc.) | in run | Yes, but rare. |
| C13 | Level-up | in run, Site of Grace | **No stat choice** (automatic). At most a "you can afford N levels" nudge. |
| C14 | Consumable loadout (which to carry when slots are full) | in run | Low |

---

## 4. Per-choice UI surfaces

Legend: **OCR-friendly** means the surface shows English text that identifies the item and its effects. **Partial** means an icon or name only, so a second surface or a lookup is needed.

| Choice | Surface | What's shown (evidence) | Enough to identify item + passives? |
|---|---|---|---|
| C6 reward cache | **Dormant Power selection screen** (full-screen menu after interacting with the haze) | Up to 3 options with icons and a rarity indicator on "equipment and item icons" [S4]. Since 1.03 the HUD stays visible [S4]. Wikis list each option by name and strength (e.g. "Improved Physical Attack Power +9%") [W8]. | **Likely yes** [unverified layout]. Option names plus a description panel for the highlighted option, as in standard FromSoft menus. Effect strength is in the text ("+1/+2" or %). The capture spike must confirm whether passives are visible for armament options without extra input. **Best single surface.** |
| C7 pickup | **Action prompt** near loot ("An action prompt will be displayed when approaching such items" [S12]) | The prompt verb ("Pick up", "Examine", "Open") [S12][S13]. A rarity-coloured loot glow may also show [unverified, no source found]. | **No.** The prompt doesn't show passives. Officially, DoN players are told to "check before looting" [S10], but no source describes a pre-pickup detail view, so this is **[unverified]**. |
| C7 after pickup | **Equipment menu, item details panel** ("Displays the details about the currently selected item" [S12]) | Name, icon with rarity border [S13], attack/affinity stats, skill, passives listed under the skill description [community guides via search, unverified], hand icon on use-only passives [W10] | **Yes**, but the player must open the menu. Passives sit below the skill text and may need scrolling **[unverified]**. Good for reconciling run state. |
| C7 / run state | **Status menu → Special Effects & Passive Effects** (R3) [S12] | An aggregated list of active effects. Icons and descriptions added for more effects in 1.03.1/1.03.2 [S5][S3]. Detrimental effects have their own icon colour [S8]. | **Yes, for run state.** One screen summarises all active buffs, including Dormant Powers picked earlier. A strong OCR anchor for correcting the run model. |
| C9 shop | **Shop menu + Detailed Description** (R3 / `3`) [S6] | The item list, plus the full armament description on demand | **Yes**, with the detail view open. Shop stock is per player and rerolled per run [S13]. |
| C8 talisman | Chest / merchant 1-of-3 pick, equipment menu | The selection works like the reward cache [W7]. Talisman names are fixed with no rolled properties. | **Yes.** The name alone is enough (talismans aren't rolled). |
| C10 smithing | Smithing Table menu | Choose an armament, spend a stone [S12] | Yes (an armament list). Low OCR need. |
| C2/C3 relics | **Relic Rites menu** (Roundtable) | The vessel and its coloured slots. The relic list with filter-by-effect [S6]. Effects text with "[Nightfarer]" prefixes, dimmed incompatible effects, red "!" on clashes [W1][W4]. Icons marking placement in other vessels [S4]. Presets by name [S4]. | **Yes.** Static, menu-driven text. A relic's name encodes colour, size and deep-ness [W4], and its effect lines are English strings. Possibly OCR-able as a scrolling inventory import (see implications). |
| C3 after run | **Results / rewards screen** | Sell/favourite relics on the rewards menu [S6]. Results show passives and special effects [S6], each player's relics, and the map layer [S4]. | **Yes.** Good for the post-run log. |
| C1/C2 pre-run | **Character Selection** | Equipped relics with an effect toggle (1.03) [S4], presets [S6] | Yes. Can verify that the active loadout matches the Build. |
| C5 | **Expedition screen** | Nightlord portrait with a weakness icon at the upper left [S13] | Icon only (an affinity glyph). Small, fixed set, so classifiable. Not shown in DoN [S10]. |
| Map | **Map → Detailed View** | Armament types and affinities at points of interest, ranged/magic only [S13] | Icons only. A possible later "route" feature. Out of the v1 choice list. |
| Multiplayer | "You indicated equipment" pin message | Item name in rarity colour (1.03.1) [S5] | Name only. Teammates' items are out of scope. |

**What this means for detection:**
- **Most OCR-friendly** (menus with static English text): Relic Rites, the Status menu effects list, the Results screen, the shop Detailed Description, and the Dormant Power selection screen.
- **Least OCR-friendly:** the in-world pickup prompt, which shows no passives, and icon-only cues (rarity border colour, affinity glyphs, the hand icon, curse icon colour). Icon cues need small template or colour classifiers, not OCR.
- **Pickup flow:** the realistic way to learn a picked-up armament's rolled properties is the item-acquired notification for the *name*, followed by the Equipment-menu detail panel or the Status effects list when the player opens it. "Passive detection" of pickups cannot see passives unless a menu is open.

---

## 5. What varies per patch (belongs in reference data, not code)

- **Effect catalogue:** which passive, special and relic effects exist, with English text, tiers, values, trigger conditions and stacking caps. Examples: 1.03 added three passives and retuned about 15 [S4]; 1.03.2 *renamed* an Executor relic effect [S3]; 1.02.2 added stacking caps [S7]. Effect text can change, so match OCR on **stable effect IDs with per-version string aliases**.
- **Nightfarer data:** growth tables, skills and ultimates (most patches touch these) [S3][S7].
- **Armament pool:** which weapons merchants stock (1.03 "Adjusted the weapon lineup in Limveld's shops" [S4]), skill pools, fixed skills (1.03.2 changed Rotten Crystal Staff's primary sorcery [S3]), talisman values (1.03.2 [S3]).
- **Shop rules:** purchase caps (1.03.2 [S3]) and new stock (Fire Pots in the Spirit Shelter, 1.03 [S4]).
- **Reward-cache tables:** per source and tier [W8][W9]. Chest probabilities also change (1.03.2 [S3]).
- **Vessels and relics:** new vessels and fixed relics with DLC [W2]. New shop goods (Deep Large Scenic Flatstone, 1.03 [S4]).
- **Expedition content:** Nightlords and weaknesses, DoN pools (1.03.2 adjusted DLC appearance rates [S3]).
- **UI changes:** these alter capture templates, e.g. the Dormant Power screen layout (1.03), Status-menu icons (1.03.1, 1.03.2) and the Holy icon redesign (1.02.2 [S7]). **Capture templates and ROIs need versioning too**, not only data.
- **DLC ownership:** DLC Nightfarers, bosses and Shifting Earth exist only for owners, and DoN DLC content appears only when all matched players own it [S5][S3]. Reference data needs a `requiresDlc` flag.

---

## 6. Open questions to confirm with real captures (English PC client)

1. The exact Dormant Power selection layout: is a description panel visible for the highlighted option, and does it show an armament's skill, affinity and passives without extra input?
2. Does the item-acquired popup show a picked-up armament's name, and for how long?
3. Is affinity shown as a name prefix, an icon, or only in the stats block?
4. Does the in-world pickup prompt show anything beyond the verb (name? rarity?), and is there any pre-pickup inspect option in Deep of Night?
5. Which on-screen string does the English client use for the cache ("Dormant Power" vs "Untold Power")?
6. Do weapon passives on unslotted carried armaments apply? (Sources conflict.)

---

## Sources

### Official

- **[S1]** Bandai Namco EU, *Hotfix 1.03.5* (31/03/2026). https://en.bandainamcoent.eu/elden-ring/news/elden-ring-nightreign-hotfix-1035
- **[S2]** Steam official announcements feed for app 2622380 (pulled 2026-10-24 via `ISteamNews/GetNewsForApp`). https://store.steampowered.com/news/app/2622380
- **[S3]** *Patch Notes Version 1.03.2* (15/01/2026). Steam: https://steamstore-a.akamaihd.net/news/externalpost/steam_community_announcements/1821922921815369 · Bandai EU: https://en.bandainamcoent.eu/elden-ring/news/elden-ring-nightreign-patch-notes-version-1032
- **[S4]** *Patch Notes 1.03* (03/12/2025). https://steamstore-a.akamaihd.net/news/externalpost/steam_community_announcements/1818118366169784
- **[S5]** Bandai Namco EU, *Patch Notes Version 1.03.1* (17/12/2025). https://en.bandainamcoent.eu/elden-ring/news/elden-ring-nightreign-patch-notes-version-1031
- **[S6]** *Patch Notes 1.02* (31/07/2025). https://steamstore-a.akamaihd.net/news/externalpost/steam_community_announcements/1806698490676875
- **[S7]** *Patch Notes 1.02.2* (10/09/2025). https://steamstore-a.akamaihd.net/news/externalpost/steam_community_announcements/1810503566383135
- **[S8]** Bandai Namco US, *Patch Notes Version 1.02.3* (24/09/2025). https://www.bandainamcoent.com/news/elden-ring-nightreign-patch-notes-version-1-02-3
- **[S9]** *Patch Notes Version 1.02.4* (09/10/2025). https://steamstore-a.akamaihd.net/news/externalpost/steam_community_announcements/1813041031193476
- **[S10]** *The Deep of Night Explained* (11/09/2025). https://steamstore-a.akamaihd.net/news/externalpost/steam_community_announcements/1810503566416852 (also https://en.bandainamcoent.eu/elden-ring/news/elden-ring-nightreign-the-deep-of-night-explained)
- **[S11]** Bandai Namco, *ELDEN RING NIGHTREIGN – Gameplay Guide* (12/02/2025, pre-launch). https://www.bandainamcoent.com/news/elden-ring-nightreign-gameplay-guide
- **[S12]** Bandai Namco EU, *Starter Manual* (06/02/2025). https://en.bandainamcoent.eu/elden-ring/news/elden-ring-nightreign-starter-manual-ps5
- **[S13]** Future Press, *Official Starter Guide* (PDF, hosted by Bandai Namco EU, page dated 17/06/2025, launch-era content). https://media-center.bandainamcoent.eu/games/elden-ring-nightreign/ERN_FuturePress_StarterGuide.pdf · landing page: https://en.bandainamcoent.eu/elden-ring/news/elden-ring-nightreign-the-official-starter-guide
- **[S14]** Bandai Namco EU, *Advanced Tips* (02/06/2025). https://en.bandainamcoent.eu/elden-ring/news/advanced-tips-elden-ring-nightreign
- **[S15]** Bandai Namco EU, *The Forsaken Hollows DLC launches today* (04/12/2025). https://en.bandainamcoent.eu/elden-ring/news/elden-ring-nightreign-the-forsaken-hollows-dlc-launches-today
- **[S16]** Bandai Namco EU, Nightreign game page news list (accessed 2026-10-24). https://en.bandainamcoent.eu/elden-ring/elden-ring-nightreign
- **[S17]** Bandai Namco EU, *Meet the New Nightfarers, Scholar and Undertaker* (11/12/2025). https://en.bandainamcoent.eu/elden-ring/news/elden-ring-nightreign-meet-the-new-nightfarers-scholar-and-undertaker

### Community (flag confidence)

- **[W1]** Eldenpedia (wiki.gg), *Nightreign: Relics*. https://eldenring.wiki.gg/wiki/Nightreign:Relics
- **[W2]** Eldenpedia, *Nightreign: Vessels / Relic Rites*. https://eldenring.wiki.gg/wiki/Nightreign:Vessels
- **[W3]** Eldenpedia, *Nightreign: Weapons*. https://eldenring.wiki.gg/wiki/Nightreign:Weapons
- **[W4]** Fextralife, *Relics*. https://eldenringnightreign.wiki.fextralife.com/Relics
- **[W5]** Fextralife, *Deep of Night*. https://eldenringnightreign.wiki.fextralife.com/Deep+of+Night
- **[W6]** Fextralife, *Nightfarers* (attribute grade table). https://eldenringnightreign.wiki.fextralife.com/Nightfarers
- **[W7]** Fextralife, *Talismans*. https://eldenringnightreign.wiki.fextralife.com/Talismans
- **[W8]** Fextralife, *Night 2 Boss Reward* (and sibling reward pages, e.g. *Strong Field Boss Reward*). https://eldenringnightreign.wiki.fextralife.com/Night_2_Boss_Reward · https://eldenringnightreign.wiki.fextralife.com/Strong+Field+Boss+Reward
- **[W9]** EIP Gaming, *What Are Dormant Powers?* (May 2025). https://eip.gg/nightreign/guides/dormant-powers/
- **[W10]** Wccftech, *How Passive Skills Work* (hand icon). https://wccftech.com/how-to/elden-ring-nightreign-how-passive-skills-work/amp/
- **[W11]** Icy Veins, *Why Keep Weapons in Each Slot?* https://wp-prod.icy-veins.com/elden-ring-nightreign-why-keep-weapons-in-each-slot/
- **[W12]** Smithing stone tiers: esports.gg https://esports.gg/news/gaming/how-to-get-smithing-stones-and-upgrade-weapons-in-elden-ring-nightreign · Icy Veins https://wp-prod.icy-veins.com/elden-ring-nightreign-best-smithing-stones-guide/
- **[W13]** Filibuster Blog, *Changing Weapon Skill at the Smithing Table* (DLC Blacksmith Village). https://thefilibusterblog.com/changing-weapon-skill-at-the-smithing-table-in-elden-ring-nightreign/
- **[W14]** Patch archives: Eldenpedia https://eldenring.wiki.gg/wiki/Nightreign:Patch_Notes · Fextralife https://eldenringnightreign.wiki.fextralife.com/Patch_Notes
- **[W15]** Fextralife, *Level*. https://eldenringnightreign.wiki.fextralife.com/Level
