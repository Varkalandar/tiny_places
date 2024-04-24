use std::{collections::HashMap, fs::read_to_string, path::Path, str::Lines};

pub struct Tile {
    id: usize,
}


pub struct TileSet {
     pub tiles_by_id: HashMap<usize, Tile>,
     pub tiles_order_to_id: HashMap<usize, usize>,    
}


impl TileSet {
    pub fn load(path_str: &str) -> TileSet {
        
        let path = Path::new(path_str);    
        let rs = read_to_string(path).unwrap();
        let mut line_vec = Vec::new();
        
        for line in rs.lines() {
            line_vec.push(line);
        }

        println!("Read {} lines from {:?}", line_vec.len(), path);
        
        let mut tileset = TileSet {
            tiles_by_id: HashMap::new(),
            tiles_order_to_id: HashMap::new(),
        };
        
        let mut ordinal = 0;    
        for i in 0..line_vec.len() {
            let line = line_vec[i];
            
            if line.starts_with("Tile Description") {
                
                let tile = load_tile(&line_vec, i);
                let id = tile.id;
                tileset.tiles_by_id.insert(id, tile);
                tileset.tiles_order_to_id.insert(ordinal, id);
                
                ordinal += 1;
            }
        }

        tileset        
    } 
}

    fn load_tile(lines: &Vec<&str>, start: usize) -> Tile {

        let id = lines[start + 2].parse::<usize>().unwrap();

        let mut size = lines[start + 3].split(" ");
        let width = size.next().unwrap().parse::<i32>().unwrap();
        let height = size.next().unwrap().parse::<i32>().unwrap();

        let mut foot = lines[start + 5].split(" ");
        let foot_x = foot.next().unwrap().parse::<i32>().unwrap();
        let foot_y = foot.next().unwrap().parse::<i32>().unwrap();

        let name = lines[start + 11];

        if width > 1 && height > 1 {
            println!("Item {} is {} size={}x{} foot={}x{}", id, name, width, height, foot_x, foot_y);
        }

        Tile {
            id: id,            
        }      
    }
    
    
    
    
