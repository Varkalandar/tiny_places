use std::rc::Rc;
use std::collections::HashMap;

use glium::Texture2d;

use crate::ui::{UiArea, UiFont, MouseButton, MouseMoveEvent, MouseState, ButtonEvent};
use crate::Inventory;
use crate::inventory::Slot;
use crate::inventory::Entry;
use crate::TileSet;
use crate::item::Item;
use crate::GameWorld;
use crate::sound::Sound;
use crate::ui::Button;
use crate::ui::ButtonState;


pub struct PlayerInventoryView {
    area: UiArea,
    texture: Texture2d,
    item_tiles: TileSet,

    slot_offsets: HashMap<Slot, [i32; 2]>,
    slot_sizes: HashMap<Slot, [i32; 2]>,

    hover_item: Option<usize>,
    dragged_item: Option<usize>,
    drag_x: i32,
    drag_y: i32,

    font: Rc<UiFont>,
}


impl PlayerInventoryView {

    pub fn new(x: i32, y: i32, font: &Rc<UiFont>, tiles: &TileSet, texture: Texture2d) -> PlayerInventoryView {

        let mut slot_offsets = HashMap::new();
        slot_offsets.insert(Slot::Bag, [10, 452]);
        slot_offsets.insert(Slot::Body, [204, 213]);
        slot_offsets.insert(Slot::LWing, [400, 202]);
        slot_offsets.insert(Slot::RWing, [20, 205]);
        slot_offsets.insert(Slot::Engine, [214, 96]);

        let mut slot_sizes = HashMap::new();
        slot_sizes.insert(Slot::Bag, [15*32, 9*32]);
        slot_sizes.insert(Slot::LWing, [2*32, 3*32]);
        slot_sizes.insert(Slot::RWing, [2*32, 3*32]);
        slot_sizes.insert(Slot::Engine, [2*32, 3*32]);
        slot_sizes.insert(Slot::Body, [2*32, 3*32]);

        // let query = texture.query();

        PlayerInventoryView {
            area: UiArea {
                x, 
                y,
                w: 500,
                h: 750,                
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


    fn find_slot_size(&self, item: &Item, slot: Slot) -> [i32; 2] {

        if slot == Slot::Bag {
            [item.inventory_w * 32, item.inventory_h * 32]
        }
        else {
            *self.slot_sizes.get(&slot).unwrap()
        }
    }


    fn show_item_popup(&self, x: i32, y: i32, item: &Item) {

        let line_space = 20;

        let mut line_count = 1; // first line is item name

        for modifier in &item.mods {
            if modifier.max_value > 0 {
                line_count += 1;
            }
        }

        let mut line = y - line_count * line_space;

        /*
        gl.draw(viewport, |c, gl| {
            // show decorated box  ... todo
            let rect = Rectangle::new([0.0, 0.0, 0.0, 0.5]); 
            rect.draw([x as f64, line as f64, 200.0, (line_count * line_space) as f64], draw_state, c.transform, gl);
        });
        */

        self.font.draw(x, line, &item.name, &[0.8, 1.0, 0.0, 1.0]);
        line += line_space;

        for modifier in &item.mods {

            let min_value = modifier.min_value;
            let max_value = modifier.max_value;

            if max_value > 0 {
                let range = if min_value == max_value {
                    min_value.to_string()
                } else {
                    min_value.to_string() + "-" + &max_value.to_string()
                };

                let text = modifier.attribute.to_string() + ": " + &range;
                self.font.draw(x, line, &text, &[0.8, 1.0, 0.0, 1.0]);
                line += line_space;
            }
        }
    }


    fn find_slot_at(&self, mx: i32, my: i32) -> Option<Slot> {
        let area = &self.area;

        for key in self.slot_offsets.keys() {
            let offset = self.slot_offsets.get(key).unwrap();
            let size = self.slot_sizes.get(key).unwrap();
        
            if mx >= offset[0] && my >= offset[1] &&
               mx < offset[0] + size[0] && my < offset[1] + size[1] {
                return Some(*key);
            }
        }
 
        None
    }


    fn find_item_at(&self, inventory: &Inventory, mx: i32, my: i32) -> Option<usize> {
        let area = &self.area;

        for entry in &inventory.entries {
            if entry.slot != Slot::Stash && entry.slot != Slot::OnCursor {
                let offsets = self.slot_offsets.get(&entry.slot).unwrap();
                let entry_x = area.x + offsets[0] + entry.location_x * 32;
                let entry_y = area.y + offsets[1] + entry.location_y * 32;
                
                let item = inventory.bag.get(&entry.item_id).unwrap();
                let size = self.find_slot_size(item, entry.slot);

                if mx >= entry_x && my >= entry_y &&
                   mx < entry_x + size[0] && my < entry_y + size[1] {
                    // println!("Found {}", &item.name);
                    return Some(item.id);
                }
            }
        }
 
        None
    }


    fn draw_item(&self,
                 id: usize, entry_x: f64, entry_y: f64, slot_w: f64, slot_h: f64,
                 item_inventory_w: f64 , item_inventory_h: f64,
                 inventory_scale: f64) {

        let tile = self.item_tiles.tiles_by_id.get(&id).unwrap();
/*
        let mut tw = tile.tex.get_width() as f64;
        let mut th = tile.tex.get_height() as f64;

        let s1 = item_inventory_w / tw;
        let s2 = item_inventory_h / th;

        let scale = if s1 < s2 { s1 } else { s2 };

        tw = tw * scale * 0.95 * inventory_scale;
        th = th * scale * 0.95 * inventory_scale;

        let ox = (slot_w - tw) / 2.0;
        let oy = (slot_h - th) / 2.0;

        let image = 
            Image::new()
                .rect([entry_x + ox, entry_y + oy, tw, th])
                .color([1.0, 1.0, 1.0, 1.0]);
        image.draw(&tile.tex, draw_state, tf, gl);
        */
    }


    pub fn draw(&self, x: i32, y: i32, inventory: &Inventory) {
        let area = &self.area;
        let xp = x + area.x;
        let yp = y + area.y;
/*
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
                    let w = size[0] as f64;
                    let h = size[1] as f64;

                    if self.hover_item == Some(item.id) {
                        let rect = Rectangle::new([0.2, 0.7, 0.0, 0.05]); 
                        rect.draw([entry_x as f64 + 1.0, entry_y as f64 + 1.0, w - 2.0, h - 2.0], draw_state, c.transform, gl);
                    }
                    else {
                        let rect = Rectangle::new([0.0, 0.02, 0.1, 0.7]); 
                        rect.draw([entry_x as f64 + 1.0, entry_y as f64 + 1.0, w - 2.0, h - 2.0], draw_state, c.transform, gl);
                    }

                    self.draw_item(gl, draw_state, tf,
                        item.inventory_tile_id, entry_x, entry_y, w, h, 
                        (item.inventory_w * 32) as f64, (item.inventory_h * 32) as f64,
                        item.inventory_scale);
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
                        item.inventory_tile_id, (self.drag_x - 16) as f64, (self.drag_y - 16) as f64, 
                        (item.inventory_w * 32) as f64, (item.inventory_h * 32) as f64, 
                        (item.inventory_w * 32) as f64, (item.inventory_h * 32) as f64,
                        item.inventory_scale);
                }
            }
        });
        */
    }


    pub fn handle_button_event(&mut self, event: &ButtonEvent, mouse: &MouseState, world: &mut GameWorld) -> bool {

        if event.args.state == ButtonState::Release &&
           event.args.button == Button::Mouse(MouseButton::Left) {

            match self.dragged_item {
                None => {
                    if self.hover_item.is_some() {
                        self.dragged_item = self.hover_item;
        
                        world.speaker.play(Sound::Click, 0.5);
                        println!("Started to drag item idx={:?} from {}, {}", self.dragged_item, event.mx, event.my);
                        
                        let item_id = self.dragged_item.unwrap();
                        let inventory = &mut world.player_inventory;
                        let idx = inventory.find_entry_for_id(item_id).unwrap();
                        let entry: &mut Entry = &mut inventory.entries[idx];
                        entry.slot = Slot::OnCursor;

                        return true;
                    }
                },
                Some(id) => {
                    let inventory = &mut world.player_inventory;
                    let item = inventory.bag.get(&id).unwrap();

                    world.speaker.play(Sound::Click, 0.5);

                    let idx = inventory.find_entry_for_id(id).unwrap();
                    let entry: &mut Entry = &mut inventory.entries[idx];

                    let mx = (mouse.position[0] as i32) - self.area.x;
                    let my = (mouse.position[1] as i32) - self.area.y;
                    
                    let slot_opt = self.find_slot_at(mx, my);

                    if slot_opt.is_some() {
                        let slot = slot_opt.unwrap();
                        entry.slot = slot;
                        self.dragged_item = None;

                        println!("Dropped an {} to slot {:?}", item.name, slot);

                        if slot == Slot::Bag {
                            let offsets = self.slot_offsets.get(&Slot::Bag).unwrap();
                            let rel_x = mx - offsets[0];
                            let rel_y = my - offsets[1];
                            entry.location_x = rel_x / 32;
                            entry.location_y = rel_y / 32;
                        }
                        else {
                            entry.location_x = 0;
                            entry.location_y = 0;
                        }
    
                        return true;
                    }
                    else {
                        println!("No suitable drop location {}, {}", mx, my);
                    }
                }
            }
        }

        false
    }


    pub fn handle_mouse_move_event(&mut self, event: &MouseMoveEvent, _mouse: &MouseState, inventory: &mut Inventory) -> bool {

        // println!("Mouse moved to {}, {}", event.mx, event.my);

        let item_opt = self.find_item_at(inventory, event.mx, event.my);
        self.hover_item = item_opt;

        self.drag_x = event.mx;
        self.drag_y = event.my;

        false
    }
}
