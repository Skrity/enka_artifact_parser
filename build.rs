use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

// Also you can pull jsons from github
// https://raw.githubusercontent.com/EnkaNetwork/API-docs/master/store/loc.json
// https://raw.githubusercontent.com/EnkaNetwork/API-docs/master/store/characters.json
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/loc.json");
    println!("cargo:rerun-if-changed=src/loc.cbor");
    println!("cargo:rerun-if-changed=src/characters.json");
    println!("cargo:rerun-if-changed=src/characters.cbor");
    println!("cargo:rerun-if-changed=src/enka.json");
    println!("cargo:rerun-if-changed=src/enka.cbor");

    match write_cbor("src/loc.cbor", &parse_loc_json()["en"]) {
        Ok(()) => {},
        Err(r) =>
            panic!("Something gone terribly wrong while saving loc.cbor: {:?}", r),
    }

    match write_cbor("src/characters.cbor", parse_characters_json()) {
        Ok(()) => {},
        Err(r) =>
            panic!("Something gone terribly wrong while saving characters.cbor: {:?}", r),
    }

    match write_cbor("src/enka.cbor", create_enka_dict()) {
        Ok(()) => {},
        Err(r) =>
            panic!("Something gone terribly wrong while saving enka.cbor: {:?}", r),
    }
}

fn write_cbor<T: serde::Serialize>(path: &str, bar: T) -> Result<(), Box<dyn std::error::Error>> {
    let cbor_file = File::create(path)?;
    serde_cbor::to_writer(cbor_file, &bar)?;
    Ok(())
}

// Please can you make this generic?
fn parse_loc_json() -> HashMap<String, HashMap<String, String>> {
    let file = File::open("src/loc.json").unwrap();
    let reader = BufReader::new(file);
    let u: HashMap<String, HashMap<String,String>> = serde_json::from_reader(reader).unwrap();
    u
}

fn parse_characters_json() -> HashMap<String, String> {
    // Read file characters.json into characters var
    let file = File::open("src/characters.json").unwrap();
    let reader = BufReader::new(file);
    let characters: HashMap<String, CharacterInfo> = serde_json::from_reader(reader).unwrap();
    // Read locale
    let loc_map: HashMap<String, HashMap<String, String>> = parse_loc_json();
    let loc_map_en = &loc_map["en"];
    // Resulting HashMap
    let mut out: HashMap<String, String> = HashMap::new();
    for (char, info) in characters {
        match info.NameTextMapHash {
            Some(x) => {
                out.insert(char, loc_map_en.get(&x.to_string()).unwrap().to_string());
            },
            None => continue,
        }
    }
    out
}

fn create_enka_dict() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("EQUIP_BRACER"                  , "flower"),
        ("EQUIP_NECKLACE"                , "plume"),
        ("EQUIP_SHOES"                   , "sands"),
        ("EQUIP_RING"                    , "goblet"),
        ("EQUIP_DRESS"                   , "circlet"),

        ("FIGHT_PROP_HP"                 , "hp"),
        ("FIGHT_PROP_HP_PERCENT"         , "hp_"),
        ("FIGHT_PROP_ATTACK"             , "atk"),
        ("FIGHT_PROP_ATTACK_PERCENT"     , "atk_"),
        ("FIGHT_PROP_DEFENSE"            , "def"),
        ("FIGHT_PROP_DEFENSE_PERCENT"    , "def_"),
        ("FIGHT_PROP_CRITICAL"           , "critRate_"),
        ("FIGHT_PROP_CRITICAL_HURT"      , "critDMG_"),
        ("FIGHT_PROP_CHARGE_EFFICIENCY"  , "enerRech_"),
        ("FIGHT_PROP_HEAL_ADD"           , "heal_"),
        ("FIGHT_PROP_ELEMENT_MASTERY"    , "eleMas"),
        ("FIGHT_PROP_PHYSICAL_ADD_HURT"  , "physical_dmg_"),
        ("FIGHT_PROP_FIRE_ADD_HURT"      , "pyro_dmg_"),
        ("FIGHT_PROP_ELEC_ADD_HURT"      , "electro_dmg_"),
        ("FIGHT_PROP_WATER_ADD_HURT"     , "hydro_dmg_"),
        ("FIGHT_PROP_WIND_ADD_HURT"      , "anemo_dmg_"),
        ("FIGHT_PROP_ICE_ADD_HURT"       , "cryo_dmg_"),
        ("FIGHT_PROP_ROCK_ADD_HURT"      , "geo_dmg_"),
        ("FIGHT_PROP_GRASS_ADD_HURT"     , "dendro_dmg_"),
    ])
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize)]
struct CharacterInfo {
    NameTextMapHash: Option<u32>,
}