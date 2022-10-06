
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
    pub equipList: Vec<Equip>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Equip {
    pub itemId: u32,
    pub reliquary: Option<EquipRelic>,
    pub flat: EquipFlat,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct EquipFlat {
    pub setNameTextMapHash: Option<String>,
    pub rankLevel: u8,
    pub reliquaryMainstat: Option<RelicMS>,
    pub reliquarySubstats: Option<Vec<RelicSS>>,
    pub equipType: Option<String>,
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

// GOOD format description (not complete) https://frzyc.github.io/genshin-optimizer/#/doc
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct GoodType {
    pub format: String,
    pub version: u8,
    pub source: String,
    pub artifacts: Vec<GoodArtifact>,
}

impl GoodType {
    pub fn new() -> GoodType {
        GoodType {
            format: String::from("GOOD"),
            version: 2,
            source: String::from("enka_artifact_parser"),
            artifacts: vec![],
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
