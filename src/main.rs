/* TODO:
*+ USE CBOR TO APPEND DATA STRUCTURES AT COMPILE TIME https://www.reddit.com/r/rust/comments/f47h5o/include_json_files_along_with_my_library/
*+ \ref:convert_enka_to_good_lit change to hashmap
* Parse weapon (? needed?)
*+ Parse character to location key of artifact(use the same conversion) https://github.com/Dimbreath/GenshinData/blob/master/ExcelBinOutput/AvatarSkillDepotExcelConfigData.json
* Pull the previous json to append stuff (handle updating same arts and adding new ones)
* Use ttl value to specify refresh cycle
*+ Move everything out of main ( :) )
*+ Parse file from web, allow user to provide UID
*+ Save data to nickname-UID.json
*? Move tests to different file (needed?)
* Make reader_function either a impl on struct or generic for any type
*+ Add Build.rs to update loc.cbor when loc.json is updated
*? possibly download new loc.json altogether
*+ Move types to a different file (?) https://stackoverflow.com/questions/28010796/move-struct-into-a-separate-file-without-splitting-into-a-separate-module
* Move derive_literal to build.rs
*/

#[macro_use]
extern crate lazy_static;

mod types;

use types::{EnkaPlayer,GoodType,GoodArtifact,GoodSubstat};
use std::collections::HashMap;
use clap::Parser;

#[derive(Parser)]
#[command(name = "Enka Artifact Parser")]
#[command(author = "skrit <skrityx@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "Parses your ENKA profile artifacts to GOOD format file: {nickname}-{uid}.json.", long_about = None)]
struct Args {
   /// Your account UID
   uid: String,
}

lazy_static! {
    static ref LOCALE: HashMap<String, String> = serde_cbor::from_slice(include_bytes!("loc.cbor")).unwrap();
    static ref CHARACTERS: HashMap<String, String> = serde_cbor::from_slice(include_bytes!("characters.cbor")).unwrap();
    static ref ENKA: HashMap<String, String> = serde_cbor::from_slice(include_bytes!("enka.cbor")).unwrap();
    static ref ARGS: Args = Args::parse();
}

fn main() {
    let mut output = GoodType::new();
    match pull_file(ARGS.uid.to_string()) {
        Ok(player) => {
            parse_data(player, &mut output);
        },
        Err(e) => panic!("Error parsing Enka response, check your UID. {:?}",e),
    }
}
// Possilbly move to build.rs
fn derive_literal(input: &str) -> String {
    input
        .split(" ")
        .map(|word| format!("{}{}", &word[..1].to_uppercase(), &word[1..]))
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect()
}

fn parse_data(enka: EnkaPlayer, ref mut data:&mut GoodType) {
    let filename: String = format!("{}-{}.json", enka.playerInfo.nickname, enka.uid);
    for character in enka.avatarInfoList {
        for item in character.equipList {
            let flat = item.flat;
            match item.reliquary {
                Some(artifact) => {
                    let mut good_artifact = GoodArtifact {
                        setKey: derive_literal(&LOCALE[&flat.setNameTextMapHash.unwrap()]),
                        slotKey: ENKA[&flat.equipType.unwrap()].to_owned(),
                        level: artifact.level-1,
                        rarity: flat.rankLevel,
                        mainStatKey: ENKA[&flat.reliquaryMainstat.unwrap().mainPropId].to_owned(),
                        location: derive_literal(&CHARACTERS[&character.avatarId.to_string()]),
                        substats: vec![],
                        _id: item.itemId,
                    };
                    for substat in flat.reliquarySubstats.unwrap() {
                        good_artifact.substats.push(
                            GoodSubstat {
                                key: ENKA[&substat.appendPropId].to_owned(),
                                value: substat.statValue,
                            }
                        );
                    };
                    data.artifacts.push(good_artifact);
                },
                None => { //Codepath for weapon if ever needed
                    continue
                },
            }
        }
    };
    data.to_file(filename).unwrap();
}

fn pull_file(uid: String) -> Result<EnkaPlayer, minreq::Error> {
    Ok(minreq::get(format!("https://enka.network/u/{}/__data.json", uid)).send()?.json()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_derive_literal() {
        assert_eq!("GladiatorsFinale", derive_literal("Gladiator's Finale"));
        assert_eq!("SpiritLocketOfBoreas", derive_literal("Spirit Locket of Boreas"));
        assert_eq!("TheCatch", derive_literal("The Catch"));
        assert_eq!("ABDD", derive_literal("'''A b d435 D123/ /'''"));
        // Add test for every non-alphabetical symbol
    }
    #[test]
    fn test_locale() {
        assert_eq!(LOCALE["4238339131"], "Staff of the Scarlet Sands");
        assert_eq!(LOCALE["3914045794"], "Sangonomiya Kokomi");
        assert_eq!(LOCALE["3782508715"], "Traveling Doctor");
        assert_eq!(LOCALE["3600623979"], "Hunter's Bow");
    }
    #[test]
    fn test_characters() {
        assert_eq!(CHARACTERS["10000002"], "Kamisato Ayaka");
        assert_eq!(CHARACTERS["10000005"], "Traveler");
        assert_eq!(CHARACTERS["10000053"], "Sayu");
        assert_eq!(CHARACTERS["10000054"], "Sangonomiya Kokomi");
    }
    #[test]
    fn test_enka() {
        assert_eq!(ENKA["FIGHT_PROP_HP_PERCENT"],       "hp_");
        assert_eq!(ENKA["FIGHT_PROP_ROCK_ADD_HURT"],    "geo_dmg_");
        assert_eq!(ENKA["EQUIP_DRESS"],                 "circlet");
        assert_eq!(ENKA["EQUIP_BRACER"],                "flower");
    }
}