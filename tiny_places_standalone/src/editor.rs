use piston::{ButtonState, MouseButton};

use crate::ui::{UI, UiController, UiComponent, TileSet, ButtonEvent, ScrollEvent};
use crate::map::{MapObject, MAP_DECO_LAYER};
use crate::screen_to_world_pos;
use crate::GameWorld;

use std::rc::Rc;


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


        if event.args.state == ButtonState::Release {
            let comp = ui.handle_button_event(&event);

            match comp {
                None => {
                    
                    if event.args.button == piston::Button::Mouse(MouseButton::Right) {
                        let pos = screen_to_world_pos(&ui, &world.map.player.position, &ui.mouse_state.position);
                        let id = self.selected_tile_id;
    
                        println!("creating deco {} at {:?}", id, pos);
                        let deco = MapObject::new(id, pos, 1.0);
                        world.map.layers[MAP_DECO_LAYER].push(deco);
                    }
                    
                    if event.args.button == piston::Button::Keyboard(piston::Key::Space) {
                        let cont = self.make_tile_selector(&ui, &world.decoration_tiles);
                        ui.root.head.add_child(Rc::new(cont));
                    }        

                    if event.args.button == piston::Button::Keyboard(piston::Key::C) {
                        let cont = ui.make_color_choice(100, 100, 256, 256, 1000);
                        ui.root.head.add_child(cont);
                    }        

                    if event.args.button == piston::Button::Keyboard(piston::Key::L) {
                        world.map.load("test.map");
                    }        

                    if event.args.button == piston::Button::Keyboard(piston::Key::S) {
                        world.map.save("test.map").unwrap();
                    }        
                },
                Some(comp) => {
                    let id = comp.get_userdata();

                    println!("Selected tile id={}", id);

                    if id > 0 {
                        self.selected_tile_id = id;
                        ui.root.head.clear();

                        return true;
                    }
                }
            }
        }

        false
    }


    /**
     * @return true if this controller could handle the event, false to pass the event to other controllers
     */
     fn handle_scroll_event(&mut self, _ui: &mut UI, _event: &ScrollEvent, _world: &mut Self::Appdata) -> bool {
        false
    }
}


impl MapEditor {

    pub fn new() -> MapEditor {
        MapEditor {
            selected_tile_id: 0,
        }
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

        // let scrolly = ui.make_scrollpane((ww - w)/2, (wh - h)/2, w, h, cont);
        let scrolly = ui.make_scrollpane(0, 0, w, h, cont);
        scrolly
    }
}