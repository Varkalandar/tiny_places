use std::collections::HashMap;

use rand::Rng;
use rand::rngs::StdRng;
use vecmath::Vector2;

use crate::map::MapObject;
use crate::map::MapObjectFactory;
use crate::map::move_mob;
use crate::game::fire_projectile;


pub struct MobGroup {

    // Group center x and y - the group should move as a whole
    center: Vector2<f64>,

    members: Vec<MobGroupMember>,
}

pub struct MobGroupMember {
    id: u64,

    // seconds till next action
    action_countdown: f64,
    mobile: bool,
}


impl MobGroup {

    pub fn new(mobs: Vec<u64>, center: Vector2<f64>, mobile: bool, rng: &mut StdRng) -> MobGroup {

        let mut members = Vec::with_capacity(mobs.len());

        for id in mobs {
            members.push(MobGroupMember {
                id,
                action_countdown: 0.1 + rng.gen::<f64>(),
                mobile,
            });
        }

        MobGroup {
            center,
            members,
        }
    }


    pub fn update(&mut self, player_id: u64, dt: f64, mobs: &mut HashMap<u64, MapObject>, rng: &mut StdRng, factory: &mut MapObjectFactory) {
            
        let player_position = mobs.get(&player_id).unwrap().position;

        for member in &mut self.members {

            let mob = mobs.get_mut(&member.id).unwrap();

            member.action_countdown -= dt;

            if member.action_countdown < 0.0 {

                // fire at a player?
                if rng.gen::<f64>() < 0.25 {

                    // world.speaker.play_sound(Sound::FireballLaunch);

                    let projectile = fire_projectile(mob.position, 25, player_position, 200.0, factory);
                    mobs.insert(projectile.uid, projectile);

                    member.action_countdown = 1.0 + rng.gen::<f64>();
                }
                else if member.mobile {
                    
                    // move
                    let mut count = 0;
                    let mut x;
                    let mut y;

                    loop {
                        x = mob.position[0] + 100.0 - rng.gen::<f64>() * 200.0;
                        y = mob.position[1] + 100.0 - rng.gen::<f64>() * 200.0;

                        let dx = x - self.center[0];
                        let dy = y - self.center[1];

                        let len = dx * dx + dy * dy;
                        count += 1;

                        // System.err.println("len=" + len);

                        if len < 100.0 * 100.0 || count >= 5 { break; }
                    } 

                    if count >= 5 {
                        x = self.center[0] + 50.0 - rng.gen::<f64>() * 100.0;
                        y = self.center[1] + 50.0 - rng.gen::<f64>() * 100.0;
                    }

                    // System.err.println("id=" + creature.id + "moves to " + x + ", " + y);

                    move_mob(mob, [x, y], mob.attributes.base_speed);
                    
                    member.action_countdown = 3.0 + rng.gen::<f64>() * 2.0;
                }
            }
        }
    }
}
