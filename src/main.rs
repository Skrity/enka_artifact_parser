mod types;

use types::{
    EnkaPlayer,
    EquipVariant::{Artifact, Weapon},
    good::{
        GoodType,
        GoodArtifact,
        GoodSubstat,
        GoodWeapon,
        GoodCharacter,
        GoodTalents
    },
};
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

// Contains pre-generated maps of data: loc.json, characters.json, enka stuff
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

fn main() {
    loop {
        match pull_file(Args::parse().uid) {
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

// Most of the business logic is here, returns TTL wrapped in Result
fn parse_data(enka: EnkaPlayer) -> Result<u8> {
    let filename: String = format!("{}-{}.json", enka.player_info.nickname, enka.uid);
    println!("Found account: {}, using file: {}.", enka.player_info.nickname, filename);
    let mut data: GoodType;
    if std::path::Path::new(&filename).exists() {
        println!("Found existing file {}, trying to append.", filename);
        println!("This can go wrong if program version changed.");
        data = GoodType::from_file(filename.clone())
            .context("Error while reading old file.")?;
    } else {
        println!("File {} not found, creating one.",filename);
        data = GoodType::new();
    }
    for character in enka.avatar_info_list {
        let char_name = DICT.get(&character.avatar_id.to_string());
        let talent_ids = *SKILLS.get(
            if 10000005 == character.avatar_id { // Workaround for Traveler
                    format!("{}-{}", character.avatar_id, character.skill_depot_id)
                } else {
                    character.avatar_id.to_string()
                }.as_str()
            ).context("Character not found in skill order list.")?;
        match char_name {
            Some(char) => {
                print!("Found character {}:", char);
                data.characters.insert(GoodCharacter {
                    key: char.to_string(),
                    level: character.prop_map.level.val // Assumed nullable
                        .unwrap_or("1".to_string())
                        .parse::<u8>()
                        .context("Error while converting character level into u8")?,
                    constellation: character.talent_id_list // Assumed nullable
                        .unwrap_or(vec![])
                        .len() as u8,
                    ascension: character.prop_map.ascension.val // Assumed nullable
                        .unwrap_or("0".to_string())
                        .parse::<u8>()
                        .context("Error while converting character ascension into u8")?,
                    talent: GoodTalents {
                        auto:  *character.skill_level_map.get(&talent_ids.0.to_string())
                            .unwrap_or(&1),
                        skill: *character.skill_level_map.get(&talent_ids.1.to_string())
                            .unwrap_or(&1),
                        burst: *character.skill_level_map.get(&talent_ids.2.to_string())
                            .unwrap_or(&1),
                    },
                });
            },
            None => print!("Found unknown character:"),
        }
        for item in character.equip_list {
            match item {
                Artifact {reliquary, flat} => {
                    match DICT.get(&flat.set_name_text_map_hash) {
                        Some (art_name) => {
                            print!(" {},", art_name.to_string());
                            let mut good_artifact = GoodArtifact {
                                set_key: art_name.to_string().to_owned(),
                                slot_key: DICT.get(&flat.equip_type)
                                    .context("Slot not found in dictionary.")?
                                    .to_string(),
                                level: reliquary.level-1,
                                rarity: flat.rank_level,
                                main_stat_key: DICT.get(&flat.reliquary_mainstat.main_prop_id)
                                    .context("Main stat not found in dictionary.")?
                                    .to_string(),
                                location: char_name.unwrap_or(&"").to_string(),
                                substats: vec![],
                            };
                            for substat in flat.reliquary_substats {
                                good_artifact.substats.push(
                                    GoodSubstat {
                                        key: DICT.get(&substat.append_prop_id)
                                        .context("Sub stat not found in dictionary.")?
                                        .to_string(),
                                        value: substat.stat_value,
                                    }
                                );
                            };
                            data.artifacts.replace(good_artifact);
                        },
                        None => print!(" UnknownArtifact,"),
                    }
                },
                Weapon {weapon, flat} => {
                    match DICT.get(&flat.name_text_map_hash) {
                        Some(wep_name) => {
                            print!(" {}.", wep_name);
                            let good_weapon = GoodWeapon {
                                key: wep_name.to_string(),
                                level: weapon.level,
                                ascension: weapon.promote_level.unwrap_or(0), // Can be null
                                refinement: 1 + *Vec::from_iter(weapon.affix_map.values())
                                                .pop()
                                                .unwrap_or(&0), // Assumed nullable
                                location: char_name.unwrap_or(&"").to_string(),
                            };
                            data.weapons.replace(good_weapon);
                        },
                        None => print!(" UnknownWeapon."),
                    }
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
            .send()
            .context("Error while doing HTTP GET")?
            .json()
            .context("Error while parsing JSON response")?
    )
}
