use vecmath::{Vector2, vec2_sub, vec2_add, vec2_scale, vec2_normalized, vec2_square_len};
use std::f64::consts::PI;

use std::io::prelude::*;
use std::io::{Result, BufWriter};
use std::fs::File;
use std::path::PathBuf;
use std::collections::HashMap;
use std::boxed::Box;

use rand::Rng;
use rand::rngs::StdRng;

use piston_window::draw_state::Blend;

use crate::item::Item;
use crate::inventory::Inventory;
use crate::particle_driver::ParticleDriver;
use crate::animation::*;
use crate::sound::Sound;
use crate::SoundPlayer;


pub const MAP_GROUND_LAYER:usize = 0;
pub const MAP_OBJECT_LAYER:usize = 1;
pub const MAP_CLOUD_LAYER:usize = 2;


pub struct Map {

    pub layers: [HashMap<u64, MapObject>; 7],
    pub animations: HashMap<u64, Box<dyn Animated>>,
    pub transitions: Vec<MapTransition>,

    // all items on this map
    pub items: Inventory,

    pub has_selection: bool,
    pub selected_item: u64,
    pub selected_layer: usize,

    pub name: String,
    pub map_image_name: String,
    pub backdrop_image_name: String,

    pub factory: MapObjectFactory,

    pub player_id: u64, 
}


impl Map {
    pub fn new(name: &str, map_image_name: &str, backdrop_image_name: &str) -> Map {
        let mut layers = [HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new(),];
        
        let player_visual = Visual {
            base_image_id: 39,
            tileset_id: 4,
            current_image_id: 39,
            frames: 16, 
            height: 24.0,
            color: [1.0, 1.0, 1.0, 1.0],
            blend: Blend::Alpha,
            particles: ParticleDriver::new(),       
        };

        let mut factory = MapObjectFactory {
            next_id: 1,
        };

        let mut player = factory.create_mob(39, 4, [1000.0, 1000.0], 24.0, 1.0);
        let player_id = player.uid;
        player.visual = player_visual;
        player.update_action = UpdateAction::EmitDriveParticles;

        layers[MAP_OBJECT_LAYER].insert(player.uid, player);

        Map {
            layers,

            animations: HashMap::new(),
            transitions: Vec::new(),
            
            items: Inventory::new(),
            has_selection: false,
            selected_item: 0,
            selected_layer: 0,

            name: name.to_string(),
            map_image_name: map_image_name.to_string(),
            backdrop_image_name: backdrop_image_name.to_string(),
        
            factory,
            player_id,
        }
    }


    pub fn player_position(&self) -> Vector2<f64> {

        let mob = self.layers[MAP_OBJECT_LAYER].get(&self.player_id).unwrap();
        return mob.position;
    }


    pub fn find_nearest_object(&self, layer: usize, position: &Vector2<f64>, search_radius: f64, ignore_uid: u64) -> Option<u64> {
        let objects = &self.layers[layer];
        let mut distance = search_radius * search_radius;
        let mut best_id = 0;

        for (_key, object) in objects {
            let dx = object.position[0] - position[0];
            let dy = object.position[1] - position[1];
            let d2 = dx * dx + dy * dy;

            // println!("object {} has distance {}", object.uid, d2);

            if d2 < distance && object.uid != ignore_uid {
                distance = d2;
                best_id = object.uid;
            }
        }

        let mut result:Option<u64> = None;

        if distance < search_radius * search_radius {
            result = Some(best_id);
            // println!("  best object is {}", best_id);
        }

        result
    }


    pub fn update(&mut self, dt: f64, rng: &mut StdRng, speaker: &mut SoundPlayer) {

        let mut kill_list = Vec::new();
        let mut phit_list = Vec::new();

        for (_key, mob) in &mut self.layers[MAP_OBJECT_LAYER] {
            let before = mob.move_time_left;
            mob.move_dt(dt);
            let after = mob.move_time_left;

            // did the move just end?
            if before > 0.0 && after <= 0.0 {
                mob.visual.particles.clear();

                if mob.move_end_action == MoveEndAction::RemoveFromMap {
                    kill_list.push(mob.uid);
                }
            }

            mob.visual.particles.drive(dt);

            let animation_opt = self.animations.get(&mob.uid);
            match animation_opt {
                None => {},
                Some(animation) => {
                    animation.update(dt, mob);
                }
            }

            // must this mob be removed from the map?
            if mob.update_action == UpdateAction::RemoveFromMap {
                kill_list.push(mob.uid);
            }
            else if mob.update_action == UpdateAction::EmitDriveParticles && after > 0.0 {
                emit_drive_particles(mob, dt, rng);
            }
        }

        for (_key, mob) in &self.layers[MAP_OBJECT_LAYER] {

            // projectiles may have hit something in the move
            if mob.attributes.is_projectile {
                let target = self.find_nearest_object(MAP_OBJECT_LAYER, &mob.position, 80.0, mob.uid);
                match target {
                    None => {}
                    Some(uid) => {
                        phit_list.push((mob.uid, uid));                        
                    }
                }
            }
        }

        for (projectile, target) in phit_list {

            self.handle_projectile_hit(projectile, target, rng, speaker);
            kill_list.push(projectile);

            self.animations.insert(target, Box::new(RemovalAnimation::new(0.7)));
        }

        for id in kill_list {
            self.layers[MAP_OBJECT_LAYER].remove(&id);
            self.animations.remove(&id);
        }
    }


    pub fn check_player_transition(&mut self) -> bool {
        let player = self.layers[MAP_OBJECT_LAYER].get(&self.player_id).unwrap();
        let pos = player.position;
        let mut best_map = -1;
        
        for transit in &self.transitions {
            let v = vec2_sub(pos, transit.from);
            let d = vec2_square_len(v);
            if d < transit.rad * transit.rad {
                best_map = transit.to;
            }
        }

        if best_map >= 0 {
            self.load("warmup.map");
            return true;
        }

        false
    }


    fn handle_projectile_hit(&mut self, projectle_uid: u64, target_uid: u64, rng: &mut StdRng, speaker: &mut SoundPlayer) {

        speaker.play_sound(Sound::FireballHit);

        let target = self.layers[MAP_OBJECT_LAYER].get_mut(&target_uid).unwrap();

        println!("Handle projectile hit, adding particles");
        let sparks = [403, 404, 1993, 1994, 1995, 1996, 1997];

        let z_off = target.visual.height * target.scale * 0.5;

        for _i in 0..100 {
            let xv = rng.gen::<f64>() * 2.0 - 1.0;
            let yv = rng.gen::<f64>() * 2.0 - 1.0;
            let zv = rng.gen::<f64>();
            let speed = 100.0;
            let color = [0.8 + rng.gen::<f32>() * 0.4, 0.5 + rng.gen::<f32>() * 0.4, 0.1 + rng.gen::<f32>() * 0.4];
            let tile = sparks[rng.gen_range(0..sparks.len())];

            let speed = if tile == 403 {100.0} else {100.0 + rng.gen_range(1.0..50.0)};

            target.visual.particles.add_particle(0.0, 0.0, z_off, xv * speed, yv * speed, zv * speed, 0.7, tile, color);
            target.visual.color = [0.0, 0.0, 0.0, 0.0];            
        }
    }


    pub fn load(&mut self, filename: &str) {

        // preserve player
        let mut player = self.layers[MAP_OBJECT_LAYER].remove(&self.player_id).unwrap();

        for layer in &mut self.layers {
            layer.clear();
        }
        self.transitions.clear();

        let mut path = PathBuf::new();
        path.push("maps");
        path.push(filename);

        let content = std::fs::read_to_string(path.as_path()).unwrap();
        let mut lines = content.lines();

        lines.next(); // version

        lines.next(); // header start
        self.name = lines.next().unwrap().to_string();
        self.map_image_name = lines.next().unwrap().to_string();
        self.backdrop_image_name = lines.next().unwrap().to_string();
        println!("map name={} image={} backdrop={}", self.name, self.map_image_name, self.backdrop_image_name);
        lines.next(); // header end

        lines.next(); // objects start
        let mut line = lines.next().unwrap();

        let object_end_marker = "end map objects".to_string();
        while object_end_marker != line {
            println!("line='{}'", line);
            self.load_mob(line);
            line = lines.next().unwrap();
        }

        lines.next(); // transitions start
        line = lines.next().unwrap();

        let transition_end_marker = "end map transitions".to_string();
        while transition_end_marker != line {
            println!("line='{}'", line);
            self.load_transition(line);
            line = lines.next().unwrap();
        }

        println!("player_id={}", self.player_id);

        // stop player movement
        player.move_time_left = 0.0;
        self.layers[MAP_OBJECT_LAYER].insert(self.player_id, player);
    }


    fn load_mob(&mut self, line: &str) {
        let parts: Vec<&str> = line.split(",").collect();

        let layer = parts[0].parse::<usize>().unwrap();
        let tile_id = parts[1].parse::<usize>().unwrap();
        let frames = parts[2].parse::<usize>().unwrap();

        let x = parts[3].parse::<f64>().unwrap();
        let y = parts[4].parse::<f64>().unwrap();
        let height = parts[5].parse::<f64>().unwrap();
        let scale = parts[6].parse::<f64>().unwrap();

        // parts[7] is an RGBA tuple
        let mut color_in = parts[7].split(" ");

        let mut color: [f32; 4] = [0.0; 4];
        for i in 0..4 {
            color[i] = color_in.next().unwrap().parse::<f32>().unwrap();
        }

        let blend = key_to_blend(parts[8]);

        println!("{}, {}, {}, {}, {}, {}, {:?}, {:?}", layer, tile_id, x, y, height, scale, color, blend);

        let mut mob = self.factory.create_mob(tile_id, layer, [x, y], height, scale);
        mob.visual.color = color;
        mob.visual.blend = blend;
        mob.visual.frames = frames;

        self.layers[layer].insert(mob.uid, mob);
    }


    fn load_transition(&mut self, line: &str) {
        let mut parts = line.split(",");

        let x = parts.next().unwrap().parse::<f64>().unwrap();
        let y = parts.next().unwrap().parse::<f64>().unwrap();
        let r = parts.next().unwrap().parse::<f64>().unwrap();
        let map_id = parts.next().unwrap().parse::<i32>().unwrap();

        self.transitions.push(MapTransition {
            from: [x, y],
            rad: r,
            to: map_id,
        });
    }


    pub fn save(&self, filename: &str) -> Result<()> {
        let mut path = PathBuf::new();
        path.push("maps");
        path.push(filename);

        let f = File::create(path.as_path())?;
        {        
            let mut writer = BufWriter::new(f);

            writer.write("v10\n".as_bytes())?;
            
            writer.write("begin map header\n".as_bytes())?;
            let name = self.name.to_string()  + "\n";
            writer.write(name.as_bytes())?;
            let map_image_name = self.map_image_name.to_string() + "\n";
            writer.write(map_image_name.as_bytes())?;
            let backdrop_image_name = self.backdrop_image_name.to_string() + "\n";
            writer.write(backdrop_image_name.as_bytes())?;
            writer.write("end map header\n".as_bytes())?;

            writer.write("begin map objects\n".as_bytes())?;
            self.save_layer(&mut writer, MAP_GROUND_LAYER)?;
            self.save_layer(&mut writer, MAP_OBJECT_LAYER)?;
            self.save_layer(&mut writer, MAP_CLOUD_LAYER)?;
            writer.write("end map objects\n".as_bytes())?;

            self.save_map_transitions(&mut writer)?
        }

        Ok(())
    }
    
    
    fn save_layer(&self, writer: &mut BufWriter<File>, layer: usize) -> Result<()> {
        let objects = &self.layers[layer];

        for (_key, object) in objects {

            if object.uid != self.player_id {

                let color = object.visual.color; 

                let line = 
                layer.to_string() + "," +
                &object.visual.base_image_id.to_string() + "," +
                &object.visual.frames.to_string() + "," +
                &object.position[0].to_string() + "," +
                &object.position[1].to_string() + "," +
                &object.visual.height.to_string() + "," +
                &object.scale.to_string() + "," +
                &color[0].to_string() + " " +
                &color[1].to_string() + " " +
                &color[2].to_string() + " " +
                &color[3].to_string() + "," +            
                &blend_to_key(object.visual.blend) +
                "\n";
                
                writer.write(line.as_bytes())?;
            }
        }

        Ok(())
    }

    
    fn save_map_transitions(&self, writer: &mut BufWriter<File>) -> Result<()> {
        writer.write("begin map transitions\n".as_bytes())?;

        for transit in &self.transitions {
            let line = 
                transit.from[0].to_string() + "," +
                &transit.from[1].to_string() + "," +
                &transit.rad.to_string() + "," +
                &transit.to.to_string() + "\n";
            writer.write(line.as_bytes())?;
        }

        writer.write("end map transitions\n".as_bytes())?;

        Ok(())
    }

    
    pub fn move_selected_object(&mut self, dx: f64, dy: f64) {        
        if self.has_selection {
            let object = self.layers[self.selected_layer].get_mut(&self.selected_item).unwrap();
            object.position[0] += dx;
            object.position[1] += dy;
        }
    }

    pub fn apply_to_selected_mob<F>(&mut self, func: F)
        where F: FnOnce(&mut MapObject) {
        let mob = self.layers[self.selected_layer].get_mut(&self.selected_item);

        match mob {
            None => {}
            Some(mob) => { func(mob); }
        }
    }
}

fn emit_drive_particles(mob: &mut MapObject, dt: f64, rng: &mut StdRng) {

    let direction = vec2_scale(mob.velocity, -1.0);
    let rad = 0.5;

    let chance_per_second = 20.0;
    let chance = chance_per_second * dt;

    if rng.gen::<f64>() < chance {
        let xp = direction[0] * rad + direction[1] * (rng.gen::<f64>() * 2.0 - 1.0) * 0.15;
        let yp = direction[1] * rad + direction[0] * (rng.gen::<f64>() * 2.0 - 1.0) * 0.15;

        let xv = direction[0] + rng.gen::<f64>() * 2.0 - 1.0;
        let yv = direction[1] + rng.gen::<f64>() * 2.0 - 1.0;
        let zv = (rng.gen::<f64>() *2.0 - 1.0) * 0.15;
        let speed = 1.0;

        let spark = 1993 + (rng.gen::<f64>() * 5.0) as usize;

        mob.visual.particles.add_particle(xp, yp, 25.0, xv * speed, yv * speed, zv * speed, 1.0, spark, [0.5, 0.8, 1.0]);
    }
}


fn blend_to_key(blend: Blend) -> String {
    let key =
        match blend {
            Blend::Alpha => {"n"}, 
            Blend::Add => {"a"},
            Blend::Lighter => {"l"},
            Blend::Multiply => {"m"},
            Blend::Invert => {"i"},
        };

    key.to_string()
}


fn key_to_blend(key: &str) -> Blend {

    println!("key='{}'", key);

    if key == "n" {
        Blend::Alpha
    } else if key == "a" {
        Blend::Add
    } else if key == "l" {
        Blend::Lighter
    } else if key == "m" {
        Blend::Multiply
    } else if key == "i" {
        Blend::Invert
    } else {
        Blend::Alpha
    }
}


pub struct MapObject {

    pub uid: u64,
    pub visual: Visual,
    pub attributes: MobAttributes,
    pub item: Option<Item>,

    // world coordinates of this object. Note that screen coordinates are different
    pub position: Vector2<f64>,
    pub velocity: Vector2<f64>,
    pub move_time_left: f64,

    pub scale: f64,

    pub move_end_action: MoveEndAction,
    pub update_action: UpdateAction,
    pub animation_timer: f64,
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

    pub fn create_mob(&mut self, tile_id: usize, tileset_id: usize, position: Vector2<f64>, height: f64, scale: f64) -> MapObject {

        let visual = Visual {
            base_image_id: tile_id,
            current_image_id: tile_id,
            frames: 8,
            tileset_id,
            height,
            color: [1.0, 1.0, 1.0, 1.0],
            blend: Blend::Alpha,
            particles: ParticleDriver::new(),
        };

        let attributes = MobAttributes {
            base_speed: 150.0,
            is_projectile: false,
        };

        let uid = self.next_id;
        self.next_id += 1;

        println!("MapObjectFactory: next id will be {}", self.next_id);

        MapObject {
            uid,
            visual,
            attributes,
            item: None,

            position, 
            velocity: [0.0, 0.0],
            move_time_left: 0.0,
            scale,

            move_end_action: MoveEndAction::None,
            update_action: UpdateAction::None,
            animation_timer: 0.0,
        }
    }
}


#[derive(PartialEq)]
pub enum MoveEndAction {
    None,
    RemoveFromMap,
}


#[derive(PartialEq)]
pub enum UpdateAction {
    None,
    RemoveFromMap,
    EmitDriveParticles,
}


pub struct Visual {
    pub base_image_id: usize,
    pub current_image_id: usize,
    pub frames: usize,
    pub tileset_id: usize,
    pub height: f64,
    pub color: [f32; 4],
    pub blend: Blend,
    pub particles: ParticleDriver,
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
    pub base_speed: f64,
    pub is_projectile: bool,
}


pub struct MapTransition {

    // entrance location
    from: Vector2<f64>,
    // catchment area
    rad: f64,
    // destination map
    to: i32,
}