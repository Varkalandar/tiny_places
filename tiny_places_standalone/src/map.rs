use vecmath::Vector2;

use std::io::prelude::*;
use std::io::{Result, BufWriter};
use std::fs::File;
use std::path::PathBuf;

use crate::item::Item;
use crate::mob::{Mob, Visual};
use crate::inventory::Inventory;

pub const MAP_GROUND_LAYER:usize = 0;
pub const MAP_DECO_LAYER:usize = 1;
pub const MAP_CLOUD_LAYER:usize = 2;


pub struct Map {
    pub layers: [Vec<MapObject>; 7],

    pub player: Mob,

    // all items on this map
    pub items: Inventory,

    pub has_selection: bool,
    pub selected_item: usize,
    pub selected_layer: usize,

    pub backdrop_image_name: String,
}


impl Map {
    pub fn new(backdrop_image_name: &str) -> Map {
        let layers = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(),];        
        
        let player_visual = Visual {
            base_image_id: 39,
            current_image_id: 39,
            frames: 16,        
        };

        let mut player = Mob::new(1000.0, 1000.0);
        player.visual = player_visual;

        Map {
            layers,
            player,
            items: Inventory::new(),
            has_selection: false,
            selected_item: 0,
            selected_layer: 0,

            backdrop_image_name: backdrop_image_name.to_string(),
        }
    }


    pub fn find_nearest_object(&self, layer: usize, position: &Vector2<f64>) -> Option<usize> {
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


    pub fn find_idx_from_id(&self, layer: usize, id: usize) -> Option<usize> {
    
        let objects = &self.layers[layer];

        for idx in 0..objects.len() {
            let object = &objects[idx];
            if object.tile_id == id {
                return Some(idx);
            }
        }

        None
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
        self.backdrop_image_name = lines.next().unwrap().to_string(); // map name

        println!("Backdrop image='{}'", self.backdrop_image_name);

        for line in lines {
            println!("{}", line);

            let parts: Vec<&str> = line.split(",").collect();

            let layer = parts[0].parse::<usize>().unwrap();
            let tile_id = parts[1].parse::<usize>().unwrap();

            let x = parts[2].parse::<f64>().unwrap();
            let y = parts[3].parse::<f64>().unwrap();
            let scale = parts[4].parse::<f64>().unwrap();

            // parts[5] is an RGBA tuple
            let mut color_in = parts[5].split(" ");

            let mut color: [f32; 4] = [0.0; 4];
            for i in 0..4 {
                color[i] = color_in.next().unwrap().parse::<f32>().unwrap();
            }

            println!("{}, {}, {}, {}, {}, {:?}", layer, tile_id, x, y, scale, color);

            let mut m = MapObject::new(tile_id, [x, y], scale);
            m.color = color;
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

            let backdrop_name = self.backdrop_image_name.to_string()  + "\n";

            writer.write(backdrop_name.as_bytes())?;
            
            self.save_layer(&mut writer, MAP_GROUND_LAYER)?;
            self.save_layer(&mut writer, MAP_DECO_LAYER)?;
            self.save_layer(&mut writer, MAP_CLOUD_LAYER)?;
        }

        Ok(())
    }
    
    
    fn save_layer(&self, writer: &mut BufWriter<File>, layer: usize) -> Result<()> {
        let objects = &self.layers[layer];

        for object in objects {
            let color = object.color; 

            let line = 
            layer.to_string() + "," +
            &object.tile_id.to_string() + "," +
            &object.position[0].to_string() + "," +
            &object.position[1].to_string() + "," +
            &object.scale.to_string() + "," +
            &color[0].to_string() + " " +
            &color[1].to_string() + " " +
            &color[2].to_string() + " " +
            &color[3].to_string() + " " +            
            "\n";
            
            writer.write(line.as_bytes())?;
        }

        Ok(())
    }
    
    
    pub fn move_selected_object(&mut self, dx: f64, dy: f64) {        
        if self.has_selection {
            let object = &mut self.layers[self.selected_layer][self.selected_item];
            object.position[0] += dx;
            object.position[1] += dy;
        }
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