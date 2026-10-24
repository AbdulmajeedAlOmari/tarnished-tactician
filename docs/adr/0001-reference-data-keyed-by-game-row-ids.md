# Reference data is keyed by the game's own row IDs, scoped by kind

Every entity in a Data Pack (effects, relics, vessels, armaments, talismans, Dormant Powers, Nightfarers, Nightlords, skills) is identified by its kind plus the numeric row ID of its source param table in the game files, not by a project-assigned slug. The kind is part of the identity because IDs repeat across tables (Dormant Power buffs reuse weapon-passive numbers). Concepts with no param row (affinity, status ailment, weapon type, rarity, slot colour, relic colour and size) use small project enums instead. Builds, Share codes and the Run log will hold these IDs indefinitely, which is why this is hard to reverse.

## Considered Options

- **Project slugs** (`improved-critical-hits-relic`): readable, but every pack regeneration would need a hand-maintained mapping from game rows to slugs, and same-named effects (one relic row, three weapon tiers, one talisman row) would need invented disambiguators.
- **Game row IDs, kind-scoped** (chosen): free on every regeneration, match what relics.pro exports and what save-file readers produce, and keep the paste-text name matcher a plain lookup.

## Consequences

- A patch that renumbers rows would silently retarget stored Builds. No renumbering has been observed (changes so far are additive), so the publishing workflow must diff row IDs against the previous pack and fail loudly on a reused or vanished ID.
- The game's text IDs are only used during pack generation to join names. They are never an entity identity.
