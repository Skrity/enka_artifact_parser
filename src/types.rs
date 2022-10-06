
use serde::{Deserialize, Serialize};

// Typify the input format (ENKA) https://api.enka.network/#/api https://github.com/EnkaNetwork/API-docs

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct EnkaPlayer {
    pub playerInfo: PlayerInfo,
    pub avatarInfoList: Vec<AvatarInfo>,
    pub ttl: u8,
    pub uid: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerInfo {
    pub nickname: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct AvatarInfo {
    pub avatarId: u32,
    pub equipList: Vec<EquipVariant>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum EquipVariant {
    Artifact {
        itemId: u32,
        reliquary: EquipRelic,
        flat: EquipFlatVariantArtifact,
    },
    Weapon {
        itemId: u32,
        weapon: EquipWeapon,
        flat: EquipFlatVariantWeapon,
    },
}
// Artifact
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct EquipFlatVariantArtifact {
    pub setNameTextMapHash: String,
    pub rankLevel: u8,
    pub reliquaryMainstat: RelicMS,
    pub reliquarySubstats: Vec<RelicSS>,
    pub equipType: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct EquipRelic {
    pub level: u8,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct RelicMS {
    pub mainPropId: String,
    pub statValue: f64,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct RelicSS {
    pub appendPropId: String,
    pub statValue: f64,
}

// Weapon
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct EquipFlatVariantWeapon {
    pub nameTextMapHash: String,
    pub rankLevel: u8,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct EquipWeapon {
    pub level: u8,
    pub promoteLevel: u8,
    pub affixMap: std::collections::HashMap<String,u8>,
}
// GOOD format description (not complete) https://frzyc.github.io/genshin-optimizer/#/doc
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct GoodType {
    pub format: String,
    pub version: u8,
    pub source: String,
    pub artifacts: Vec<GoodArtifact>,
    pub weapons: Vec<GoodWeapon>,
}

impl GoodType {
    pub fn new() -> GoodType {
        GoodType {
            format: String::from("GOOD"),
            version: 2,
            source: String::from("enka_artifact_parser"),
            artifacts: vec![],
            weapons: vec![],
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

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct GoodArtifact {
    pub setKey: String,
    pub slotKey: String,
    pub level: u8,
    pub rarity: u8,
    pub mainStatKey: String,
    pub location: String,
    pub substats: Vec<GoodSubstat>,
    pub _id: u32, //Added for safekeeping
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct GoodSubstat {
    pub key: String,
    pub value: f64,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct GoodWeapon {
    pub key: String,
    pub level: u8,
    pub ascension: u8,
    pub refinement: u8,
    pub location: String,
    pub _id: u32, //Added for safekeeping
}
