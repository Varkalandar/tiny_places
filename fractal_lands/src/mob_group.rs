use rand::rngs::StdRng;
use vecmath::Vector2;


pub struct MobGroup {

    // Group center x and y - the group should move as a whole
    center: Vector2<f64>,

    members: Vec<u64>,
}


impl MobGroup {

    pub fn new(mobs: Vec<u64>, center: Vector2<f64>) -> MobGroup {

        let mut members = Vec::with_capacity(mobs.len());

        for id in mobs {
            members.push(id);
        }

        MobGroup {
            center,
            members,
        }
    }



    pub fn update(&mut self, dt: f64, rng: &mut StdRng) {

    }
}