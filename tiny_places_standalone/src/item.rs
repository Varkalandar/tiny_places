use std::fs::read_to_string;
use std::path::Path;
use std::fmt::Formatter;
use core::str::Split;

use crate::inventory::Slot;

#[derive(Debug)]
pub struct Item {
    
    // the ID must be unique in a game
    pub id: usize,

    pub name: String,
    pub mods: Vec<Mod>,
    
    pub inventory_tile_id: usize,
    pub inventory_w: i32,
    pub inventory_h: i32,
    pub inventory_scale: f64,
    pub slot: Slot,
    pub map_tile_id: usize,
}


impl Item {
    
    pub fn get_attribute_total_mod(self, attribute: Attribute) -> f32 {
        let mut sum: f32 = 0.0;

        for m in self.mods {
            if m.attribute == attribute {
                sum = sum + m.min_value as f32;
            }            
        }
        
        sum
    }
    
    pub fn print_debug(self) {
        println!("{}", self.name);
    }
}


pub struct ItemFactory
{
    next_id: usize,

    proto_items: Vec<Item>,
}


impl ItemFactory {
    pub fn new() -> ItemFactory {

        let mut proto_items = read_proto_items();
        let mut plugins = read_plugins();

        proto_items.append(&mut plugins);

        ItemFactory {
            next_id: 0,
            proto_items,
        }
    }


    pub fn make_item(&mut self, key: usize) -> Item {
        let id = self.next_id;
        self.next_id += 1;
        
        let proto = &self.proto_items[key];

        Item {
            id, 
            name: proto.name.to_string(),
            mods: proto.mods.clone(),

            inventory_tile_id: proto.inventory_tile_id,
            inventory_w: proto.inventory_w,
            inventory_h: proto.inventory_h,
            inventory_scale: proto.inventory_scale,
            slot: proto.slot,
        
            map_tile_id: proto.map_tile_id,
        }
    }
}

fn read_proto_items() -> Vec<Item> {

    let lines = read_lines("resources/items/items.csv");
    let mut proto_items: Vec<Item> = Vec::new();

    for i in 1..lines.len() {
        let mut parts = lines[i].split(",");

        proto_items.push(Item {
            id: 0,
            name: parts.next().unwrap().to_string(),
            inventory_tile_id: parts.next().unwrap().parse::<usize>().unwrap(),
            map_tile_id: parts.next().unwrap().parse::<usize>().unwrap(),
            inventory_w: parts.next().unwrap().parse::<i32>().unwrap(),
            inventory_h: parts.next().unwrap().parse::<i32>().unwrap(),
            inventory_scale: parts.next().unwrap().parse::<f64>().unwrap(),
            slot: calc_slot(parts.next().unwrap().parse::<i32>().unwrap()),
            mods: parse_mods(&mut parts),
        });
    }

    proto_items
}


fn read_plugins() -> Vec<Item> {

    let lines = read_lines("resources/items/plugins.csv");
    let mut plugins: Vec<Item> = Vec::new();

    for i in 1..lines.len() {
        let mut parts = lines[i].split(",");

        plugins.push(Item {
            id: 0,
            name: parts.next().unwrap().to_string(),
            inventory_tile_id: parts.next().unwrap().parse::<usize>().unwrap(),
            map_tile_id: parts.next().unwrap().parse::<usize>().unwrap(),
            inventory_w: parts.next().unwrap().parse::<i32>().unwrap(),
            inventory_h: parts.next().unwrap().parse::<i32>().unwrap(),
            inventory_scale: parts.next().unwrap().parse::<f64>().unwrap(),
            slot: Slot::Bag,
            mods: Vec::new(),
        });
    }

    plugins
}


fn read_lines(pathname: &str) -> Vec<String> {
    let path = Path::new(pathname);    
    let rs = read_to_string(path).unwrap();
    let mut lines = Vec::new();
    
    for line in rs.lines() {
        lines.push(line.to_string());
    }

    lines
}


fn calc_slot(v: i32) -> Slot {
    match v {
        0 => Slot::Bag,
        1 => Slot::Stash,
        2 => Slot::Nose,
        3 => Slot::Body,
        4 => Slot::LWing,
        5 => Slot::RWing,
        6 => Slot::Engine,
        _ => Slot::Bag,
    }
}


fn parse_mods(parts: &mut Split<&str>) -> Vec<Mod> {
    let mut result = Vec::new();

    result.push(parse_mod(parts.next(), Attribute::Structure));
    result.push(parse_mod(parts.next(), Attribute::Agility));
    result.push(parse_mod(parts.next(), Attribute::Armor));
    result.push(parse_mod(parts.next(), Attribute::Computation));
    result.push(parse_mod(parts.next(), Attribute::Speed));
    result.push(parse_mod(parts.next(), Attribute::PhysicalDamage));
    result.push(parse_mod(parts.next(), Attribute::PlasmaDamage));
    result.push(parse_mod(parts.next(), Attribute::RadiationDamage));

    result
}

fn parse_mod(input: Option<&str>, attribute: Attribute) -> Mod {

    let (min_value, max_value) = parse_range(input.unwrap());

    Mod { 
        attribute,
        min_value,
        max_value,
    }
}


fn parse_range(input: &str) -> (i32, i32) {
    // .parse::<i32>().unwrap();

    if input.contains("-") {
        let mut parts = input.split("-");
        let min_value = parts.next().unwrap().parse::<i32>().unwrap();
        let max_value = parts.next().unwrap().parse::<i32>().unwrap();
        (min_value, max_value)
    }
    else {
        let value = input.parse::<i32>().unwrap();
        (value, value)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Attribute {

    Structure,
    Agility,
    Armor,
    Computation,
    Speed,
    PhysicalDamage,
    PlasmaDamage,
    RadiationDamage,

    Integrity,
    Energy,
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {

        let name = match self {
            Attribute::Structure => "Structure",
            Attribute::Agility => "Agility",
            Attribute::Armor => "Armor",
            Attribute::Computation => "Computation",
            Attribute::Speed => "Speed",
            Attribute::PhysicalDamage => "Physical Damage",
            Attribute::PlasmaDamage => "Plasma Damage",
            Attribute::RadiationDamage => "Radiation Damage",
                
            Attribute::Integrity => "Integrity",
            Attribute::Energy => "Energy",        
        };

        write!(f, "{}", name)
    }
}


#[derive(Debug, Clone)]
pub struct Mod {
    pub attribute: Attribute,
    pub min_value: i32,
    pub max_value: i32,
}