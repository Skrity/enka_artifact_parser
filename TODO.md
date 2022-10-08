# TODO

    This is a TODO, deal with it.

## Main

- [?] Move tests to different file (needed?)
- [ ] Handle ctrl+c
- [ ] Test ENKA types for nullability, context: promoteLevel fix in this commit
- [ ] Use phf instead of static_lazy

## Types

- [?] Change affix map to tuple
- [ ] Refactor: use str instead of String, explore zero-cost copy from serde

## Build

- [ ] Lookup how to implement tests in build.rs
- [?] Possibly download new loc.json altogether at build time

## DONE

- [x] Do derive_literal at build time in build.rs
- [x] Parse weapon
- [x] Use CBOR to append data structures at compile time \
        <https://www.reddit.com/r/rust/comments/f47h5o/include_json_files_along_with_my_library/>
- [x] \ref:convert_enka_to_good_lit change to hashmap
- [x] Parse character to location key of artifact(use the same conversion) \
        <https://github.com/Dimbreath/GenshinData/blob/master/ExcelBinOutput/AvatarSkillDepotExcelConfigData.json>
- [x] Move everything out of main ( :) )
- [x] Parse file from web, allow user to provide UID
- [x] Save data to nickname-UID.json
- [x] Add Build.rs to update loc.cbor when loc.json is updated
- [x] Move types to a different file (?)
        <https://stackoverflow.com/questions/28010796/move-struct-into-a-separate-file-without-splitting-into-a-separate-module>
- [x] Pull the previous json to append stuff (handle updating same arts and adding new ones)
- [x] use variants to distignuish weapon and artifact
- [x] try de/serializing vector to hashset
- [x] refactor types to snake_case #[serde(rename_all = "camelCase")]
- [x] Possibly Parse Characters (why not lmao?)
- [-]  Think through logic for 1 item on 1 char (possibly not needed, GO can do dedup by itself)
- [x] Use ttl value to specify refresh cycle
- [x] Change Hashing for GoodArtifact and GoodWeapon to not consider location
- [x] Create TODO.md
- [x] Move todo from main.rs
- [x] Move GOOD to it's own file
