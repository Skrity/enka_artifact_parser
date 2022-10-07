
use serde::{Deserialize, Serialize};
use std::collections::{HashSet, HashMap};
use std::hash::{Hash,Hasher};
use anyhow::Result;

// Typify the input format (ENKA) https://api.enka.network/#/api https://github.com/EnkaNetwork/API-docs //#[serde(rename_all = "camelCase")]

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnkaPlayer {
    pub player_info: PlayerInfo,
    pub avatar_info_list: Vec<AvatarInfo>,
    pub ttl: u8,
    pub uid: String,
}

#[derive(Deserialize)]
pub struct PlayerInfo {
    pub nickname: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarInfo {
    pub avatar_id: u32,
    pub talent_id_list: Option<Vec<u32>>, //Constellations
    pub prop_map: AvatarProps,
    pub skill_depot_id: u32,
    pub skill_level_map: HashMap<String,u8>,
    pub equip_list: Vec<EquipVariant>,
}

#[derive(Deserialize)]
pub struct AvatarProps {
    #[serde(rename = "4001")]
    pub level: Prop,
    #[serde(rename = "1002")]
    pub ascension: Prop,
}

#[derive(Deserialize)]
pub struct Prop {
    pub val: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum EquipVariant {
    Artifact {
        reliquary: EquipRelic,
        flat: EquipFlatVariantArtifact,
    },
    #[serde(rename_all = "camelCase")]
    Weapon {
        item_id: u32,
        weapon: EquipWeapon,
        flat: EquipFlatVariantWeapon,
    },
}
// Artifact
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipFlatVariantArtifact {
    pub set_name_text_map_hash: String,
    pub rank_level: u8,
    pub reliquary_mainstat: RelicMS,
    pub reliquary_substats: Vec<RelicSS>,
    pub equip_type: String,
}

#[derive(Deserialize)]
pub struct EquipRelic {
    pub level: u8,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelicMS {
    pub main_prop_id: String,
    pub stat_value: Substat,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelicSS {
    pub append_prop_id: String,
    pub stat_value: Substat,
}

// Weapon
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipFlatVariantWeapon {
    pub name_text_map_hash: String,
    pub rank_level: u8,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipWeapon {
    pub level: u8,
    pub promote_level: Option<u8>,
    pub affix_map: HashMap<String,u8>,
}
// GOOD format description (not complete) https://frzyc.github.io/genshin-optimizer/#/doc
#[derive(Serialize, Deserialize)]
pub struct GoodType {
    pub format: String,
    pub version: u8,
    pub source: String,
    pub characters: HashSet<GoodCharacter>,
    pub artifacts: HashSet<GoodArtifact>,
    pub weapons: HashSet<GoodWeapon>,
}

impl GoodType {
    pub fn new() -> GoodType {
        GoodType {
            format: String::from("GOOD"),
            version: 2,
            source: String::from("enka_artifact_parser"),
            characters: HashSet::new(),
            artifacts: HashSet::new(),
            weapons: HashSet::new(),
        }
    }

    pub fn to_file(&self, filename: String) -> Result<()> {
        use std::fs::File;
        use std::io::Write;

        let json_string = serde_json::to_string(self)?;
        let mut file = File::create(filename)?;
        writeln!(&mut file, "{}", json_string)?;
        Ok(())
    }

    pub fn from_file(filename: String) -> Result<GoodType> {
        use std::fs::File;
        use std::io::BufReader;

        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }

}


#[derive(Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GoodArtifact {
    pub set_key: String,
    pub slot_key: String,
    pub level: u8,
    pub rarity: u8,
    pub main_stat_key: String,
    pub location: String,
    pub substats: Vec<GoodSubstat>,
}

impl GoodArtifact {
    fn key(&self) -> (&String, &String, &u8, &u8, &String, &Vec<GoodSubstat>) {(
        &self.set_key,
        &self.slot_key,
        &self.level,
        &self.rarity,
        &self.main_stat_key,
        &self.substats
    )}
}

impl Hash for GoodArtifact {
    fn hash<H>(&self, state: &mut H) where H: Hasher { self.key().hash(state); }
}

impl PartialEq for GoodArtifact {
    fn eq(&self, other: &Self) -> bool { self.key() == other.key() }
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct GoodSubstat {
    pub key: String,
    pub value: Substat,
}

#[derive(Serialize, Deserialize, Eq)]
pub struct GoodWeapon {
    pub key: String,
    pub level: u8,
    pub ascension: u8,
    pub refinement: u8,
    pub location: String,
}

impl GoodWeapon {
    fn key(&self) -> (&String, &u8, &u8, &u8) {(
        &self.key,
        &self.level,
        &self.ascension,
        &self.refinement,
    )}
}

impl Hash for GoodWeapon {
    fn hash<H>(&self, state: &mut H) where H: Hasher { self.key().hash(state); }
}

impl PartialEq for GoodWeapon {
    fn eq(&self, other: &Self) -> bool { self.key() == other.key() }
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct GoodCharacter {
    pub key: String,
    pub level: u8,
    pub constellation: u8,
    pub ascension: u8,
    pub talent: GoodTalents,

}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct GoodTalents {
    pub auto: u8,
    pub skill: u8,
    pub burst: u8,
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct Substat(serde_json::Number); //Also used in ENKA

#[derive(Deserialize)]
pub struct CharData {
    pub good_name: String,
    pub skill_order: (u32, u32, u32),
}