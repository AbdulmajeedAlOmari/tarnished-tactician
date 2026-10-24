# Tarnished Tactician

A Windows desktop companion and overlay for *Elden Ring Nightreign* that recommends a player's in-run picks and between-run relic loadout from their chosen build, and explains why each recommendation fits.

## Language

**Build**:
A durable pre-run plan: a Nightfarer, a relic loadout (with its vessel), and priorities for weapon types, passives, talismans and affinities. Curated, user-authored, imported, or decoded from a share code.
_Avoid_: Loadout (for the whole plan), setup, preset

**Run**:
One expedition being played with a Build, and the transient state of what the player currently holds and where they are in it.
_Also_: Expedition (the game's own word; interchangeable with Run)
_Avoid_: Session, match, game

**Recommendation**:
The app's verdict on a candidate pick, derived from a Build and the current Run, always carrying a reason that ties it back to the Build.
_Avoid_: Suggestion, tip, advice

**Share code**:
A copy-pasteable string that encodes one Build so another player can import it without any server.
_Avoid_: Build link, export string

**Data Pack**:
A versioned bundle of game reference data for one Regulation Version, pairing facts extracted from the game with the project's own authored annotations. The app always uses the newest Data Pack it can read.
_Avoid_: Database, data file, patch data

### Game

**Nightfarer**:
One of the game's playable characters; every Build is for exactly one.
_Avoid_: Class, hero, character

**Relic**:
A persistent item carrying one to three relic effects, equipped into a Vessel's slots between expeditions.
_Avoid_: Artifact, rune

**Depth Relic**:
A Relic that only takes effect in Deep of Night, pairing stronger effects with detrimental ones.
_Avoid_: Deep relic, cursed relic

**Vessel**:
The slot layout a Nightfarer equips Relics into: three coloured slots plus three deep slots for Depth Relics.
_Avoid_: Chalice, grail (except as the game's names for specific Vessels)

**Relic Rites**:
The Roundtable Hold menu where Vessels and Relics are equipped.

**Deep of Night**:
The game's harder mode in which Depth Relics and deep slots are active.

**Armament**:
A weapon picked up during an expedition, with a rolled rarity, affinity, status ailment, skill and passive effects.
_Avoid_: Weapon drop, loot

**Passive Effect**:
An effect carried by an Armament or granted by a Dormant Power, often tiered (+1, +2).
_Avoid_: Perk, modifier

**Relic Effect**:
An effect carried by a Relic. It is distinct from a same-named Passive Effect on an Armament and has its own values.
_Avoid_: Relic passive, relic perk

**Curse**:
A detrimental effect attached to some Armaments and Depth Relics.
_Avoid_: Debuff, drawback

**Dormant Power**:
The reward cache after a boss from which each player picks one Armament or special effect.
_Avoid_: Boon, Untold Power

**Rarity**:
An Armament's tier (Common, Uncommon, Rare, Legendary), each gated behind a minimum level.
_Avoid_: Quality, upgrade level

**Nightlord**:
The final boss an expedition targets; its weakness shapes which affinities a Build wants.
_Avoid_: Final boss

**Regulation Version**:
The version of the game's balance and item data, released separately from the game client (consoles can receive it alone); shown as "Regulation Ver." on the title screen.
_Avoid_: Patch version, game version (for the data)
