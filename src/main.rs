/* TODO:
* Use ttl value to specify refresh cycle
*? Move tests to different file (needed?)
* Make reader_function either a impl on struct or generic for any type
*? possibly download new loc.json altogether
* Move derive_literal to build.rs for faster runtime
* think through logic for 1 item on 1 char
*/
/* DONE
*+ Parse weapon
*+ USE CBOR TO APPEND DATA STRUCTURES AT COMPILE TIME https://www.reddit.com/r/rust/comments/f47h5o/include_json_files_along_with_my_library/
*+ \ref:convert_enka_to_good_lit change to hashmap
*+ Parse character to location key of artifact(use the same conversion) https://github.com/Dimbreath/GenshinData/blob/master/ExcelBinOutput/AvatarSkillDepotExcelConfigData.json
*+ Move everything out of main ( :) )
*+ Parse file from web, allow user to provide UID
*+ Save data to nickname-UID.json
*+ Add Build.rs to update loc.cbor when loc.json is updated
*+ Move types to a different file (?) https://stackoverflow.com/questions/28010796/move-struct-into-a-separate-file-without-splitting-into-a-separate-module
*+ Pull the previous json to append stuff (handle updating same arts and adding new ones)
 */

#[macro_use]
extern crate lazy_static;

mod types;

use types::{EnkaPlayer,GoodType,GoodArtifact,GoodSubstat,GoodWeapon};
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
    //let mut output = GoodType::new();
    match pull_file(ARGS.uid.to_string()) {
        Ok(player) => {
            parse_data(player).unwrap();
        },
        Err(e) => panic!("Error parsing Enka response, check your UID. {:?}",e),
    }
}
// Consider moving to build.rs for faster runtime
fn derive_literal(input: String) -> String {
    input
        .split(" ")
        .map(|word| format!("{}{}", &word[..1].to_uppercase(), &word[1..]))
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect()
}

fn parse_data(enka: EnkaPlayer) -> anyhow::Result<()> {
    let filename: String = format!("{}-{}.json", enka.playerInfo.nickname, enka.uid);
    let mut data: GoodType;
    if std::path::Path::new(&filename).exists() {
        // Change when accounted for copy artifacts
        data = GoodType::new();
        //data = GoodType::from_file(filename.clone()).unwrap();
    } else {
        data = GoodType::new();
    }
    for character in enka.avatarInfoList {
        for item in character.equipList {
            let flat = item.flat;
            match item.reliquary {
                Some(artifact) => {
                    // Test if artifact exists here by iterating over data
                    // Problems to solve: same artifact, same artifact upgraded (+lvl), 2 artifacts of the same type on the same character (?)
                    // unwraps here should be covered by the match above
                    let mut good_artifact = GoodArtifact {
                        setKey: derive_literal(LOCALE[&flat.setNameTextMapHash.unwrap()].to_owned()),
                        slotKey: ENKA[&flat.equipType.unwrap()].to_owned(),
                        level: artifact.level-1,
                        rarity: flat.rankLevel,
                        mainStatKey: ENKA[&flat.reliquaryMainstat.unwrap().mainPropId].to_owned(),
                        location: derive_literal(CHARACTERS[&character.avatarId.to_string()].to_owned()),
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
                    let weapon = item.weapon.unwrap();
                    let good_weapon = GoodWeapon {
                        key: derive_literal(LOCALE[&flat.nameTextMapHash].to_owned()),
                        level: weapon.level,
                        ascension: weapon.promoteLevel,
                        refinement: weapon.affixMap[&(item.itemId+100000)]+1,
                        location: derive_literal(CHARACTERS[&character.avatarId.to_string()].to_owned()),
                        _id: item.itemId,
                    };
                    data.weapons.push(good_weapon);
                },
            }
        }
    };
    data.to_file(filename).unwrap();
    Ok(())
}

fn pull_file(uid: String) -> Result<EnkaPlayer, minreq::Error> {
    Ok(minreq::get(format!("https://enka.network/u/{}/__data.json", uid)).send()?.json()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_derive_literal() {
        assert_eq!("GladiatorsFinale", derive_literal("Gladiator's Finale".to_string()));
        assert_eq!("SpiritLocketOfBoreas", derive_literal("Spirit Locket of Boreas".to_string()));
        assert_eq!("TheCatch", derive_literal("The Catch".to_string()));
        assert_eq!("ABDD", derive_literal("'''A b d435 D123/ /'''".to_string()));
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