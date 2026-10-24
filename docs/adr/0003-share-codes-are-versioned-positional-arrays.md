# Share codes are versioned positional arrays, decoded forever

A Share code is `TT<n>-` followed by base64url of a deflate-raw positional JSON array plus a CRC32 of the compressed bytes. The array holds `[nightfarerId, name, regulation, attribution, priorities]`, with Priorities grouped by tier in a fixed order and then by kind as `[kindCode, ...values]`. Kind codes are small integers frozen by the format, independent of any Data Pack, while Tag slugs and enum values stay as strings. Codes live on indefinitely in Discord, Reddit and YouTube posts, which is why this is hard to reverse: every app release must keep a decoder for every code version ever published.

## Considered Options

- **Object JSON**: self-describing and tolerant of added fields, but about four times longer (388 vs 94 characters in samples), pushing a full Build past comfortable Discord and X lengths.
- **Positional array with pack-relative indices** for Tags and kinds: shorter still, but any pack that reordered its tables would silently retarget old codes.
- **Positional array with frozen kind codes and string Tags** (chosen): a full 40-Priority Build stays under 600 characters, and nothing in a code depends on the pack that produced it.

## Consequences

- Every layout change bumps the code version. There are no "ignore unknown fields" rules: an older app refuses a newer code with an update message rather than importing part of it.
- Decoders output a `BuildSchema` at some `buildSchemaVersion`, and the normal Build migrations bring it current, so the code version and `buildSchemaVersion` move independently.
- Encoding is canonical (values sorted within each group), so one Build always yields one code.
