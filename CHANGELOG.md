# [master](https://github.com/Skrity/enka_artifact_parser)

## Running changes

- changed functions to take &str to avoid excessive copying
- changed skill_order from tuple to array
- download dicts at build time
- covered recoverable errors in match statements
- remove Indexing everywhere in favour of get()
- use phf_map instead of CBOR
- moved GOOD into it's own file, referenced cargo.toml version etc
- Changed Result to Anyhow, errors should be prettier now, should remove those pesky unwraps.
- Implemented Hash function for GoodArtifact and GoodWeapon, now it ignores location \
(same artifact now won't appear twice if seen on a different characters)
- Added TODO.md & CHANGELOG.md to repo
- Fixed a bug with nullable type for weapon ascension stat
- Start of changelog

---
