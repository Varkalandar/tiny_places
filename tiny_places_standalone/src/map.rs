use vecmath::{Vector2, vec2_add, vec2_scale};
use std::f64::consts::PI;

use std::io::prelude::*;
use std::io::{Result, BufWriter};
use std::fs::File;
use std::path::PathBuf;

use crate::item::Item;
use crate::inventory::Inventory;

pub const MAP_GROUND_LAYER:usize = 0;
pub const MAP_OBJECT_LAYER:usize = 1;
pub const MAP_CLOUD_LAYER:usize = 2;


pub struct Map {
    pub layers: [Vec<MapObject>; 7],

    // all items on this map
    pub items: Inventory,

    pub has_selection: bool,
    pub selected_item: usize,
    pub selected_layer: usize,

    pub backdrop_image_name: String,

    pub factory: MapObjectFactory,
}


impl Map {
    pub fn new(backdrop_image_name: &str) -> Map {
        let mut layers = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(),];        
        
        let player_visual = Visual {
            base_image_id: 39,
            tileset_id: 4,
            current_image_id: 39,
            frames: 16, 
            color: [1.0, 1.0, 1.0, 1.0],       
        };

        let mut factory = MapObjectFactory {
            next_id: 1,
        };


        let mut player = factory.create_mob(39, 4, [1000.0, 1000.0], 1.0);
        player.visual = player_visual;

        layers[MAP_OBJECT_LAYER].push(player);

        Map {
            layers,
            items: Inventory::new(),
            has_selection: false,
            selected_item: 0,
            selected_layer: 0,

            backdrop_image_name: backdrop_image_name.to_string(),
        
            factory,
        }
    }


    pub fn find_player_index(&self) -> usize {

        self.find_index_from_image_id(MAP_OBJECT_LAYER, 39).unwrap()
    }


    pub fn player_position(&self) -> Vector2<f64> {
        for mob in &self.layers[MAP_OBJECT_LAYER] {
            if mob.visual.base_image_id == 39 {
                return mob.position;
            }
        }

        [0.0, 0.0]
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

            println!("object {} has distance {}", object.visual.base_image_id, d2);

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


    pub fn find_index_from_image_id(&self, layer: usize, id: usize) -> Option<usize> {
    
        let objects = &self.layers[layer];

        for idx in 0..objects.len() {
            let object = &objects[idx];
            if object.visual.base_image_id == id {
                return Some(idx);
            }
        }

        None
    }

/*
    pub fn fireProjectile(source, id, layer, ptype, castTime, dx, dy, speed) {
        println!("Adding projectile with type {} fired at {}, {}", ptype, dx, dy)
    
        local shooter, i = findMob(source, layer)
        local nx = dx - shooter.x
        local ny = dy - shooter.y
    
        shooter:orient(nx, ny)
    
        local spell = spells.new(map, shooter, id, layer, ptype, castTime, dx, dy, speed, animationSet)
        
        // some spells have a buildup time, the projectile will be fired later
        table.insert(map.actions, spell)
    
    }    
*/

    pub fn update(&mut self, dt: f64) {

        for mob in &mut self.layers[MAP_OBJECT_LAYER] {
            mob.move_dt(dt);
        }
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

            let mut m = self.factory.create_mob(tile_id, layer, [x, y], scale);
            m.visual.color = color;
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
            self.save_layer(&mut writer, MAP_OBJECT_LAYER)?;
            self.save_layer(&mut writer, MAP_CLOUD_LAYER)?;
        }

        Ok(())
    }
    
    
    fn save_layer(&self, writer: &mut BufWriter<File>, layer: usize) -> Result<()> {
        let objects = &self.layers[layer];

        for object in objects {
            let color = object.visual.color; 

            let line = 
            layer.to_string() + "," +
            &object.visual.base_image_id.to_string() + "," +
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

    uid: u64,
    pub visual: Visual,
    pub attributes: MobAttributes,
    pub item: Option<Item>,

    // world coordinates of this object. Note that screen coordinates are different
    pub position: Vector2<f64>,
    pub velocity: Vector2<f64>,
    pub move_time_left: f64,

    pub scale: f64,
}


impl MapObject {
    
    pub fn move_dt(&mut self, dt: f64) {
        if self.move_time_left > 0.0 {
            let distance = vec2_scale(self.velocity, dt);
            self.position = vec2_add(self.position, distance);
            self.move_time_left -= dt;
        }
    }
}


pub struct MapObjectFactory {
    next_id: u64,
}


impl MapObjectFactory {

    pub fn create_mob(&mut self, tile_id: usize, tileset_id: usize, position: Vector2<f64>, scale: f64) -> MapObject {

        let visual = Visual {
            base_image_id: tile_id,
            current_image_id: tile_id,
            frames: 8,
            tileset_id,
            color: [1.0, 1.0, 1.0, 1.0],
        };

        let attributes = MobAttributes {
            speed: 150.0,
        };

        let uid = self.next_id;
        self.next_id += 1;

        MapObject {
            uid,
            visual,
            attributes,
            item: None,

            position, 
            velocity: [0.0, 0.0],
            move_time_left: 0.0,
            scale,
        }
    }
}


pub struct Visual {
    pub base_image_id: usize,
    pub current_image_id: usize,
    pub frames: usize,
    pub tileset_id: usize,
    pub color: [f32; 4],
}


impl Visual {
    pub fn orient(&self, dx: f64, dy: f64) -> usize {
        let frames = self.frames;
        let mut result = 0;

        if dx != 0.0 && dy != 0.0 {
            // calculate facing
            let mut r = dy.atan2(dx);
            
            // round to a segment
            r = r + PI + PI / frames as f64;
        
            // calculate tile offsets from 0 to frames-1

            let f = (r * frames as f64)  / (PI * 2.0) - 0.5;

            result = frames/2 + f.floor() as usize;

            if result >= frames {
                result = result - frames;
            }

            println!("dx={} dy={} r={} frames={}", dx, dy, result, frames);
        } 
        else {
            // error case, zero length move
            println!("Error: Cannot orient mob by zero length direction");
        }

        result
    }
}


pub struct MobAttributes {
    pub speed: f64,
}


