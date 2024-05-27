use std::path::Path;
use std::rc::Rc;
use std::collections::HashMap;

use graphics::{draw_state::DrawState, Rectangle, Viewport, ImageSize, Image, types::Matrix2d};
use opengl_graphics::{GlGraphics, Texture, TextureSettings};

use crate::ui::{UiArea, UiFont, MouseMoveEvent, MouseState, ButtonEvent};
use crate::Inventory;
use crate::inventory::Slot;
use crate::inventory::Entry;
use crate::TileSet;
use crate::item::Item;
use crate::ButtonState;
use crate::MouseButton;

pub struct PlayerInventoryView {
    area: UiArea,
    texture: Texture,
    item_tiles: TileSet,

    slot_offsets: HashMap<Slot, [i32; 2]>,
    slot_sizes: HashMap<Slot, (i32, i32)>,

    hover_item: Option<usize>,
    dragged_item: Option<usize>,
    drag_x: i32,
    drag_y: i32,

    font: Rc<UiFont>,
}


impl PlayerInventoryView {

    pub fn new(x: i32, y: i32, font: &Rc<UiFont>, tiles: &TileSet) -> PlayerInventoryView {

        let texture = Texture::from_path(Path::new("resources/ui/inventory_bg.png"), &TextureSettings::new()).unwrap();

        let mut slot_offsets = HashMap::new();
        slot_offsets.insert(Slot::Bag, [10, 452]);
        slot_offsets.insert(Slot::RWing, [20, 205]);

        let mut slot_sizes = HashMap::new();
        slot_sizes.insert(Slot::LWing, (2*32, 3*32));
        slot_sizes.insert(Slot::RWing, (2*32, 3*32));

        PlayerInventoryView {
            area: UiArea {
                x, 
                y,
                w: texture.get_width() as i32,
                h: texture.get_height() as i32,                
            },
            
            texture,
            item_tiles: tiles.shallow_copy(),

            slot_offsets,
            slot_sizes,
            hover_item: None,
            dragged_item: None,
            drag_x: 0,
            drag_y: 0,
            font: font.clone(),
        }
    }


    fn find_slot_size(&self, item: &Item, slot: Slot) -> (i32, i32) {

        if slot == Slot::Bag {
            (item.inventory_w * 32, item.inventory_h * 32)
        }
        else {
            *self.slot_sizes.get(&slot).unwrap()
        }
    }


    fn show_item_popup(&self, viewport: Viewport, gl: &mut GlGraphics, draw_state: &DrawState,
                       x: i32, y: i32, item: &Item) {

        let linespace = 20;
        let lines = item.mods.len() as i32 + 1;
        let mut line = y - lines * linespace;

        self.font.draw(viewport, gl, draw_state, x, line, &item.name, &[0.8, 1.0, 0.0, 1.0]);
        line += linespace;

        for modifier in &item.mods {

            let text = modifier.attribute.to_string() + ": " + &modifier.value.to_string();
            self.font.draw(viewport, gl, draw_state, x, line, &text, &[0.8, 1.0, 0.0, 1.0]);
            line += linespace;
        }
    }


    fn find_item_at(&self, inventory: &Inventory, mx: i32, my: i32) -> Option<usize> {
        let area = &self.area;

        for entry in &inventory.entries {
            if entry.slot != Slot::Stash && entry.slot != Slot::OnCursor {
                let offsets = self.slot_offsets.get(&entry.slot).unwrap();
                let entry_x = area.x + offsets[0] + entry.location_x * 32;
                let entry_y = area.y + offsets[1] + entry.location_y * 32;
                
                let item = inventory.bag.get(&entry.item_id).unwrap();
                let (w, h) = self.find_slot_size(item, entry.slot);

                if mx >= entry_x && my >= entry_y &&
                   mx < entry_x + w && my < entry_y + h {
                    // println!("Found {}", &item.name);
                    return Some(item.id);
                }
            }
        }
 
        None
    }


    fn draw_item(&self, gl: &mut GlGraphics, draw_state: &DrawState, tf: Matrix2d<f64>,
                 id: usize, entry_x: f64, entry_y: f64, w: f64, h: f64, inventory_scale: f64) {

        let tile = self.item_tiles.tiles_by_id.get(&id).unwrap();

        let mut tw = tile.tex.get_width() as f64;
        let mut th = tile.tex.get_height() as f64;

        let s1 = w / tw;
        let s2 = h / th;

        let scale = if s1 < s2 { s1 } else { s2 };

        tw = tw * scale * 0.95 * inventory_scale;
        th = th * scale * 0.95 * inventory_scale;

        let ox = (w - tw) / 2.0;
        let oy = (h - th) / 2.0;

        let image = 
            Image::new()
                .rect([entry_x + ox, entry_y + oy, tw, th])
                .color([1.0, 1.0, 1.0, 1.0]);
        image.draw(&tile.tex, draw_state, tf, gl);
    }


    pub fn draw(&self, viewport: Viewport, gl: &mut GlGraphics, draw_state: &DrawState, x: i32, y: i32, inventory: &Inventory) {
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

            // show all items which are in the inventory space
            for entry in &inventory.entries {

                if entry.slot != Slot::Stash && entry.slot != Slot::OnCursor {
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

                    self.draw_item(gl, draw_state, tf,
                        item.inventory_tile_id, entry_x, entry_y, w, h, item.inventory_scale);
                }
            }
        
            match self.hover_item {
                None => {},
                Some(id) => {
                    let idx = inventory.find_entry_for_id(id).unwrap();
                    let entry = &inventory.entries[idx];

                    if self.dragged_item.is_none() && entry.slot != Slot::OnCursor {
                        let offsets = self.slot_offsets.get(&entry.slot).unwrap();
                        let item = inventory.bag.get(&id).unwrap();

                        let entry_x = xp + offsets[0] + entry.location_x * 32;
                        let entry_y = yp + offsets[1] + entry.location_y * 32;

                        self.show_item_popup(viewport, gl, draw_state, entry_x, entry_y, item);
                    }
                }
            }

            match self.dragged_item {
                None => {},
                Some(id) => {
                    let item = inventory.bag.get(&id).unwrap();

                    self.draw_item(gl, draw_state, tf,
                        item.inventory_tile_id, self.drag_x as f64, self.drag_y as f64, 
                        (item.inventory_w * 32) as f64, (item.inventory_h * 32) as f64, item.inventory_scale);
                }
            }
        });
    }


    pub fn handle_button_event(&mut self, event: &ButtonEvent, mouse: &MouseState, inventory: &mut Inventory) -> bool {

        if event.args.state == ButtonState::Release &&
           event.args.button == piston::Button::Mouse(MouseButton::Left) {

            match self.dragged_item {
                None => {
                    if self.hover_item.is_some() {
                        self.dragged_item = self.hover_item;
        
                        println!("Started to drag item idx={:?} from {}, {}", self.dragged_item, event.mx, event.my);
                        
                        let item_id = self.dragged_item.unwrap();
                        let idx = inventory.find_entry_for_id(item_id).unwrap();
                        let entry: &mut Entry = &mut inventory.entries[idx];
                        entry.slot = Slot::OnCursor;

                        return true;
                    }
                },
                Some(id) => {
                    let item = inventory.bag.get(&id).unwrap();

                    println!("Dropped an {}", item.name);

                    let idx = inventory.find_entry_for_id(id).unwrap();
                    let entry: &mut Entry = &mut inventory.entries[idx];
                    
                    let offsets = self.slot_offsets.get(&Slot::Bag).unwrap();

                    let rel_x = (mouse.position[0] as i32) - self.area.x - offsets[0];
                    let rel_y = (mouse.position[1] as i32) - self.area.y - offsets[1];
                    
                    entry.location_x = rel_x / 32;
                    entry.location_y = rel_y / 32;
                    entry.slot = Slot::Bag; // todo, find real slot

                    self.dragged_item = None;

                    return true;
                }
            }
        }

        false
    }


    pub fn handle_mouse_move_event(&mut self, event: &MouseMoveEvent, mouse: &MouseState, inventory: &mut Inventory) -> bool {

        // println!("Mouse moved to {}, {}", event.mx, event.my);

        let item_opt = self.find_item_at(inventory, event.mx, event.my);
        self.hover_item = item_opt;

        self.drag_x = event.mx;
        self.drag_y = event.my;

        false
    }
}