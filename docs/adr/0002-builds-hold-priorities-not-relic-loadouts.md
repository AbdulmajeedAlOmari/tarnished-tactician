# Builds hold Priorities, never Relic Loadouts

A Build is a Nightfarer plus a flat list of tiered Priorities (`core` / `want` / `nice` / `avoid`) over Tags, kind-scoped game row IDs and project enums. It never stores a concrete Vessel-and-Relic-per-slot loadout: Relics are rolled per player, so a Build's author cannot name the Relics a given player owns, and the Recommendation engine derives the Relic Loadout from the Build and the player's own Relics instead. Builds, Share codes and Curated Builds all depend on this shape, which is why it is hard to reverse.

## Considered Options

- **Concrete loadout only** (as nightreignbuilds.* share links do): readable, but stale the moment a player owns different rolls, and no help for in-run picks.
- **Both, with an optional per-slot "ideal"**: duplicates what the engine computes from inventory and doubles the editor and Share code surface.
- **Priorities only** (chosen, as relics.pro and Mobalytics describe builds): one model feeds both in-run and between-run Recommendations. Unique Relics (fixed effects) can still be pinned as `relic` Priorities, and preferred Vessels as `vessel` Priorities.

## Consequences

- Transcribing a concrete loadout from an external site means turning its Vessel, unique Relics and relic effects into Priorities.
- Without a known relic inventory, between-run advice can only show which effects to look for, not a ready loadout.
