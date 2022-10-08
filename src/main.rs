#[macro_use]
extern crate lazy_static;

mod types;

use types::{
    EnkaPlayer,
    EquipVariant::{Artifact, Weapon},
    good::{GoodType, GoodArtifact, GoodSubstat, GoodWeapon, GoodCharacter, GoodTalents},
    CharData,
};
use std::collections::HashMap;
use clap::Parser;
use anyhow::{Result,Context};

#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(author = env!("CARGO_PKG_AUTHORS"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"), long_about = None)]
struct Args {
   /// Your account UID
   uid: String,
}

lazy_static! {
    // Unwraps here should succeed, this should be guaranteed by build.rs
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
            Err(err) =>
                panic!("Couldn't properly parse Enka response. Debugging info follows:\n{:?}",err),
        }
    }
}

// Most of the business logic is here
fn parse_data(enka: EnkaPlayer) -> anyhow::Result<u8> {
    let filename: String = format!("{}-{}.json", enka.player_info.nickname, enka.uid);
    println!("Found account: {}, using file: {}.", enka.player_info.nickname, filename);
    let mut data: GoodType;
    if std::path::Path::new(&filename).exists() {
        println!("Found existing file {}, trying to append.",filename);
        println!("This can go wrong if program version changed.");
        data = GoodType::from_file(filename.clone()).context("Error while reading old file.")?;
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
            level: character.prop_map.level.val.parse::<u8>()
                .context("Error while converting character level into u8")?,
            constellation: character.talent_id_list.unwrap_or(vec![]).len() as u8,
            ascension: character.prop_map.ascension.val.parse::<u8>()
                .context("Error while converting character ascension into u8")?,
            talent: GoodTalents {
                auto:
                    character.skill_level_map[&CHARACTERS[&talents_id].skill_order.0.to_string()],
                skill:
                    character.skill_level_map[&CHARACTERS[&talents_id].skill_order.1.to_string()],
                burst:
                    character.skill_level_map[&CHARACTERS[&talents_id].skill_order.2.to_string()],
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
                    data.artifacts.replace(good_artifact);
                },
                Weapon {item_id, weapon, flat} => {
                    print!(" {}.",LOCALE[&flat.name_text_map_hash]);
                    let good_weapon = GoodWeapon {
                        key: LOCALE[&flat.name_text_map_hash].to_owned(),
                        level: weapon.level,
                        ascension: weapon.promote_level.unwrap_or(0),
                        refinement: weapon.affix_map[&(item_id+100000).to_string()]+1,
                        location: CHARACTERS[&char_id].good_name.to_owned(),
                    };
                    data.weapons.replace(good_weapon);
                },
            }
        }
        println!();
    };
    data.to_file(filename)?;
    Ok(enka.ttl)
}

fn pull_file(uid: String) -> Result<EnkaPlayer> {
    Ok(
        minreq::get(format!("https://enka.network/u/{}/__data.json", uid))
            .send().context("Error while doing HTTP GET")?
            .json().context("Error while parsing JSON response")?
    )
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