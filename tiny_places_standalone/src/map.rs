use vecmath::Vector2;
use crate::item::Item;
use crate::mob::Mob;

pub const MAP_GROUND_LAYER:usize = 0;
pub const MAP_DECO_LAYER:usize = 1;
pub const MAP_CLOUD_LAYER:usize = 2;


pub struct Map {
    pub layers: [Vec<MapObject>; 7],

    pub player: Mob,
}


impl Map {
    pub fn new() -> Map {
        let layers = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(),];        
        Map {
            layers,
            player: Mob::new(1000.0, 1000.0),
        }
    }


    pub fn find_nearest_object(&mut self, layer: usize, position: &Vector2<f64>) -> Option<&mut MapObject> {
        let objects = &self.layers[layer];
        let mut distance = 999999.0;
        let mut best_idx = 0;

        for idx in 0..objects.len() {
            let object = &objects[idx];
            let dx = object.position[0] - position[0];
            let dy = object.position[1] - position[1];
            let d2 = dx * dx + dy * dy;

            println!("object {} has distance {}", object.id, d2);

            if d2 < distance {
                distance = d2;
                best_idx = idx;
            }
        }

        let mut result:Option<&mut MapObject> = None;

        if distance < 10000.0 {
            result = self.layers[layer].get_mut(best_idx);

            println!("  best object is {}", best_idx);
        }

        result
    }


    pub fn update(&mut self, dt: f64) {
        self.player.move_by_time(dt);
    }
}


pub struct MapObject {
    pub id: usize,
    pub position: Vector2<f64>,
    pub scale: f64,
    pub item: Option<Item>    
}


impl MapObject {
    
    pub fn new(id: usize, position: Vector2<f64>, scale: f64) -> MapObject {
        MapObject { 
            id, 
            position, 
            scale,
            item: None,
        }
    }

}