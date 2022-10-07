/* TODO:
* Use ttl value to specify refresh cycle
*? Move tests to different file (needed?)
* Lookup how to implement tests in build.rs
*? possibly download new loc.json altogether at build time
* refactor use %str instead of String, explore zero-cost copy from serde
* move good to it's own file
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
*+ try de/serializing vector to hashset
*+ refactor types to snake_case #[serde(rename_all = "camelCase")]
*+ Possibly Parse Characters (why not lmao?)
*- think through logic for 1 item on 1 char (possibly not needed, GO can do dedup by itself)
 */

#[macro_use]
extern crate lazy_static;

mod types;

use types::{
    EnkaPlayer,
    EquipVariant::{Artifact,Weapon},
    GoodType,
    GoodArtifact,
    GoodSubstat,
    GoodWeapon,
    GoodCharacter,
    GoodTalents,
    CharData,
};
use std::collections::HashMap;
use clap::Parser;

#[derive(Parser)]
#[command(name = "Enka Artifact Parser")]
#[command(author = "skrit <skrityx@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "Parses your ENKA profile artifacts to GOOD format.",
    long_about = None)]
struct Args {
   /// Your account UID
   uid: String,
}

lazy_static! {
    static ref LOCALE: HashMap<String, String> =
        serde_cbor::from_slice(include_bytes!("loc.cbor"))
        .unwrap();
    static ref CHARACTERS: HashMap<String, CharData> =
        serde_cbor::from_slice(include_bytes!("characters.cbor"))
        .unwrap();
    static ref ENKA: HashMap<String, String> =
        serde_cbor::from_slice(include_bytes!("enka.cbor"))
        .unwrap();
    static ref ARGS: Args = Args::parse();
}

fn main() {
    loop {
        match pull_file(ARGS.uid.to_string()) {
            Ok(player) => {
                let ttl = parse_data(player).unwrap_or(120)+1;
                println!("Sleeping for {} seconds.",ttl);
                std::thread::sleep(std::time::Duration::from_secs(ttl.into()));
            },
            Err(e) =>
                panic!("Error parsing Enka response, check your UID. {:?}",e),
        }
    }
}

// Most of the business logic is here
fn parse_data(enka: EnkaPlayer) -> Result<u8, anyhow::Error> {
    let filename: String = format!("{}-{}.json", enka.player_info.nickname, enka.uid);
    println!("Found account: {}, using file: {}.", enka.player_info.nickname, filename);
    let mut data: GoodType;
    if std::path::Path::new(&filename).exists() {
        println!("Found existing file {}, trying to append.",filename);
        println!("This can go wrong if program version changed.");
        data = GoodType::from_file(filename.clone()).unwrap();
    } else {
        println!("File {} not found, creating one.",filename);
        data = GoodType::new();
    }
    for character in enka.avatar_info_list {
        let char_id = character.avatar_id.to_string();
        let talents_id = if "10000005" == char_id { // Workaround for Traveler
            format!("{}-{}",char_id.to_owned(),character.skill_depot_id)
        } else {
            char_id.to_owned()
        };
        print!("Found character {}:",CHARACTERS[&char_id].good_name);
        data.characters.insert(GoodCharacter {
            key: CHARACTERS[&char_id].good_name.to_owned(),
            level: character.prop_map.level.val.parse::<u8>().unwrap(),
            constellation: character.talent_id_list.unwrap_or(vec![]).len() as u8,
            ascension: character.prop_map.ascension.val.parse::<u8>().unwrap(),
            talent: GoodTalents {
                auto: character.skill_level_map[&CHARACTERS[&talents_id].skill_order.0.to_string()],
                skill: character.skill_level_map[&CHARACTERS[&talents_id].skill_order.1.to_string()],
                burst: character.skill_level_map[&CHARACTERS[&talents_id].skill_order.2.to_string()],
            },
        });
        for item in character.equip_list {
            match item {
                Artifact {reliquary, flat} => {
                    print!(" {},",LOCALE[&flat.set_name_text_map_hash]);
                    let mut good_artifact = GoodArtifact {
                        set_key: LOCALE[&flat.set_name_text_map_hash].to_owned(),
                        slot_key: ENKA[&flat.equip_type].to_owned(),
                        level: reliquary.level-1,
                        rarity: flat.rank_level,
                        main_stat_key: ENKA[&flat.reliquary_mainstat.main_prop_id].to_owned(),
                        location: CHARACTERS[&char_id].good_name.to_owned(),
                        substats: vec![],
                    };
                    for substat in flat.reliquary_substats {
                        good_artifact.substats.push(
                            GoodSubstat {
                                key: ENKA[&substat.append_prop_id].to_owned(),
                                value: substat.stat_value,
                            }
                        );
                    };
                    data.artifacts.insert(good_artifact);
                },
                Weapon {item_id, weapon, flat} => {
                    print!(" {}.",LOCALE[&flat.name_text_map_hash]);
                    let good_weapon = GoodWeapon {
                        key: LOCALE[&flat.name_text_map_hash].to_owned(),
                        level: weapon.level,
                        ascension: weapon.promote_level,
                        refinement: weapon.affix_map[&(item_id+100000).to_string()]+1, //flatten this
                        location: CHARACTERS[&char_id].good_name.to_owned(),
                    };
                    data.weapons.insert(good_weapon);
                },
            }
        }
        println!();
    };
    data.to_file(filename).unwrap();
    Ok(enka.ttl)
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
        assert_eq!(CHARACTERS["10000002"].good_name, "KamisatoAyaka");
        assert_eq!(CHARACTERS["10000005"].good_name, "Traveler");
        assert_eq!(CHARACTERS["10000053"].good_name, "Sayu");
        assert_eq!(CHARACTERS["10000054"].good_name, "SangonomiyaKokomi");
    }
    #[test]
    fn test_enka() {
        assert_eq!(ENKA["FIGHT_PROP_HP_PERCENT"],       "hp_");
        assert_eq!(ENKA["FIGHT_PROP_ROCK_ADD_HURT"],    "geo_dmg_");
        assert_eq!(ENKA["EQUIP_DRESS"],                 "circlet");
        assert_eq!(ENKA["EQUIP_BRACER"],                "flower");
    }
}