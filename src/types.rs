
use serde::Deserialize;
use std::collections::HashMap;
use good::Substat;

pub mod good;
// Typify the input format (ENKA)
// https://api.enka.network/#/api https://github.com/EnkaNetwork/API-docs

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

// Type of data for compile-time hashmaps, copied from build.rs
#[derive(Deserialize)]
pub struct CharData {
    pub good_name: String,
    pub skill_order: (u32, u32, u32),
}