use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use std::env;

// Also you can pull jsons from github
// https://raw.githubusercontent.com/EnkaNetwork/API-docs/master/store/loc.json
// https://raw.githubusercontent.com/EnkaNetwork/API-docs/master/store/characters.json
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/loc.json");
    println!("cargo:rerun-if-changed=src/characters.json");

    test_format_for_good();

    let mut locale = parse_loc_json("src/loc.json");
    let mut skill_order: HashMap<String, (u32, u32, u32)> = HashMap::new();
    for (k, v) in create_enka_dict() {
        locale.insert(k.to_owned(), v.to_owned());
    }
    let char_json = parse_characters_json("src/characters.json");
    for (k, v) in char_json {
        locale.insert(k.to_owned(), v.0);
        skill_order.insert(k, (v.1, v.2, v.3));
    }

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());
    // GENERATE MAP FOR LOCALISATION DICT
    let mut builder = phf_codegen::Map::new();

    for (key, value) in locale {
        builder.entry(key, &format!("\"{}\"", value));
    }
    writeln!(
        &mut file,
         "static DICT: phf::Map<&'static str, &'static str> =\n{};\n",
        builder.build()
    ).unwrap();
    // GENERATE MAP FOR SKILL ORDER
    let mut builder_2 = phf_codegen::Map::new();

    for (key, value) in skill_order {
        builder_2.entry(key, &format!("&({},{},{})", value.0, value.1, value.2));
    }
    writeln!(
        &mut file,
         "static SKILLS: phf::Map<&'static str, &'static (u32, u32, u32)> =\n{};\n",
        builder_2.build()
    ).unwrap();

}

// Please can you make this generic?
fn parse_loc_json(filename: &str) -> HashMap<String, String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let u: HashMap<String, HashMap<String,String>> = serde_json::from_reader(reader).unwrap();
    let mut temp: HashMap<String,String> = HashMap::new();
    for (key, value) in &u["en"] {
        temp.insert(key.to_string(),format_for_good(value.to_string()));
    }
    temp
}

// Pre-converted HashMap
fn parse_characters_json(filename: &str) -> HashMap<String, (String, u32, u32, u32)> {
    // Read file characters.json into characters var
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let characters: HashMap<String, CharacterInfo> = serde_json::from_reader(reader).unwrap();
    // Read locale
    let loc_map_en: HashMap<String, String> = parse_loc_json("src/loc.json");
    // Resulting HashMap
    let mut out: HashMap<String, (String, u32, u32, u32)> = HashMap::new();
    for (char, info) in characters {
        match info.NameTextMapHash {
            Some(x) => {
                out.insert(char, (
                    format_for_good(loc_map_en.get(&x.to_string()).unwrap().to_string()),
                    info.SkillOrder.unwrap().0,
                    info.SkillOrder.unwrap().1,
                    info.SkillOrder.unwrap().2,
                ));
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

fn format_for_good(input: String) -> String {
    input
        .split(" ")
        .map(|word| format!("{}{}", &word[..1].to_uppercase(), &word[1..]))
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect()
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize)]
struct CharacterInfo {
    SkillOrder: Option<(u32, u32, u32)>,
    NameTextMapHash: Option<u32>,
}

fn test_format_for_good() {
    assert_eq!("GladiatorsFinale", format_for_good("Gladiator's Finale".to_string()));
    assert_eq!("SpiritLocketOfBoreas", format_for_good("Spirit Locket of Boreas".to_string()));
    assert_eq!("TheCatch", format_for_good("The Catch".to_string()));
    assert_eq!("ABDD", format_for_good("'''A b d435 D123/ /'''".to_string()));
    // Add test for every non-alphabetical symbol
}
