use vecmath::Vector2;
use std::{collections::HashMap, fs::read_to_string, path::{Path, PathBuf}};
use opengl_graphics::{Texture, TextureSettings};

pub struct Tile {
    pub id: usize,
    pub size: Vector2<f64>,
    pub foot: Vector2<f64>,
    pub tex: Texture,
}


pub struct TileSet {
     pub tiles_by_id: HashMap<usize, Tile>,
     pub tiles_order_to_id: HashMap<usize, usize>,    
}


impl TileSet {
    pub fn load(path_str: &str, file_str: &str) -> TileSet {
        
        let mut fullpath = PathBuf::new();
        fullpath.push(path_str);
        fullpath.push(file_str);
        
        let path = Path::new(fullpath.as_path());    
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
                
                let tile_opt = load_tile(path_str, &line_vec, i);
                
                if tile_opt.is_some() {
                    let tile = tile_opt.unwrap();
                    let id = tile.id;
                    tileset.tiles_by_id.insert(id, tile);
                    tileset.tiles_order_to_id.insert(ordinal, id);
                }
                
                ordinal += 1;
            }
        }

        tileset        
    } 
}

fn load_tile(path_str: &str, lines: &Vec<&str>, start: usize) -> Option<Tile> {

    let id = lines[start + 2].parse::<usize>().unwrap();

    let mut size = lines[start + 3].split(" ");
    let width = size.next().unwrap().parse::<f64>().unwrap();
    let height = size.next().unwrap().parse::<f64>().unwrap();

    let mut foot = lines[start + 5].split(" ");
    let foot_x = foot.next().unwrap().parse::<f64>().unwrap();
    let foot_y = foot.next().unwrap().parse::<f64>().unwrap();

    let name = lines[start + 11];

    let mut result: Option<Tile> = None;

    if width > 1.0 || height > 1.0 {
        println!("Item {} is {} size={}x{} foot={}x{}", id, name, width, height, foot_x, foot_y);

        let mut filename = id.to_string();
        filename.push_str("-");
        filename.push_str(name);
        filename.push_str(".png");
        
        let mut path = PathBuf::new();
        path.push(path_str);
        path.push(filename);

        let tex = Texture::from_path(path.as_path(), &TextureSettings::new()).unwrap();

        result = Some(Tile {
            id,
            size: [width, height],
            foot: [foot_x, foot_y],
            tex,            
        });      
    }

    result
}
