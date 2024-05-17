use std::path::Path;
use std::cell::OnceCell;
use std::rc::Rc;
use std::collections::HashMap;

use graphics::{draw_state::DrawState, Rectangle, Viewport, ImageSize, Image,};
use opengl_graphics::{GlGraphics, Texture, TextureSettings};

use crate::ui::{UiHead, UiComponent, UiArea, UiFont, MouseMoveEvent};
use crate::Inventory;
use crate::inventory::Slot;
use crate::TileSet;
use crate::item::Item;


pub struct PlayerInventoryView {
    area: UiArea,
    texture: Texture,
    inventory: Rc<OnceCell<Inventory>>,
    item_tiles: TileSet,

    slot_offsets: HashMap<Slot, [i32; 2]>,
    slot_sizes: HashMap<Slot, (i32, i32)>,

    hover_item: Option<usize>,
    font: Rc<UiFont>,
}


impl PlayerInventoryView {

    pub fn new(x: i32, y: i32, font: &Rc<UiFont>, 
               inventory: Rc<OnceCell<Inventory>>, tiles: &TileSet) -> UiComponent {

        let texture = Texture::from_path(Path::new("resources/ui/inventory_bg.png"), &TextureSettings::new()).unwrap();

        let mut slot_offsets = HashMap::new();
        slot_offsets.insert(Slot::BAG, [10, 452]);
        slot_offsets.insert(Slot::RWING, [20, 205]);

        let mut slot_sizes = HashMap::new();
        slot_sizes.insert(Slot::LWING, (2*32, 3*32));
        slot_sizes.insert(Slot::RWING, (2*32, 3*32));

        let inv = PlayerInventoryView {
            area: UiArea {
                x, 
                y,
                w: texture.get_width() as i32,
                h: texture.get_height() as i32,                
            },
            
            texture,
            inventory,
            item_tiles: tiles.shallow_copy(),

            slot_offsets,
            slot_sizes,
            hover_item: None,
            font: font.clone(),
        };

        UiComponent {
            head: Box::new(inv),
        }
    }

    fn find_slot_size(&self, item: &Item, slot: Slot) -> (i32, i32) {

        if slot == Slot::BAG {
            (item.inventory_w * 32, item.inventory_h * 32)
        }
        else {
            *self.slot_sizes.get(&slot).unwrap()
        }
    }
}


impl UiHead for PlayerInventoryView {

    fn area(&self) -> &UiArea {
        &self.area
    }


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

                if entry.slot != Slot::STASH {
                    let offsets = self.slot_offsets.get(&entry.slot).unwrap();
                    let entry_x = (xp + offsets[0] + entry.location_x * 32) as f64;
                    let entry_y = (yp + offsets[1] + entry.location_y * 32) as f64;
                    
                    let item = inventory.bag.get(&entry.item_id).unwrap();
                    let size = self.find_slot_size(item, entry.slot);
                    let w = size.0 as f64;
                    let h = size.1 as f64;

                    if self.hover_item == Some(item.id) {
                        let rect = Rectangle::new([0.2, 0.7, 0.0, 0.05]); 
                        rect.draw([entry_x as f64 + 1.0, entry_y as f64 + 1.0, w - 2.0, h - 2.0], draw_state, c.transform, gl);
                    }
                    else {
                        let rect = Rectangle::new([0.0, 0.02, 0.1, 0.7]); 
                        rect.draw([entry_x as f64 + 1.0, entry_y as f64 + 1.0, w - 2.0, h - 2.0], draw_state, c.transform, gl);
                    }

                    let tile = self.item_tiles.tiles_by_id.get(&item.inventory_tile_id).unwrap();

                    let mut tw = tile.tex.get_width() as f64;
                    let mut th = tile.tex.get_height() as f64;

                    let s1 = w / tw;
                    let s2 = h / th;

                    let scale = if s1 < s2 { s1 } else { s2 };

                    tw = tw * scale * 0.95 * item.inventory_scale;
                    th = th * scale * 0.95 * item.inventory_scale;

                    let ox = (w - tw) / 2.0;
                    let oy = (h - th) / 2.0;

                    let image = 
                        Image::new()
                            .rect([entry_x + ox, entry_y + oy, tw, th])
                            .color([1.0, 1.0, 1.0, 1.0]);
                    image.draw(&tile.tex, draw_state, tf, gl);
                }
            }
        
            match self.hover_item {
                None => {

                },
                Some(id) => {
                    for entry in &inventory.entries {
                        if entry.item_id == id {
                            let offsets = self.slot_offsets.get(&entry.slot).unwrap();
                            let item = inventory.bag.get(&id).unwrap();

                            let entry_x = (xp + offsets[0] + entry.location_x * 32);
                            let entry_y = (yp + offsets[1] + entry.location_y * 32);
        
                            self.font.draw(viewport, gl, draw_state, entry_x, entry_y, 
                                           &item.name, &[0.8, 1.0, 0.0, 1.0]);
                        }
                    }
                }
            }
        });
    }


    fn handle_mouse_move_event(&mut self, event: &MouseMoveEvent) -> Option<&dyn UiHead> {

        self.hover_item = None;
        
        // println!("Mouse moved to {}, {}", event.mx, event.my);

        let area = &self.area;
        let inventory = self.inventory.get().unwrap();

        for entry in &inventory.entries {
            if entry.slot != Slot::STASH {
                let offsets = self.slot_offsets.get(&entry.slot).unwrap();
                let entry_x = (area.x + offsets[0] + entry.location_x * 32);
                let entry_y = (area.y + offsets[1] + entry.location_y * 32);
                
                let item = inventory.bag.get(&entry.item_id).unwrap();
                let (w, h) = self.find_slot_size(item, entry.slot);

                if event.mx >= entry_x && event.my >= entry_y &&
                   event.mx < entry_x + w && event.my < entry_y + h {
                    println!("Hovering {}", &item.name);
                    self.hover_item = Some(item.id);
                }
            }
        }

        None
    }
}