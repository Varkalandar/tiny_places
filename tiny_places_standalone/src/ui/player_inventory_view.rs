use std::path::Path;
use std::cell::OnceCell;
use std::rc::Rc;
use std::collections::HashMap;

use graphics::{draw_state::DrawState, Rectangle, Viewport, ImageSize, Image,};
use opengl_graphics::{GlGraphics, Texture, TextureSettings};

use crate::ui::{UiHead, UiComponent, UiArea};
use crate::Inventory;
use crate::inventory::Slot;
use crate::TileSet;


pub struct PlayerInventoryView {
    area: UiArea,
    texture: Texture,
    inventory: Rc<OnceCell<Inventory>>,
    item_tiles: TileSet,

    slot_offsets: HashMap<Slot, [i32; 2]>,
}


impl PlayerInventoryView {

    pub fn new(x: i32, y: i32, inventory: Rc<OnceCell<Inventory>>, tiles: &TileSet) -> UiComponent {

        let texture = Texture::from_path(Path::new("resources/ui/inventory_bg.png"), &TextureSettings::new()).unwrap();

        let mut slot_offsets = HashMap::new();
        slot_offsets.insert(Slot::BAG, [10, 452]);

        let inv = PlayerInventoryView {
            area: UiArea {
                x, 
                y,
                w: 400,
                h: 600,                
            },
            
            texture,
            inventory,
            item_tiles: tiles.shallow_copy(),

            slot_offsets,
        };

        UiComponent {
            head: Box::new(inv),
        }
    }
}


impl UiHead for PlayerInventoryView {

    fn draw(&self, viewport: Viewport, gl: &mut GlGraphics, draw_state: &DrawState, x: i32, y: i32) {
        let area = &self.area;
        let xp = x + area.x;
        let yp = y + area.y;

        gl.draw(viewport, |c, gl| {

            // placeholder
            /*
            let rect = Rectangle::new([0.0, 0.0, 0.0, 0.3]); 
            rect.draw([xp as f64, yp as f64, area.w as f64, area.h as f64], draw_state, c.transform, gl);
            */

            let tf = c.transform;
            let m_image = 
                Image::new()
                    .rect([xp as f64, yp as f64, self.texture.get_width() as f64, self.texture.get_height() as f64])
                    .color([1.0, 1.0, 1.0, 0.95]);
            m_image.draw(&self.texture, draw_state, tf, gl);

            let inventory = self.inventory.get().unwrap();
        
            // show all items which are in the inventory space
            for entry in &inventory.entries {

                if entry.slot == Slot::BAG {
                    let offsets = self.slot_offsets.get(&Slot::BAG).unwrap();
                    let entry_x = (xp + offsets[0] + entry.location_x * 32) as f64;
                    let entry_y = (yp + offsets[1] + entry.location_y * 32) as f64;
                    
                    let item = inventory.bag.get(&entry.item_id).unwrap();
                    let w = (item.inventory_w * 32) as f64;
                    let h = (item.inventory_h * 32) as f64;

                    let rect = Rectangle::new([0.2, 0.7, 0.0, 0.05]); 
                    rect.draw([entry_x as f64 + 1.0, entry_y as f64 + 1.0, w - 2.0, h - 2.0], draw_state, c.transform, gl);

                    let tile = self.item_tiles.tiles_by_id.get(&item.inventory_tile_id).unwrap();

                    let mut tw = tile.tex.get_width() as f64;
                    let mut th = tile.tex.get_height() as f64;

                    let s1 = w / tw;
                    let s2 = h / th;

                    let scale = if s1 < s2 { s1 } else { s2 };

                    tw = tw * scale * 0.95;
                    th = th * scale * 0.95;

                    let ox = (w - tw) / 2.0;
                    let oy = (h - th) / 2.0;

                    let image = 
                        Image::new()
                            .rect([entry_x + ox, entry_y + oy, tw, th])
                            .color([1.0, 1.0, 1.0, 1.0]);
                    image.draw(&tile.tex, draw_state, tf, gl);
                }
            }
        
        });
    }
}