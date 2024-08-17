use std::rc::Rc;

use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use vecmath::Vector2;

use crate::ui::{UI, UiController, UiComponent, TileSet, Button, ButtonState, ButtonEvent, MouseMoveEvent, ScrollEvent};
use crate::map::{MAP_GROUND_LAYER, MAP_OBJECT_LAYER, MAP_CLOUD_LAYER};
use crate::{screen_to_world_pos, build_transform, build_image};
use crate::GameWorld;
use crate::sound::Sound;


pub struct MapEditor {
    pub selected_tile_id: usize,
    pub show_editor_keys: bool,
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
                    
                    if event.args.button == Button::Mouse(MouseButton::Left) {
                        let id = self.selected_tile_id;

                        if id == 0 {
                            let ok = self.select_nearest_item(ui, world);
                            return ok;
                        }
                        else {
                            let pos = screen_to_world_pos(&ui, &world.map.player_position(), &ui.mouse_state.position);
                            world.speaker.play(Sound::Click, 0.5);
                            println!("creating map object {} at {:?}", id, pos);
                            
                            let map = &mut world.map;
                            let layer = map.selected_layer;
                            let height = world.layer_tileset[layer].tiles_by_id.get(&id).unwrap().foot[1];
                            let mob = map.factory.create_mob(id, layer, pos, height, 1.0);
                            let mob_id = mob.uid;
                            map.layers[layer].insert(mob_id, mob);

                            return true;
                        }
                    }

                    if event.args.button == Button::Mouse(MouseButton::Right) {

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
                    
                    if event.args.button == Button::Keyboard(Keycode::F1) { 
                        self.show_editor_keys = !self.show_editor_keys;
                    }        

                    if event.args.button == Button::Keyboard(Keycode::Num1) {                        
                        world.map.selected_layer = MAP_GROUND_LAYER;
                        self.selected_tile_id = 0;
                    }        

                    if event.args.button == Button::Keyboard(Keycode::Num2) {                        
                        world.map.selected_layer = MAP_OBJECT_LAYER;
                        self.selected_tile_id = 0;
                    }        

                    if event.args.button == Button::Keyboard(Keycode::Num3) {                        
                        world.map.selected_layer = MAP_CLOUD_LAYER;
                        self.selected_tile_id = 0;
                    }        

                    let step = if ui.keyboard_state.shift_pressed {8.0} else {1.0};

                    if event.args.button == Button::Keyboard(Keycode::Right) {                        
                        world.map.move_selected_object(step, 0.0);
                    }        

                    if event.args.button == Button::Keyboard(Keycode::Left) {                        
                        world.map.move_selected_object(-step, 0.0);
                    }        

                    if event.args.button == Button::Keyboard(Keycode::Up) {  
                        world.map.move_selected_object(0.0, -step);
                    }        

                    if event.args.button == Button::Keyboard(Keycode::Down) {                        
                        world.map.move_selected_object(0.0, step);
                    }        


                    if event.args.button == Button::Keyboard(Keycode::Space) {
                        let set = &world.layer_tileset[world.map.selected_layer];
                        let cont = self.make_tile_selector(&ui, set);
                        ui.root.head.add_child(Rc::new(cont));
                    }        

                    if event.args.button == Button::Keyboard(Keycode::A) {
                        let map = &mut world.map;
                        map.apply_to_selected_mob(|mob| {mob.visual.blend = sdl2::render::BlendMode::Add;});
                    }

                    if event.args.button == Button::Keyboard(Keycode::C) {
                        let map = &mut world.map;
                        let object = map.layers[map.selected_layer].get_mut(&map.selected_item);
                        match object {
                            None => {},
                            Some(mob) => {
                                let color_choice = ui.make_color_choice(100, 100, 256, 256, 1000, mob.visual.color);
                                ui.root.head.add_child(Rc::new(color_choice));
                            }
                        }
                    }        

                    if event.args.button == Button::Keyboard(Keycode::Delete) {
                        let map = &mut world.map;
                        let mob = map.layers[map.selected_layer].get(&map.selected_item);

                        match mob {
                            None => {}
                            Some(mob) => {
                                let uid = mob.uid;
                                let layer = &mut map.layers[map.selected_layer];
                                layer.remove(&uid);
                            }
                        }
                    }        

                    if event.args.button == Button::Keyboard(Keycode::L) {
                        world.map.load("start.map");
                    }

                    if event.args.button == Button::Keyboard(Keycode::M) {
                        let map = &mut world.map;
                        map.apply_to_selected_mob(|mob| {mob.visual.blend = sdl2::render::BlendMode::Blend;});
                    }

                    if event.args.button == Button::Keyboard(Keycode::P) {
                        let pos = screen_to_world_pos(&ui, &world.map.player_position(), &ui.mouse_state.position);
                        place_particle_generator(world, pos);
                    }

                    if event.args.button == Button::Keyboard(Keycode::S) {
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
                            let object = map.layers[map.selected_layer].get_mut(&map.selected_item);
                            match object {
                                None => {},
                                Some(mob) => {
                                    mob.visual.color = [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0]
                                }
                            }
                        }

                        return true;
                    }
                    else {
                        // must have been the tile selector
                        println!("Selected tile id={}", id);

                        if id > 0 {
                            world.speaker.play(Sound::Click, 0.5);
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
                let pos = screen_to_world_pos(ui, &world.map.player_position(), &ui.mouse_state.position);

                let map = &mut world.map;
                let option = map.find_nearest_object(map.selected_layer, &pos, 100.0, 0);
        
                match option {
                    None => {
                        println!("Found no object at {}, {}", pos[0], pos[1]);
                    },
                    Some(id) => {
                        let mob = map.layers[map.selected_layer].get_mut(&id).unwrap();
                        println!("Found object {} at scale {}", mob.uid, mob.visual.scale);
                        mob.visual.scale += 0.05 * event.dy;
                    }
                }
            },
            Some(_comp) => {
                println!("Scroll event consumed");
            }
        }

        true
    }

    fn handle_mouse_move_event(&mut self, ui: &mut UI, _event: &MouseMoveEvent, world: &mut Self::Appdata) -> bool {

        let player_position = &world.map.player_position();
        let mp = &ui.mouse_state.position;
        let pos = screen_to_world_pos(&ui, player_position, mp);

        // Dragging?
        if ui.mouse_state.left_pressed {
            let map = &mut world.map; 
            let option = map.find_nearest_object(map.selected_layer, &pos, 100.0, 0);

            match option {
                None => {
                    // Nothing to do
                },
                Some(id) => {
                    if map.selected_item == id {

                        let mob = &mut map.layers[map.selected_layer].get_mut(&id).unwrap();
                        mob.position = pos;
                    }
                }
            }
        }

        false
    }



    fn draw(&mut self, ui: &mut UI, _world: &mut Self::Appdata) {
        // ui.draw(viewport, gl);
    }


    fn draw_overlay(&mut self, ui: &mut UI, world: &mut Self::Appdata) {
        let layer_id = world.map.selected_layer;
        let id = self.selected_tile_id;
        let set = &world.layer_tileset[layer_id];
        let tile_opt = set.tiles_by_id.get(&id);

        if tile_opt.is_some() {
            let tile = tile_opt.unwrap();
            let player_position = &world.map.player_position();

            let mp = &ui.mouse_state.position;
            let window_center: Vector2<f64> = ui.window_center(); 

            let pos = screen_to_world_pos(&ui, player_position, mp);
            // let object = world.map.factory.create_mob(id, MAP_OBJECT_LAYER, pos, 1.0);
/*
            gl.draw(viewport, |c, gl| {
                let tf = build_transform(&c.transform, &pos, 1.0, tile.foot, player_position, &window_center);        

                let image = build_image(tile, [1.0, 1.0, 1.0, 0.5]);
                image.draw(&tile.tex, &ds, tf, gl);
            });
            */
        }
        ui.font_14.draw(10, 20, "Press F1 to see editor hotkeys", &[1.0, 1.0, 1.0, 1.0]);
        ui.font_14.draw(10, 40, "Press g to enter game mode", &[1.0, 1.0, 1.0, 1.0]);

        let layer_msg = 
            "Selected layer: ".to_string() + &layer_id.to_string() + 
            "  Selected tile: " + &self.selected_tile_id.to_string();

        ui.font_14.draw(10, (ui.window_size[1] - 24) as i32, &layer_msg, &[1.0, 1.0, 1.0, 1.0]);


        if self.show_editor_keys {
            let color = [1.0, 1.0, 1.0, 1.0];
            let line_space = 20;
            let left = 100;
            let mut top = 100;

            ui.font_14.draw(left, top, "F1: Show/hide this list", &color);
            top += line_space;
            ui.font_14.draw(left, top, "Space: Open tile selector", &color);
            top += line_space;
            ui.font_14.draw(left, top, "1,2,3,.. : Select map layer", &color);
            top += line_space;
            ui.font_14.draw(left, top, "c: Open color selector for selected item", &color);
            top += line_space;
            ui.font_14.draw(left, top, "a: Set blend mode on selected item to 'Addition'", &color);
            top += line_space;
            ui.font_14.draw(left, top, "m: Set blend mode on selected item to 'Mix' (default)", &color);
            top += line_space;
            ui.font_14.draw(left, top, "Delete: Removes the selected item from the map", &color);
            top += line_space;
            ui.font_14.draw(left, top, "l: Load a saved map", &color);
            top += line_space;
            ui.font_14.draw(left, top, "s: Save the map", &color);
            top += line_space;
        }

    }


    fn update(&mut self, world: &mut Self::Appdata, dt: f64) {
        let map = &mut world.map;
        let rng = &mut world.rng;
        let speaker = &mut world.speaker;

        map.update(dt, rng, speaker);
    }
}


impl MapEditor {

    pub fn new() -> MapEditor {
        MapEditor {
            selected_tile_id: 0,
            show_editor_keys: false,
        }
    }


    fn select_nearest_item(&self, ui: &UI, world: &mut GameWorld) -> bool {
        let pos = screen_to_world_pos(ui, &world.map.player_position(), &ui.mouse_state.position);
        let map = &mut world.map;
        let option = map.find_nearest_object(map.selected_layer, &pos, 100.0, 0);

        match option {
            None => {
                map.has_selection = false;
                map.selected_item = 0;
            },
            Some(id) => {

                // toggle
                if map.has_selection && map.selected_item == id {
                    // was already seelected, unselect
                    map.has_selection = false;
                    map.selected_item = 0;
                }
                else {
                    map.has_selection = true;
                    map.selected_item = id;
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


fn place_particle_generator(world: &mut GameWorld, pos: Vector2<f64>) {
    let id = 212;
    let map = &mut world.map;
    let layer = MAP_OBJECT_LAYER;
    let height = world.layer_tileset[layer].tiles_by_id.get(&id).unwrap().foot[1];
    let mut mob = map.factory.create_mob(id, layer, pos, height, 1.0);
    let mob_id = mob.uid;

    let particles = &mut mob.visual.particles;

    let ids = [44, 45, 46, 47, 48, 49, 50, 51];

    for id in ids {
        particles.spawn_ids.push(id+8);
    }

    particles.spawn_chance = 30.0;
    particles.spawn_tile_set = MAP_CLOUD_LAYER;

    map.layers[layer].insert(mob_id, mob);
}

