use std::path::Path;
use std::cell::OnceCell;
use std::rc::Rc;
use std::collections::HashMap;

use graphics::{draw_state::DrawState, Rectangle, Viewport, ImageSize, Image,};
use opengl_graphics::{GlGraphics, Texture, TextureSettings};

use crate::ui::{UiHead, UiComponent, UiArea};
use crate::Inventory;
use crate::inventory::Slot;


pub struct PlayerInventoryView {
    area: UiArea,
    texture: Texture,
    inventory: Rc<OnceCell<Inventory>>,

    offsets: HashMap<Slot, [i32; 2]>,
}


impl PlayerInventoryView {

    pub fn new(x: i32, y: i32, inventory: Rc<OnceCell<Inventory>>) -> UiComponent {

        let texture = Texture::from_path(Path::new("resources/ui/inventory_bg.png"), &TextureSettings::new()).unwrap();

        let mut offsets = HashMap::new();
        offsets.insert(Slot::BAG, [10, 452]);

        let inv = PlayerInventoryView {
            area: UiArea {
                x, 
                y,
                w: 400,
                h: 600,                
            },
            
            texture,
            inventory,
            offsets,
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
            let m_image   = 
                Image::new()
                    .rect([xp as f64, yp as f64, self.texture.get_width() as f64, self.texture.get_height() as f64])
                    .color([1.0, 1.0, 1.0, 0.95]);
            m_image.draw(&self.texture, draw_state, tf, gl);

            let inventory = self.inventory.get().unwrap();
        
            // show all items which are in the inventory space
            for entry in &inventory.entries {

                if entry.slot == Slot::BAG {
                    let offsets = self.offsets.get(&Slot::BAG).unwrap();
                    let entry_x = xp + offsets[0] + entry.location_x * 32;
                    let entry_y = yp + offsets[1] + entry.location_y * 32;
                    
                    let item = inventory.bag.get(&entry.item_id).unwrap();
                    let w = item.inventory_w * 32;
                    let h = item.inventory_h * 32;

                    let rect = Rectangle::new([0.2, 0.7, 0.0, 1.0]); 
                    rect.draw([entry_x as f64, entry_y as f64, w as f64, h as f64], draw_state, c.transform, gl);
                }
            }
        
        });
    }
}