# [master](https://github.com/Skrity/enka_artifact_parser)

## Running changes

- moved GOOD into it's own file, referenced cargo.toml version etc
- Changed Result to Anyhow, errors should be prettier now, should remove those pesky unwraps.
- Implemented Hash function for GoodArtifact and GoodWeapon, now it ignores location \
(same artifact now won't appear twice if seen on a different characters)
- Added TODO.md & CHANGELOG.md to repo
- Fixed a bug with nullable type for weapon ascension stat
- Start of changelog

---
