use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug)]
pub struct Item {
    
    // the ID must be unique in a game
    pub id: usize,

    pub name: String,
    pub mods: Vec<Mod>,
    
    pub inventory_tile_id: usize,
    pub inventory_w: i32,
    pub inventory_h: i32,

    pub map_tile_id: usize,
}


impl Item {
    
    pub fn get_attribute_total_mod(self, attribute: Attribute) -> f32 {
        let mut sum: f32 = 0.0;

        for m in self.mods {
            if m.attribute == attribute {
                sum = sum + m.power as f32;
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

        let path = Path::new("resources/items/items.csv");    
        let rs = read_to_string(path).unwrap();
        let mut lines = Vec::new();
        
        for line in rs.lines() {
            lines.push(line);
        }

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
                mods: Vec::new(),
            });
        }


        ItemFactory {
            next_id: 0,
            proto_items,
        }
    }


    pub fn make_item(&mut self) -> Item {
        let id = self.next_id;
        self.next_id += 1;
        
        let proto = &self.proto_items[0];

        Item {
            id, 
            name: proto.name.to_string(),
            mods: Vec::new(),

            inventory_tile_id: proto.inventory_tile_id,
            inventory_w: proto.inventory_w,
            inventory_h: proto.inventory_h,
        
            map_tile_id: proto.map_tile_id,
        }
    }
}




#[derive(PartialEq, Eq, Debug)]
pub enum Attribute {

    Structure = 1,
    Agility = 2,
    Computation = 3,

    Integrity = 4,
    Energy = 5,
}

#[derive(Debug)]
pub struct Mod {
    attribute: Attribute,
    power: i32,
}