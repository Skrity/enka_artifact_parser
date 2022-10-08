use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::hash::{Hash,Hasher};
use anyhow::Result;

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
            source:
                String::from(format!("{}-{}",env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))),
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
    pub value: serde_json::Number,
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
