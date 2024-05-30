use vecmath::{Vector2, vec2_sub, vec2_add, vec2_scale, vec2_normalized};
use std::f64::consts::PI;

use std::io::prelude::*;
use std::io::{Result, BufWriter};
use std::fs::File;
use std::path::PathBuf;
use std::collections::HashMap;

use crate::item::Item;
use crate::inventory::Inventory;

pub const MAP_GROUND_LAYER:usize = 0;
pub const MAP_OBJECT_LAYER:usize = 1;
pub const MAP_CLOUD_LAYER:usize = 2;


pub struct Map {
    pub layers: [HashMap<u64, MapObject>; 7],

    // all items on this map
    pub items: Inventory,

    pub has_selection: bool,
    pub selected_item: u64,
    pub selected_layer: usize,

    pub backdrop_image_name: String,

    pub factory: MapObjectFactory,

    pub player_id: u64, 
}


impl Map {
    pub fn new(backdrop_image_name: &str) -> Map {
        let mut layers = [HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new(),];
        
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
        let player_id = player.uid;
        player.visual = player_visual;

        layers[MAP_OBJECT_LAYER].insert(player.uid, player);

        Map {
            layers,
            items: Inventory::new(),
            has_selection: false,
            selected_item: 0,
            selected_layer: 0,

            backdrop_image_name: backdrop_image_name.to_string(),
        
            factory,
            player_id,
        }
    }


    pub fn player_position(&self) -> Vector2<f64> {

        let mob = self.layers[MAP_OBJECT_LAYER].get(&self.player_id).unwrap();
        return mob.position;
    }


    pub fn find_nearest_object(&self, layer: usize, position: &Vector2<f64>) -> Option<u64> {
        let objects = &self.layers[layer];
        let mut distance = 999999.0;
        let mut best_id = 0;

        for (_key, object) in objects {
            let dx = object.position[0] - position[0];
            let dy = object.position[1] - position[1];
            let d2 = dx * dx + dy * dy;

            println!("object {} has distance {}", object.uid, d2);

            if d2 < distance {
                distance = d2;
                best_id = object.uid;
            }
        }

        let mut result:Option<u64> = None;

        if distance < 10000.0 {
            result = Some(best_id);
            println!("  best object is {}", best_id);
        }

        result
    }


    pub fn fire_projectile(&mut self, shooter_id: u64, layer: usize, projectile_type: usize, fire_at: Vector2<f64>, speed: f64) {
        println!("Adding projectile with type {} fired at {:?}", projectile_type, fire_at);
    
        let shooter = &self.layers[layer][&shooter_id];
        let np = vec2_sub(fire_at, shooter.position);
    
        let dir = vec2_normalized(np);
        let velocity = vec2_scale(dir, speed);

        let start_pos = vec2_add(shooter.position, vec2_scale(dir, 80.0));

        let mut projectile = self.factory.create_mob(projectile_type, 5, start_pos, 1.0);
        projectile.velocity = velocity;
        projectile.move_time_left = 2.0;
        projectile.action = MoveEndAction::RemoveFromMap;

        let offset = projectile.visual.orient(velocity[0], velocity[1]);
        projectile.visual.current_image_id = projectile.visual.base_image_id + offset;

        self.layers[layer].insert(projectile.uid, projectile);
    }    


    pub fn update(&mut self, dt: f64) {

        let mut kill_list = Vec::new();

        for (_key, mob) in &mut self.layers[MAP_OBJECT_LAYER] {
            let before = mob.move_time_left;
            mob.move_dt(dt);
            let after = mob.move_time_left;

            // did the move just end?
            if before > 0.0 && after <= 0.0 {
                if mob.action == MoveEndAction::RemoveFromMap {
                    kill_list.push(mob.uid);
                }
            }
        }

        for id in kill_list {
            self.layers[MAP_OBJECT_LAYER].remove(&id);
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

            let mut mob = self.factory.create_mob(tile_id, layer, [x, y], scale);
            mob.visual.color = color;
            self.layers[layer].insert(mob.uid, mob);
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

        for (_key, object) in objects {
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
            let object = self.layers[self.selected_layer].get_mut(&self.selected_item).unwrap();
            object.position[0] += dx;
            object.position[1] += dy;
        }
    }

}


pub struct MapObject {

    pub uid: u64,
    pub visual: Visual,
    pub attributes: MobAttributes,
    pub item: Option<Item>,
    pub action: MoveEndAction,

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

        println!("MapObjectFactory: next id will be {}", self.next_id);

        MapObject {
            uid,
            visual,
            attributes,
            item: None,
            action: MoveEndAction::None,
            position, 
            velocity: [0.0, 0.0],
            move_time_left: 0.0,
            scale,
        }
    }
}

#[derive(PartialEq)]
pub enum MoveEndAction {
    None,
    RemoveFromMap,
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
            r = r + PI + PI * 2.0 / frames as f64;
        
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
