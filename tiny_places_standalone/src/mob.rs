use vecmath::{vec2_add, vec2_scale, Vector2};

pub struct Mob {
    // world coordinates of this mob. Note that screen coordinates are different
    pub position: Vector2<f64>,
    pub speed: Vector2<f64>,
    pub move_over_time: f64,
    
    // measured in pixel per second
    pub base_speed: f64,
}


impl Mob {  
    pub fn new(x: f64, y: f64) -> Mob {
        Mob {
            position: [x, y],
            speed: [0.0, 0.0],
            move_over_time: 0.0,
            base_speed: 150.0,
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