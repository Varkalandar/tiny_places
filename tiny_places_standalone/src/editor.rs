#[path = "tileset.rs"]
mod tileset;

#[path = "ui.rs"]
mod ui;

use std::rc::Rc;

use crate::tileset::{TileSet, Tile};
use crate::ui::{UI, UiContainer};


pub struct MapEditor {
}


impl MapEditor {

    pub fn new() -> MapEditor {
        MapEditor {
        }
    }


    pub fn make_tile_selector(&self, ui: &UI, tileset: &TileSet) -> UiContainer {
        let count = tileset.tiles_by_id.len();
        let rows = count / 8;
        let size = &ui.window_size;
        
        let ww = size[0] as i32;
        let wh = size[1] as i32;
        let w = 800;
        let h = 600;
        let x_space = 134;
        let y_space = 150;


        let mut cont = ui.make_container((ww - w)/2, (wh - h)/2, w, h);
        let mut x = 0;
        let mut y = 0;

        for i in 0..10000 {

            let id = tileset.tiles_order_to_id.get(&i);
            match id {
                None => {

                },
                Some(id) => {
                    let tile = tileset.tiles_by_id.get(id).unwrap();
                    let callback = |x| {
                        println!("Icon {} clicked", x);
                        x
                    };

                    let icon = ui.make_icon(x+10, y+10, x_space-20, y_space-20, 
                                            tile, &tile.name, callback, *id);
                        
                    cont.children.push(Rc::new(icon));
        
                    x += x_space;
        
                    if x >= w {
                        x = 0;
                        y += y_space;
                    }
                }
            }
        }

        cont
    }

}