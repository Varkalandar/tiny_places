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
    // build_winded_corridor(map, &mut rng, 0, 0, 10, 10);

    rooms_and_corridors(map, &mut rng);

    [0.0, 0.0]
}


fn rooms_and_corridors<R: Rng + ?Sized>(map: &mut Map, rng: &mut R) {

    for ry in 0 .. 4 {
        for rx in 0 .. 4 {
            let x = rx * 12;
            let y = ry * 12;

            let l = x - rng.random_range(1..4);
            let t = y - rng.random_range(1..4); 
            let r = x + rng.random_range(2..5);
            let b = y + rng.random_range(2..5);

            build_room(map, rng, l, t, r, b);
        }
    }

    for ry in 0 .. 3 {
        for rx in 0 .. 3 {
            let x = rx * 12;
            let y = ry * 12;

            let wriggle_prob = rng.random_range(0.1 .. 1.0);
            build_winded_corridor(map, rng, x, y, x+12, y, wriggle_prob);

            let wriggle_prob = rng.random_range(0.1 .. 1.0);
            build_winded_corridor(map, rng, x, y, x, y+12, wriggle_prob);
        }
    }
}


fn create_mob(map: &mut Map, tile_id: usize, layer: usize, position: Vector2<f64>, height: f64, scale: f64) -> u64 {
    let mob = map.factory.create_mob(tile_id, layer, position, height, scale);
    let mob_id = mob.uid;
    map.layers[layer].insert(mob_id, mob);

    mob_id
}


fn build_room<R: Rng + ?Sized>(map: &mut Map, rng: &mut R, sx: i32, sy: i32, dx: i32, dy: i32) {

    for y in sy .. dy {
        for x in sx .. dx {
            place_floor_tile(map, x, y);
        }
    }
}


fn build_winded_corridor<R: Rng + ?Sized>(map: &mut Map, rng: &mut R, sx: i32, sy: i32, dx: i32, dy: i32,
                                          wriggle_prob: f64) {
    // is this straight?

    if sx == dx || sy == dy {
        // straight corridor

        subdivide_corridor(map, rng, sx, sy, dx, dy, wriggle_prob);
    }
    else {
        // L-shaped corridor, split it into two straight parts

        // two options to chose

        if rng.random() {
            subdivide_corridor(map, rng, sx, sy, sx, dy, wriggle_prob);
            subdivide_corridor(map, rng, sx, dy, dx, dy, wriggle_prob);
        }
        else {
            subdivide_corridor(map, rng, sx, sy, sy, dx, wriggle_prob);
            subdivide_corridor(map, rng, sy, dx, dx, dy, wriggle_prob);    
        }
    }
}


fn subdivide_corridor<R: Rng + ?Sized>(map: &mut Map, rng: &mut R, sx: i32, sy: i32, dx: i32, dy: i32,
                                       wriggle_prob: f64) {
    let vx = (dx - sx).signum();
    let vy = (dy - sy).signum();

    let n = cmp::max((dx - sx).abs(), (dy - sy).abs());
    let p: f64 = rng.random();

    if n < 5 || p > wriggle_prob{
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
                                     sx + min * vx + d * vy, sy + min * vy + d * -vx,
                                wriggle_prob);

        subdivide_corridor(map, rng, sx + min * vx + d * vy, sy + min * vy + d * -vx, 
                                     sx + max * vx + d * vy, sy + max * vy + d * -vx,
                                wriggle_prob);

        subdivide_corridor(map, rng, sx + max * vx + d * vy, sy + max * vy + d * -vx, 
                                     sx + max * vx, sy + max * vy,
                                wriggle_prob);

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
    let scale = 0.25;

    create_mob(map, id, layer, map_pos(x, y, scale), height, scale);
}


fn map_pos(x: i32, y: i32, scale: f64) -> [f64; 2] {

    let fx = ((y + x) * 108) as f64; 
    let fy = ((y - x) * 108) as f64;
    // let fx = ((y + x) * 54) as f64; 
    // let fy = ((y - x) * 54) as f64;

    // println!("{}, {} -> {}, {}", x, y, fx, fy);

    [fx * scale, fy * scale]
}