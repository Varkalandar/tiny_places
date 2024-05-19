use vecmath::{vec2_add, vec2_scale, Vector2};
use std::f64::consts::PI;

pub struct Visual {
    pub base_image_id: usize,
    pub current_image_id: usize,
    pub frames: usize,
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

pub struct Mob {
    // world coordinates of this mob. Note that screen coordinates are different
    pub position: Vector2<f64>,
    pub speed: Vector2<f64>,
    pub move_over_time: f64,
    
    // measured in pixel per second
    pub base_speed: f64,

    pub visual: Visual
}


impl Mob {  
    pub fn new(x: f64, y: f64) -> Mob {
        let visual = Visual {
            base_image_id: 0,
            current_image_id: 0,
            frames: 8,
        };

        Mob {
            position: [x, y],
            speed: [0.0, 0.0],
            move_over_time: 0.0,
            base_speed: 150.0,

            visual,
        }        
    }
    
    
    pub fn move_by_time(&mut self, dt: f64) {
        if self.move_over_time > 0.0 {
            let distance = vec2_scale(self.speed, dt);
            self.position = vec2_add(self.position, distance);
            self.move_over_time -= dt;
        }
    }
}