use vecmath::Vector2;
use std::{rc::Rc, collections::HashMap, fs::read_to_string, path::{Path, PathBuf}};

use sdl2::render::Texture;
use sdl2::render::TextureCreator;

use crate::load_texture;

pub struct Tile {
    pub id: usize,
    pub size: Vector2<f64>,
    pub foot: Vector2<f64>,
    pub tex: Texture,
    pub name: String,
}


pub struct TileSet {
     pub tiles_by_id: HashMap<usize, Rc<Tile>>,
     pub tiles_order_to_id: HashMap<usize, usize>,    
}


impl TileSet {

    /**
     * creates an empty tile set
     */
     /*
    pub fn new() -> TileSet {
        TileSet {
            tiles_by_id: HashMap::new(),
            tiles_order_to_id: HashMap::new(),
        }
    }
    */
    
    pub fn load(creator: &TextureCreator<sdl2::video::WindowContext>, path_str: &str, file_str: &str) -> TileSet {
        
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
                
                let tile_opt = load_tile(creator, path_str, &line_vec, i);
                
                if tile_opt.is_some() {
                    let tile = tile_opt.unwrap();
                    let id = tile.id;

                    tileset.tiles_by_id.insert(id, Rc::new(tile));
                    tileset.tiles_order_to_id.insert(ordinal, id);
                }
                
                ordinal += 1;
            }
        }

        tileset        
    }
    
    
    pub fn shallow_copy(&self) -> TileSet {
        let mut result = TileSet {
            tiles_by_id: HashMap::new(),
            tiles_order_to_id: HashMap::new(),
        };

        for key in &self.tiles_order_to_id {
            let id = *key.1;
            result.tiles_order_to_id.insert(*key.0, id);

            let tile = self.tiles_by_id.get(&id).unwrap();
            result.tiles_by_id.insert(id, tile.clone());
        }
        
        result
    }
}


fn load_tile(creator: &TextureCreator<sdl2::video::WindowContext>, path_str: &str, lines: &Vec<&str>, start: usize) -> Option<Tile> {

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
        // println!("Item {} is {} size={}x{} foot={}x{}", id, name, width, height, foot_x, foot_y);

        let filename = 
            path_str.to_string() + "/" + &id.to_string() + "-" + name + ".png";
        
        let mut tex = load_texture(creator, &filename);

        result = Some(Tile {
            id,
            size: [width, height],
            foot: [foot_x, foot_y],
            tex,
            name: name.to_string(),            
        });      
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;
    use sdl2::render::CanvasBuilder;


    #[test]
    fn test_load_tileset() {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
    
        let window = video_subsystem
            .window("Fractal Lands 0.0.1", 800, 500)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string()).unwrap();

        let canvas_builder = CanvasBuilder::new(window);
        let canvas = 
            canvas_builder
                .accelerated()
                .present_vsync()
                .build()
                .unwrap();

        let creator = canvas.texture_creator();

        let set = TileSet::load(&creator, "../tiny_places_client/resources/grounds", "map_objects.tica");
    }
}