use vecmath::{Vector2, vec2_sub, vec2_add, vec2_scale, vec2_len, vec2_square_len};
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
use crate::mob_group::MobGroup;
use crate::CREATURE_TILESET;


pub const MAP_GROUND_LAYER:usize = 0;
pub const MAP_OBJECT_LAYER:usize = 1;
pub const MAP_CLOUD_LAYER:usize = 2;


pub struct Map {

    pub layers: [HashMap<u64, MapObject>; 7],
    pub animations: HashMap<u64, Box<dyn Animated>>,
    pub transitions: Vec<MapTransition>,
    
    // 'AI' controlled objects
    pub mob_groups: Vec<MobGroup>,

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
            mob_groups: Vec::new(),

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

        {
            let groups = &mut self.mob_groups;
            let mobs = &mut self.layers[MAP_OBJECT_LAYER];
            let factory = &mut self.factory;

            for group in groups {
                group.update(self.player_id, dt, mobs, rng, factory);
            }
        }


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

            // particle stuff
            {
                let particles = &mut mob.visual.particles;
                let len = particles.spawn_ids.len();

                if len > 0 {
                    let chance = particles.spawn_chance * dt;
                    if rng.gen::<f64>() < chance {
                        let spark = particles.spawn_ids[rng.gen_range(0..len)];
                        
                        particles.add_particle(0.0, -400.0, 0.0, 0.0, 0.0, 0.0, 
                                               0.1, spark, [0.7, 0.75, 0.9]);
                    }
                }

                particles.drive(dt);
            }


            let animation_opt = self.animations.get(&mob.uid);
            match animation_opt {
                None => {
                },
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

            let mob_type = mob.attributes.mob_type;
            if mob_type == MobType::PlayerProjectile || mob_type == MobType::CreatureProjectile {

                // projectiles may have hit something in the move
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

            // some projectiles can only hit certain targets, check if the hit was valid
            let valid = self.handle_projectile_hit(projectile, target, rng, speaker);

            if valid {
                speaker.play_sound(Sound::FireballHit);
                kill_list.push(projectile);
                
                let start_time = self.layers[MAP_OBJECT_LAYER].get(&target).unwrap().animation_timer;
                self.animations.insert(target, Box::new(RemovalAnimation::new(start_time, 0.7)));
            }
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


    fn handle_projectile_hit(&mut self, projectile_uid: u64, target_uid: u64, rng: &mut StdRng, speaker: &mut SoundPlayer) -> bool {

        let projectile_type = self.layers[MAP_OBJECT_LAYER].get_mut(&projectile_uid).unwrap().attributes.mob_type;
        let target = self.layers[MAP_OBJECT_LAYER].get_mut(&target_uid).unwrap();

        // projectiles can only hit "the enemy" or obstacles on the map
        if projectile_type == MobType::PlayerProjectile &&
            target.attributes.mob_type == MobType::Player {
            // no, we do not shoot ourselves into the foot
            return false;
        }

        if projectile_type == MobType::CreatureProjectile && 
           target.attributes.mob_type == MobType::Creature {
            return false;
        }

        // println!("Handle projectile hit, adding particles");
        let sparks = [403, 404, 1993, 1994, 1995, 1996, 1997];

        let z_off = target.visual.height * target.scale * 0.5;

        if projectile_type == MobType::PlayerProjectile && 
           target.attributes.mob_type == MobType::Creature &&
           target.attributes.hit_points > 0 {
            for _i in 0..10 {
                let xv = rng.gen::<f64>() * 2.0 - 1.0;
                let yv = rng.gen::<f64>() * 2.0 - 1.0;
                let zv = rng.gen::<f64>();
                let speed = 40.0;
                let color = [0.8 + rng.gen::<f32>() * 0.4, 0.5 + rng.gen::<f32>() * 0.4, 0.1 + rng.gen::<f32>() * 0.4];
                let tile = sparks[rng.gen_range(0..sparks.len())];

                let speed = if tile == 403 {100.0} else {100.0 + rng.gen_range(1.0..50.0)};

                target.visual.particles.add_particle(0.0, 0.0, z_off, xv * speed, yv * speed, zv * speed, 0.7, tile, color);
                target.visual.color = [0.0, 0.0, 0.0, 0.0];
                
                let damage = 10; // todo
                target.attributes.hit_points -= damage;
            }
        
            return true;
        }

        // don't kill the player (yet)
        false
    }


    pub fn populate(&mut self, _filename: &str, rng: &mut StdRng) {

        let position = [1216.0, 1448.0];

        let group = self.make_creature_group(41, 5, 9, position, 40.0, rng);
        self.mob_groups.push(group);
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


    pub fn make_creatures(&mut self, id: usize, min_count: i32, max_count: i32, center: Vector2<f64>, spacing: f64, scale: f64, rng: &mut StdRng) -> Vec<MapObject> {

        let count = rng.gen_range(min_count ..= max_count) as usize;
        
        let mut list: Vec<MapObject> = Vec::with_capacity(count);
    
        for i in 0 .. count {
            let mut tries = 0;
            
            // don't place mobs in the same spot if possible
            // 10 tries will be made to find a clear spot
            loop {
                let x = center[0] + spacing * (rng.gen::<f64>() * 10.0 - 5.0);
                let y = center[1] + spacing * (rng.gen::<f64>() * 10.0 - 5.0);
    
                let mut ok = true;
                for mob in &list {

                    let dx = mob.position[0] - x;
                    let dy = mob.position[1] - y;
                    let d = dx * dx + dy * dy;
                    
                    // must be at least 20 units from each other
                    ok = d > 20.0 * 20.0;
                }

                tries += 1;

                if ok {
                    let mut mob = self.factory.create_mob(id, CREATURE_TILESET, [x, y], 32.0, scale);
                    mob.attributes.mob_type = MobType::Creature;
                    mob.attributes.hit_points = 1;
                    mob.animation_timer = rng.gen::<f64>(); // otherwise all start with the very same frame
                    list.push(mob);

                    break; 
                }
                
                if tries > 10 { 
                    break; 
                }
            }
        }
    
        list
    }

    
    pub fn make_creature_group(&mut self, id: usize, min_count: i32, max_count: i32, center: Vector2<f64>, spacing: f64, rng: &mut StdRng) -> MobGroup {
        
        let mut mobs = self.make_creatures(id, min_count, max_count, center, spacing, 0.5, rng);
        let mut list = Vec::new();

        for i in (0..mobs.len()).rev() {
            let mob = mobs.remove(i);
            let id = mob.uid;

            self.layers[MAP_OBJECT_LAYER].insert(id, mob);
            list.push(id);
        
            self.animations.insert(id, Box::new(SpinAnimation::new(24.0)));
        }

        MobGroup::new(list, center, true, rng)
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


pub fn move_mob(mob: &mut MapObject, destination: Vector2<f64>, base_speed: f64) {

    let direction = vec2_sub(destination, mob.position);

    println!("creature {} moves in direction {:?}", mob.uid, direction);

    let distance = vec2_len(direction);
    let time = distance / base_speed; // pixel per second

    mob.move_time_left = time;
    mob.velocity = vec2_scale(direction, 1.0/time);

    let d = mob.visual.orient(direction[0], direction[1]);
    mob.visual.current_image_id = mob.visual.base_image_id + d;
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
            hit_points: 0,
            mob_type: MobType::MapObject,
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

#[derive(PartialEq, Clone, Copy)]
pub enum MobType {
    MapObject,
    Player,
    Creature,
    PlayerProjectile,
    CreatureProjectile,
}


pub struct MobAttributes {
    pub base_speed: f64,
    pub hit_points: i32,
    pub mob_type: MobType,
}


pub struct MapTransition {

    // entrance location
    from: Vector2<f64>,
    // catchment area
    rad: f64,
    // destination map
    to: i32,
}