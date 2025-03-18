use std::cmp;

use vecmath::Vector2;
// use rand::Rng;
use rand::prelude::*;

use crate::MAP_GROUND_LAYER;
use crate::MAP_OBJECT_LAYER;
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

    let mut entrances: [i32; 16 * 8] = [0; 16 * 8];

    for ry in 0 .. 4 {
        for rx in 0 .. 4 {
            let x = rx * 12 + rng.random_range(-3..3);
            let y = ry * 12 + rng.random_range(-3..3);

            let l = x - rng.random_range(1..3);
            let t = y - rng.random_range(1..3); 
            let r = x + rng.random_range(1..3);
            let b = y + rng.random_range(1..3);

            // keep track of entrances

            // start index of the room data in the array. 4 coordinates, 2 values each
            let room: usize = ((ry * 4 + rx) * 8) as usize;

            entrances[room + 0] = x;
            entrances[room + 1] = t;

            entrances[room + 2] = r;
            entrances[room + 3] = y;

            entrances[room + 4] = x;
            entrances[room + 5] = b;

            entrances[room + 6] = l;
            entrances[room + 7] = y;

            build_room(map, rng, l, t, r, b, &entrances[room .. room + 8]);
        }
    }

    for ry in 0 .. 4 {
        for rx in 0 .. 4 {

            let room = (ry * 4 + rx) * 8;
            
            // "down right" corridors

            // straight starting stubs
            if ry > 0 {
                build_straight_corridor(map,
                    entrances[room + 0], //  = x;
                    entrances[room + 1] - 1, //  = b;
                    entrances[room + 0], //  = x;
                    entrances[room + 1] - 2, //  = b;
                );
            }

            if ry < 3 {
                build_straight_corridor(map,
                    entrances[room + 4], //  = x;
                    entrances[room + 5] + 1, //  = b;
                    entrances[room + 4], //  = x;
                    entrances[room + 5] + 2, //  = b;
                );

                // now the windy connection
                let wriggle_prob = rng.random_range(0.1 .. 1.0);
                build_winded_corridor(map, rng, 
                    entrances[room + 4], //  = x;
                    entrances[room + 5] + 2, //  = b;
        
                    entrances[room + 8 * 4], //  = x;
                    entrances[room + 8 * 4 + 1] - 2, //  = t;

                    wriggle_prob);
            }
           

            // "up right" corridors

            // straight starting stubs
            if rx > 0 {
                println!("{}, {}", entrances[room + 2], entrances[room + 3]);

                build_straight_corridor(map,
                    entrances[room + 6] - 1,
                    entrances[room + 7],
                    entrances[room + 6] - 2,
                    entrances[room + 7],
                );
            }

            
            if rx < 3 {
                
                build_straight_corridor(map,
                    entrances[room + 2] + 1,
                    entrances[room + 3],
                    entrances[room + 2] + 2,
                    entrances[room + 3],
                );

                let wriggle_prob = rng.random_range(0.1 .. 1.0);
                build_winded_corridor(map, rng, 
                    entrances[room + 2] + 2,
                    entrances[room + 3],
                    entrances[room + 8 + 6] - 2,
                    entrances[room + 8 + 7],

                    wriggle_prob);
                    
            }
              
        }
    }
}


fn create_mob(map: &mut Map, tile_id: usize, layer: usize, position: Vector2<f64>, height: f64, scale: f64) -> u64 {
    let mob = map.factory.create_mob(tile_id, layer, position, height, scale);
    let mob_id = mob.uid;
    map.layers[layer].insert(mob_id, mob);

    mob_id
}


fn build_room<R: Rng + ?Sized>(map: &mut Map, rng: &mut R, sx: i32, sy: i32, dx: i32, dy: i32,
                               entrances: &[i32]) {

    for y in sy .. dy + 1 {
        for x in sx .. dx + 1 {
            place_floor_tile(map, x, y, 49);
        }
    }

    // tall back walls

    // left
    for x in sx .. dx + 1 {
        if sy < 3 || entrances[4] != x {
            place_wall_tile(map, x+1, sy-2, -30, 494);
        }
    }

    // right
    for y in sy .. dy + 1 {
        if dx > 12 * 3 - 6 || entrances[3] != y {
            place_wall_tile(map, dx+2, y-2, 76, 495);
        }
    }

    // short front walls
    
    // right
    for x in sx .. dx + 1 {
        if dy > 12 * 3 - 6 || entrances[0] != x {
            place_wall_tile(map, x+1, dy-1, 98, 497);
        }
    }

    // left
    for y in sy .. dy + 1 {
        if sx < 3 || entrances[7] != y {
            place_wall_tile(map, sx, y-1, -6, 496);
        }
    }

    // left room corner
    place_wall_tile(map, sx, sy-1, 132, 498);

    // right room corner
    place_wall_tile(map, dx+1, dy, 128, 501);
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
            subdivide_corridor(map, rng, sx, sy, dx, sy, wriggle_prob);
            subdivide_corridor(map, rng, dx, sy, dx, dy, wriggle_prob);    
        }
    }
}


fn subdivide_corridor<R: Rng + ?Sized>(map: &mut Map, rng: &mut R, sx: i32, sy: i32, dx: i32, dy: i32,
                                       wriggle_prob: f64) {
    let vx = (dx - sx).signum();
    let vy = (dy - sy).signum();

    if vx != 0 && vy != 0 {
        panic!("Diagonal corridor {}, {}", vx, vy);
    }

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

    let n = cmp::max((dx - sx).abs(), (dy - sy).abs()) + 1;
    let mut x = sx;
    let mut y = sy;

    for i in 0..n {
        place_floor_tile(map, x, y, 50);

        x += vx;
        y += vy;
    }
}


fn place_floor_tile(map: &mut Map, x: i32, y: i32, id: usize) {
    let layer = MAP_GROUND_LAYER;
    let height = 0.0;
    let scale = 0.2;

    let mob_id = create_mob(map, id, layer, map_pos(x, y, 0, scale), height, scale);

    let mob = map.layers[layer].get_mut(&mob_id).unwrap();

    mob.visual.color = [0.69f32, 0.71, 0.725, 0.8];
}


fn place_wall_tile(map: &mut Map, x: i32, y: i32, z_off: i32, id: usize) {
    let layer = MAP_OBJECT_LAYER;
    let height = 0.0;
    let scale = 0.2;
    let pos = map_pos(x, y, z_off, scale);

    create_mob(map, id, layer, pos, height, scale);
}


fn map_pos(x: i32, y: i32, z_off: i32, scale: f64) -> [f64; 2] {

    let fx = ((y + x) * 108) as f64; 
    let fy = ((y - x) * 108 + z_off) as f64;

    // println!("{}, {} -> {}, {}", x, y, fx, fy);

    [fx * scale, fy * scale]
}