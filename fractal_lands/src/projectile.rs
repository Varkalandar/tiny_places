use vecmath::Vector2;
use std::collections::HashMap;
use piston_window::draw_state::Blend;

use crate::read_lines;
use crate::parse_rgba;
use crate::map::Visual;
use crate::SoundPlayer;


pub struct ProjectileBuilder {
    projectile_data: HashMap <String, ProjectileConfig>
}


pub struct ProjectileConfig {
    base_tile_id: usize,
    directions: usize,
    phases: usize,
    glow: [f32;4],
    sound: usize,
    volume: f32,
}


impl ProjectileBuilder {

    pub fn new() -> ProjectileBuilder {
        let projectile_data = read_projectile_config();

        ProjectileBuilder {
            projectile_data
        }
    }

    pub fn configure_projectile(&self, key: &str, visual: &mut Visual, direction: Vector2<f64>, speaker: &mut SoundPlayer) {
        let pd = self.projectile_data.get(&key.to_string()).unwrap();

        speaker.play_sound(pd.sound, pd.volume);

        visual.base_image_id = pd.base_tile_id;
        visual.directions = pd.directions;
        visual.phases = pd.phases;
        visual.glow = pd.glow;
        visual.blend = Blend::Add;
        visual.orient_in_direction(direction);
    }
}


fn read_projectile_config() -> HashMap <String, ProjectileConfig> {

    let lines = read_lines("resources/creatures/projectiles.csv");
    let mut projectiles = HashMap::new();

    for i in 1..lines.len() {
        let mut parts = lines[i].split(",");

        let name = parts.next().unwrap().to_string();

        projectiles.insert(name, 
            ProjectileConfig {
                base_tile_id: parts.next().unwrap().parse::<usize>().unwrap(),
                directions: parts.next().unwrap().parse::<usize>().unwrap(),
                phases: parts.next().unwrap().parse::<usize>().unwrap(),
                glow: parse_rgba(parts.next().unwrap()),
                sound: parts.next().unwrap().parse::<usize>().unwrap(),
                volume: parts.next().unwrap().parse::<f32>().unwrap(),
            });
    }

    projectiles
}
