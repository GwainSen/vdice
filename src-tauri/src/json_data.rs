use serde::{Deserialize, Serialize};
use serde_json::from_str;
use serde_json::to_string_pretty;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};

const SIMPLE_DICE_PATH: &str = "../lib/dice/simpledice/";
#[derive(Deserialize, Serialize, Debug)]
struct SimpleDice {
    name: String,
    sides: u32
}

pub fn read_simple_dice(){
    let json = std::fs::read_to_string("simple_dice_example.json").unwrap();
    let simple_dice = from_str::<SimpleDice>(&json).unwrap();
    println!("{:#?}",simple_dice.sides);
}

pub fn write_simple_dice(e_name: &str, e_sides: u32) -> Result<(), Box<dyn std::error::Error>>{
    fs::create_dir_all(SIMPLE_DICE_PATH).unwrap();
    //filter names for spaces and other formatting
    let json_data = SimpleDice { name: (e_name).to_string(), sides: e_sides };
    let json = to_string_pretty(&json_data).unwrap();
    let output = File::create(SIMPLE_DICE_PATH.to_owned() + e_name + ".json")?;

    let mut writer = BufWriter::new(output);

    let _ = writer.write_all(json.as_bytes());
    Ok(())
}
