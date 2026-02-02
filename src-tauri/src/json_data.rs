use serde::{Deserialize, Serialize};
use serde_json::from_str;

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
