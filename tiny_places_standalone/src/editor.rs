use std::rc::Rc;

use piston::{ButtonState, MouseButton};

use graphics::{draw_state::DrawState, Viewport,};
use opengl_graphics::GlGraphics;
use vecmath::Vector2;

use crate::ui::{UI, UiController, UiComponent, TileSet, ButtonEvent, ScrollEvent};
use crate::map::{MapObject, MAP_GROUND_LAYER, MAP_DECO_LAYER, MAP_CLOUD_LAYER};
use crate::{screen_to_world_pos, build_transform, build_image};
use crate::GameWorld;


pub struct MapEditor {
    pub selected_tile_id: usize,
}


impl UiController for MapEditor {
    type Appdata = GameWorld;

    /**
     * @return true if this controller could handle the event, false to pass the event to other controllers
     */
     fn handle_button_event(&mut self, ui: &mut UI, event: &ButtonEvent, world: &mut Self::Appdata) -> bool {
        // first pass the even to the UI. if there is a component
        // trigered it will consume the event. Events which are not
        // consumed by the UI will be handed to the game core

        let comp = ui.handle_button_event(&event);

        if event.args.state == ButtonState::Release {

            match comp {
                None => {
                    
                    if event.args.button == piston::Button::Mouse(MouseButton::Left) {
                        let id = self.selected_tile_id;

                        if id == 0 {
                            let ok = self.select_nearest_item(ui, world);
                            return ok;
                        }
                        else {
                            let pos = screen_to_world_pos(&ui, &world.map.player.position, &ui.mouse_state.position);
                            println!("creating map object {} at {:?}", id, pos);
                            let object = MapObject::new(id, pos, 1.0);
                            world.map.layers[world.map.selected_layer].push(object);
                            return true;
                        }
                    }

                    if event.args.button == piston::Button::Mouse(MouseButton::Right) {

                        // close dialogs
                        ui.root.head.clear();

                        if self.selected_tile_id == 0 {
                            // nothing on cursor, center map?
                        }
                        else {
                            // remove pointer item
                            self.selected_tile_id = 0;
                        }
                    }
                    
                    if event.args.button == piston::Button::Keyboard(piston::Key::D1) {                        
                        world.map.selected_layer = MAP_GROUND_LAYER;
                        self.selected_tile_id = 0;
                    }        

                    if event.args.button == piston::Button::Keyboard(piston::Key::D2) {                        
                        world.map.selected_layer = MAP_DECO_LAYER;
                        self.selected_tile_id = 0;
                    }        

                    if event.args.button == piston::Button::Keyboard(piston::Key::D3) {                        
                        world.map.selected_layer = MAP_CLOUD_LAYER;
                        self.selected_tile_id = 0;
                    }        

                    let step = if ui.keyboard_state.shift_pressed {8.0} else {1.0};

                    if event.args.button == piston::Button::Keyboard(piston::Key::Right) {                        
                        world.map.move_selected_object(step, 0.0);
                    }        

                    if event.args.button == piston::Button::Keyboard(piston::Key::Left) {                        
                        world.map.move_selected_object(-step, 0.0);
                    }        

                    if event.args.button == piston::Button::Keyboard(piston::Key::Up) {  
                        world.map.move_selected_object(0.0, -step);
                    }        

                    if event.args.button == piston::Button::Keyboard(piston::Key::Down) {                        
                        world.map.move_selected_object(0.0, step);
                    }        


                    if event.args.button == piston::Button::Keyboard(piston::Key::Space) {
                        let set = &world.layer_tileset[world.map.selected_layer];
                        let cont = self.make_tile_selector(&ui, set);
                        ui.root.head.add_child(Rc::new(cont));
                    }        

                    if event.args.button == piston::Button::Keyboard(piston::Key::C) {
                        let map = &mut world.map;
                        let object = &mut map.layers[map.selected_layer][map.selected_item];
                        let color_choice = ui.make_color_choice(100, 100, 256, 256, 1000, object.color);
                        ui.root.head.add_child(Rc::new(color_choice));
                    }        

                    if event.args.button == piston::Button::Keyboard(piston::Key::L) {
                        world.map.load("test.map");
                    }        

                    if event.args.button == piston::Button::Keyboard(piston::Key::S) {
                        world.map.save("test.map").unwrap();
                    }        
                },
                Some(comp) => {
                    let id = comp.get_id();
                    // let id = data[0];

                    if id == 1000 {
                        // this was the color choice box
                        
                        if world.map.has_selection {
                        
                            let result = comp.get_numeric_result();
                            let r = result[0];
                            let g = result[1];
                            let b = result[2];
                            let a = result[3];
                            println!("selected color is {:02x}{:02x}{:02x}{:02x}", r, g, b, a);

                            let map = &mut world.map;
                            let object = &mut map.layers[map.selected_layer][map.selected_item];
                            object.color = [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0]
        
                        }

                        return true;
                    }
                    else {
                        // must have been the tile selector
                        println!("Selected tile id={}", id);

                        if id > 0 {
                            self.selected_tile_id = id;
                            ui.root.head.clear();
    
                            return true;
                        }
                    }
                }
            }
        }

        false
    }


    /**
     * @return true if this controller could handle the event, false to pass the event to other controllers
     */
    fn handle_scroll_event(&mut self, ui: &mut UI, event: &ScrollEvent, world: &mut Self::Appdata) -> bool {

        let comp = ui.handle_scroll_event(&event);

        match comp {
            None => {
                let pos = screen_to_world_pos(ui, &world.map.player.position, &ui.mouse_state.position);

                let map = &mut world.map;
                let option = map.find_nearest_object(map.selected_layer, &pos);
        
                match option {
                    None => {
                        println!("Found no object at {}, {}", pos[0], pos[1]);
                    },
                    Some(idx) => {
                        let object = map.layers[map.selected_layer].get_mut(idx).unwrap();
                        println!("Found object {} at scale {}", object.tile_id, object.scale);
                        object.scale += 0.05 * event.dy;
                    }
                }
            },
            Some(_comp) => {
                println!("Scroll event consumed");
            }
        }

        true
    }


    fn draw_overlay(&mut self, viewport: Viewport, gl: &mut GlGraphics, ds: &DrawState, ui: &mut UI, world: &mut Self::Appdata) {
        let layer_id = world.map.selected_layer;
        let id = self.selected_tile_id;
        let set = &world.layer_tileset[layer_id];
        let tile_opt = set.tiles_by_id.get(&id);

        if tile_opt.is_some() {
            let tile = tile_opt.unwrap();
            let player_position = &world.map.player.position;

            let mp = &ui.mouse_state.position;
            let window_center: Vector2<f64> = ui.window_center(); 

            let pos = screen_to_world_pos(&ui, player_position, mp);
            let deco = MapObject::new(id, pos, 1.0);

            gl.draw(viewport, |c, gl| {
                let tf = build_transform(c.transform, &deco, tile.foot, player_position, &window_center);        

                let image = build_image(tile, &deco.color);
                image.draw(&tile.tex, &ds, tf, gl);
            });
        }
        ui.font_14.draw(viewport, gl, ds, 10, 20, "Use keys 1 .. 3 to select map layers", &[1.0, 1.0, 1.0, 1.0]);

        let layer_msg = 
            "Selected layer: ".to_string() + &layer_id.to_string() + 
            "  Selected tile: " + &self.selected_tile_id.to_string();

        ui.font_14.draw(viewport, gl, ds, 10, (ui.window_size[1] - 24) as i32, &layer_msg, &[1.0, 1.0, 1.0, 1.0]);
    }

}


impl MapEditor {

    pub fn new() -> MapEditor {
        MapEditor {
            selected_tile_id: 0,
        }
    }


    fn select_nearest_item(&self, ui: &UI, world: &mut GameWorld) -> bool {
        let pos = screen_to_world_pos(ui, &world.map.player.position, &ui.mouse_state.position);
        let map = &mut world.map;
        let option = map.find_nearest_object(map.selected_layer, &pos);

        match option {
            None => {
                map.has_selection = false;
                map.selected_item = 0;
            },
            Some(idx) => {

                // toggle
                if map.has_selection && map.selected_item == idx {
                    // was already seelected, unselect
                    map.has_selection = false;
                    map.selected_item = 0;
                }
                else {
                    map.has_selection = true;
                    map.selected_item = idx;
                }

                return true;
            }
        }

        false
    }


    pub fn make_tile_selector(&self, ui: &UI, tileset: &TileSet) -> UiComponent {
        // let count = tileset.tiles_by_id.len();
        // let rows = count / 8;
        let size = &ui.window_size;
        
        let ww = size[0] as i32;
        let wh = size[1] as i32;
        let w = 800;
        let h = 600;
        let x_space = 134;
        let y_space = 150;

        let mut cont = ui.make_container(0, 0, w, h);

        let mut x = 0;
        let mut y = 0;

        for i in 0..10000 {

            let id = tileset.tiles_order_to_id.get(&i);
            match id {
                None => {

                },
                Some(id) => {
                    let tile = tileset.tiles_by_id.get(id).unwrap();
                    let icon = ui.make_icon(x+10, y+10, x_space-20, y_space-20, 
                                            tile, &tile.name, *id);
                        
                    cont.head.add_child(icon);
        
                    x += x_space;
        
                    if x >= w {
                        x = 0;
                        y += y_space;
                    }
                }
            }
        }

        let scrolly = ui.make_scrollpane((ww - w)/2, (wh - h)/2, w, h, cont, 64, 64);
        scrolly
    }
}