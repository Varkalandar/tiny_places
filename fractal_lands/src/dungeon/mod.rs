use vecmath::Vector2;

use crate::MAP_GROUND_LAYER;
use crate::Map;
use crate::map::MapObject;

pub fn generate_dungeon(map: &mut Map) {


    let layer = MAP_GROUND_LAYER;
    let height = 0.0;
    let id = 50;
    let mut pos = [1000.0, 1000.0];

    // let height = world.layer_tileset[layer].tiles_by_id.get(&id).unwrap().foot[1];
    
    create_mob(map, id, layer, pos, height, 1.0);

    pos[0] += 108.0;
    pos[1] += 108.0;

    create_mob(map, id, layer, pos, height, 1.0);

}

fn create_mob(map: &mut Map, tile_id: usize, layer: usize, position: Vector2<f64>, height: f64, scale: f64) -> u64 {
    let mob = map.factory.create_mob(tile_id, layer, position, height, 1.0);
    let mob_id = mob.uid;
    map.layers[layer].insert(mob_id, mob);

    mob_id
}