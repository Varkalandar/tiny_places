use vecmath::Vector2;

use std::io::prelude::*;
use std::io::{Result, BufWriter};
use std::fs::File;
use std::path::PathBuf;

use crate::item::Item;
use crate::mob::Mob;

pub const MAP_GROUND_LAYER:usize = 0;
pub const MAP_DECO_LAYER:usize = 1;
pub const MAP_CLOUD_LAYER:usize = 2;


pub struct Map {
    pub layers: [Vec<MapObject>; 7],

    pub player: Mob,

    pub has_selection: bool,
    pub selected_item: usize,
    pub selected_layer: usize,
}


impl Map {
    pub fn new() -> Map {
        let layers = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(),];        
        Map {
            layers,
            player: Mob::new(1000.0, 1000.0),
            has_selection: false,
            selected_item: 0,
            selected_layer: 0,
        }
    }


    pub fn find_nearest_object(&mut self, layer: usize, position: &Vector2<f64>) -> Option<usize> {
        let objects = &self.layers[layer];
        let mut distance = 999999.0;
        let mut best_idx = 0;

        for idx in 0..objects.len() {
            let object = &objects[idx];
            let dx = object.position[0] - position[0];
            let dy = object.position[1] - position[1];
            let d2 = dx * dx + dy * dy;

            println!("object {} has distance {}", object.tile_id, d2);

            if d2 < distance {
                distance = d2;
                best_idx = idx;
            }
        }

        let mut result:Option<usize> = None;

        if distance < 10000.0 {
            result = Some(best_idx);
            println!("  best object is {}", best_idx);
        }

        result
    }


    pub fn update(&mut self, dt: f64) {
        self.player.move_by_time(dt);
    }


    pub fn load(&mut self, filename: &str) {

        for layer in &mut self.layers {
            layer.clear();
        }

        let mut path = PathBuf::new();
        path.push("maps");
        path.push(filename);

        let content = std::fs::read_to_string(path.as_path()).unwrap();
        let mut lines = content.lines();

        lines.next(); // version
        lines.next(); // map name

        for line in lines {
            println!("{}", line);

            let parts: Vec<&str> = line.split(",").collect();

            let layer = parts[0].parse::<usize>().unwrap();
            let tile_id = parts[1].parse::<usize>().unwrap();

            let x = parts[2].parse::<f64>().unwrap();
            let y = parts[3].parse::<f64>().unwrap();
            let scale = parts[4].parse::<f64>().unwrap();
            // parts[5] is an RGBA tuple

            println!("{}, {}, {}, {}, {}", layer, tile_id, x, y, scale);

            let m = MapObject::new(tile_id, [x, y], scale);

            self.layers[layer].push(m);
        }
        
    }


    pub fn save(&self, filename: &str) -> Result<()> {
        let mut path = PathBuf::new();
        path.push("maps");
        path.push(filename);

        let f = File::create(path.as_path())?;
        {        
            let mut writer = BufWriter::new(f);

            // write a byte to the buffer
            writer.write("v10\n".as_bytes())?;
            writer.write("Testmap\n".as_bytes())?;
            
            self.save_layer(&mut writer, MAP_GROUND_LAYER);
            self.save_layer(&mut writer, MAP_DECO_LAYER);
            self.save_layer(&mut writer, MAP_CLOUD_LAYER);
        }

        Ok(())
    }
    
    
    fn save_layer(&self, writer: &mut BufWriter<File>, layer: usize) -> Result<()> {
        let objects = &self.layers[layer];

        for object in objects {
            let line = 
            layer.to_string() + "," +
            &object.tile_id.to_string() + "," +
            &object.position[0].to_string() + "," +
            &object.position[1].to_string() + "," +
            &object.scale.to_string() + "," +
            "1.0 1.0 1.0 1.0\n";
            
            writer.write(line.as_bytes())?;
        }

        Ok(())
    }    
}


pub struct MapObject {
    pub tile_id: usize,
    pub position: Vector2<f64>,
    pub scale: f64,
    pub item: Option<Item>,
    pub color: [f32; 4],    
}


impl MapObject {
    
    pub fn new(tile_id: usize, position: Vector2<f64>, scale: f64) -> MapObject {
        MapObject { 
            tile_id, 
            position, 
            scale,
            color: [1.0, 1.0, 1.0, 1.0],
            item: None,
        }
    }

}