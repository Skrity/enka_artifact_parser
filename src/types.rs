
use serde::{Deserialize, Serialize};
use std::collections::{HashSet, HashMap};

// Typify the input format (ENKA) https://api.enka.network/#/api https://github.com/EnkaNetwork/API-docs //#[serde(rename_all = "camelCase")]

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnkaPlayer {
    pub player_info: PlayerInfo,
    pub avatar_info_list: Vec<AvatarInfo>,
    pub ttl: u8,
    pub uid: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerInfo {
    pub nickname: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AvatarInfo {
    pub avatar_id: u32,
    pub equip_list: Vec<EquipVariant>,
}

#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EquipFlatVariantArtifact {
    pub set_name_text_map_hash: String,
    pub rank_level: u8,
    pub reliquary_mainstat: RelicMS,
    pub reliquary_substats: Vec<RelicSS>,
    pub equip_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EquipRelic {
    pub level: u8,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RelicMS {
    pub main_prop_id: String,
    pub stat_value: Substat,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RelicSS {
    pub append_prop_id: String,
    pub stat_value: Substat,
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub struct Substat(serde_json::Number);

// Weapon
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EquipFlatVariantWeapon {
    pub name_text_map_hash: String,
    pub rank_level: u8,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EquipWeapon {
    pub level: u8,
    pub promote_level: u8,
    pub affix_map: HashMap<String,u8>,
}
// GOOD format description (not complete) https://frzyc.github.io/genshin-optimizer/#/doc
#[derive(Serialize, Deserialize, Debug)]
pub struct GoodType {
    pub format: String,
    pub version: u8,
    pub source: String,
    pub artifacts: HashSet<GoodArtifact>,
    pub weapons: HashSet<GoodWeapon>,
}

impl GoodType {
    pub fn new() -> GoodType {
        GoodType {
            format: String::from("GOOD"),
            version: 2,
            source: String::from("enka_artifact_parser"),
            artifacts: HashSet::new(),
            weapons: HashSet::new(),
        }
    }

    pub fn to_file(&self, filename: String) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs::File;
        use std::io::Write;

        let json_string = serde_json::to_string(self)?;
        let mut file = File::create(filename)?;
        writeln!(&mut file, "{}", json_string)?;
        Ok(())
    }

    pub fn from_file(filename: String) -> Result<GoodType, Box<dyn std::error::Error>> {
        use std::fs::File;
        use std::io::BufReader;

        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }

}


#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub struct GoodSubstat {
    pub key: String,
    pub value: Substat,
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub struct GoodWeapon {
    pub key: String,
    pub level: u8,
    pub ascension: u8,
    pub refinement: u8,
    pub location: String,
}
