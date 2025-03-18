use std::cmp;

use vecmath::Vector2;
// use rand::Rng;
use rand::prelude::*;

use crate::MAP_GROUND_LAYER;
use crate::Map;
use crate::map::MapObject;

pub fn generate_dungeon(map: &mut Map) -> [f64; 2] {

/*
    let layer = MAP_GROUND_LAYER;
    let height = 0.0;
    let id = 50;
    let mut pos = [1000.0, 1000.0];

    // let height = world.layer_tileset[layer].tiles_by_id.get(&id).unwrap().foot[1];
    
    create_mob(map, id, layer, pos, height, 1.0);

    pos[0] += 108.0;
    pos[1] += 108.0;

    create_mob(map, id, layer, pos, height, 1.0);
*/

    let mut rng = rand::rng();

    // place_floor_tile(map, -5 + 5, 5 + 5);
    // build_winded_corridor(map, &mut rng, 0, 10, 10, 20);
    build_winded_corridor(map, &mut rng, 0, 0, 10, 10);

    [0.0, 0.0]
}


fn create_mob(map: &mut Map, tile_id: usize, layer: usize, position: Vector2<f64>, height: f64, scale: f64) -> u64 {
    let mob = map.factory.create_mob(tile_id, layer, position, height, scale);
    let mob_id = mob.uid;
    map.layers[layer].insert(mob_id, mob);

    mob_id
}


fn build_winded_corridor<R: Rng + ?Sized>(map: &mut Map, rng: &mut R, sx: i32, sy: i32, dx: i32, dy: i32) {
    // is this straight?

    if sx == dx || sy == dy {
        // straight corridor

        subdivide_corridor(map, rng, sx, sy, dx, dy);
    }
    else {
        // L-shaped corridor, split it into two straight parts

        // two options to chose

        if rng.random() {
            subdivide_corridor(map, rng, sx, sy, sx, dy);
            subdivide_corridor(map, rng, sx, dy, dx, dy);
        }
        else {
            subdivide_corridor(map, rng, sx, sy, sy, dx);
            subdivide_corridor(map, rng, sy, dx, dx, dy);    
        }
    }
}


fn subdivide_corridor<R: Rng + ?Sized>(map: &mut Map, rng: &mut R, sx: i32, sy: i32, dx: i32, dy: i32) {
    let vx = (dx - sx).signum();
    let vy = (dy - sy).signum();

    let n = cmp::max((dx - sx).abs(), (dy - sy).abs());

    if n < 5 {
        // too short to be wriggled. Build straight
        build_straight_corridor(map, sx, sy, dx, dy);
    }
    else {
        let min = 2;
        let max = n - 2;

        // start piece
        build_straight_corridor(map, sx, sy, sx + min * vx, sy + min * vy);

        // depth of turn
        let d:i32 = rng.random_range(-n/2 .. n/2);

        // U turn

        subdivide_corridor(map, rng, sx + min * vx, sy + min * vy, 
                                    sx + min * vx + d * vy, sy + min * vy + d * -vx);

        subdivide_corridor(map, rng, sx + min * vx + d * vy, sy + min * vy + d * -vx, 
                                    sx + max * vx + d * vy, sy + max * vy + d * -vx);

        subdivide_corridor(map, rng, sx + max * vx + d * vy, sy + max * vy + d * -vx, 
                                    sx + max * vx, sy + max * vy);

        // end piece
        build_straight_corridor(map, sx + max * vx, sy + max * vy, dx, dy);
    }
}


fn build_straight_corridor(map: &mut Map, sx: i32, sy: i32, dx: i32, dy: i32) {
    let vx = (dx - sx).signum();
    let vy = (dy - sy).signum();

    let n = cmp::max((dx - sx).abs(), (dy - sy).abs());
    let mut x = sx;
    let mut y = sy;

    for i in 0..n {
        place_floor_tile(map, x, y);

        x += vx;
        y += vy;
    }
}

fn place_floor_tile(map: &mut Map, x: i32, y: i32) {
    let layer = MAP_GROUND_LAYER;
    let height = 0.0;
    let id = 50;
    let scale = 0.5;

    create_mob(map, id, layer, map_pos(x, y), height, scale);
}


fn map_pos(x: i32, y: i32) -> [f64; 2] {

    // let fx = ((y + x) * 108) as f64; 
    // let fy = ((y - x) * 108) as f64;
    let fx = ((y + x) * 54) as f64; 
    let fy = ((y - x) * 54) as f64;

    // println!("{}, {} -> {}, {}", x, y, fx, fy);

    [fx, fy]
}