/* TODO:
* Use ttl value to specify refresh cycle
* think through logic for 1 item on 1 char
* Possibly Parse Characters (why not lmao?)
*? Move tests to different file (needed?)
* Lookup how to implement tests in build.rs
*? possibly download new loc.json altogether at build time
*/
/* DONE
*+ Do derive_literal at build time in build.rs
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
*+ use variants to distignuish weapon and artifact
 */

#[macro_use]
extern crate lazy_static;

mod types;

use types::{EnkaPlayer,GoodType,GoodArtifact,GoodSubstat,GoodWeapon,EquipVariant};
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
    match pull_file(ARGS.uid.to_string()) {
        Ok(player) => {
            parse_data(player).unwrap();
        },
        Err(e) => panic!("Error parsing Enka response, check your UID. {:?}",e),
    }
}

// Most of the business logic is here
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
            match item {
                EquipVariant::Artifact {itemId, reliquary, flat} => {
                    // Test if artifact exists here by iterating over data
                    // Problems to solve: same artifact, same artifact upgraded (+lvl), 2 artifacts of the same type on the same character (?)
                    let mut good_artifact = GoodArtifact {
                        setKey: LOCALE[&flat.setNameTextMapHash].to_owned(),
                        slotKey: ENKA[&flat.equipType].to_owned(),
                        level: reliquary.level-1,
                        rarity: flat.rankLevel,
                        mainStatKey: ENKA[&flat.reliquaryMainstat.mainPropId].to_owned(),
                        location: CHARACTERS[&character.avatarId.to_string()].to_owned(),
                        substats: vec![],
                        _id: itemId,
                    };
                    for substat in flat.reliquarySubstats {
                        good_artifact.substats.push(
                            GoodSubstat {
                                key: ENKA[&substat.appendPropId].to_owned(),
                                value: substat.statValue,
                            }
                        );
                    };
                    data.artifacts.push(good_artifact);
                },
                EquipVariant::Weapon {itemId, weapon, flat} => {
                    let good_weapon = GoodWeapon {
                        key: LOCALE[&flat.nameTextMapHash].to_owned(),
                        level: weapon.level,
                        ascension: weapon.promoteLevel,
                        refinement: weapon.affixMap[&(itemId+100000).to_string()]+1,
                        location: CHARACTERS[&character.avatarId.to_string()].to_owned(),
                        _id: itemId,
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
    fn test_locale() {
        assert_eq!(LOCALE["4238339131"], "StaffOfTheScarletSands");
        assert_eq!(LOCALE["3914045794"], "SangonomiyaKokomi");
        assert_eq!(LOCALE["3782508715"], "TravelingDoctor");
        assert_eq!(LOCALE["3600623979"], "HuntersBow");
    }
    #[test]
    fn test_characters() {
        assert_eq!(CHARACTERS["10000002"], "KamisatoAyaka");
        assert_eq!(CHARACTERS["10000005"], "Traveler");
        assert_eq!(CHARACTERS["10000053"], "Sayu");
        assert_eq!(CHARACTERS["10000054"], "SangonomiyaKokomi");
    }
    #[test]
    fn test_enka() {
        assert_eq!(ENKA["FIGHT_PROP_HP_PERCENT"],       "hp_");
        assert_eq!(ENKA["FIGHT_PROP_ROCK_ADD_HURT"],    "geo_dmg_");
        assert_eq!(ENKA["EQUIP_DRESS"],                 "circlet");
        assert_eq!(ENKA["EQUIP_BRACER"],                "flower");
    }
}