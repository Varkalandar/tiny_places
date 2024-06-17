use crate::map::MapObject;
use crate::map::UpdateAction;

pub trait Animated {
    fn update(&self, _dt: f64, _mob: &mut MapObject) {

    }
}

pub struct NoAnimation {

}

impl Animated for NoAnimation {

}


pub struct SpinAnimation {
    speed: f64,
}

impl SpinAnimation {
    pub fn new(speed: f64) -> SpinAnimation {
        SpinAnimation {
            speed,
        }
    }
}

impl Animated for SpinAnimation {
    fn update(&self, dt: f64, mob: &mut MapObject) {
        mob.animation_timer += dt;

        let frame = (mob.animation_timer * self.speed) as usize;
        mob.visual.current_image_id = mob.visual.base_image_id + (frame % 8);
    }
}


pub struct RemovalAnimation {
    time_limit: f64, 
}

impl RemovalAnimation {
    pub fn new(time_limit: f64) -> RemovalAnimation {
        RemovalAnimation {
            time_limit,
        }
    }
}

impl Animated for RemovalAnimation {
    fn update(&self, dt: f64, mob: &mut MapObject) {
        mob.animation_timer += dt;

        // println!("Time left: {}", self.time_limit - mob.animation_timer);

        if mob.animation_timer > self.time_limit {
            mob.update_action = UpdateAction::RemoveFromMap;
        }
    }

}
