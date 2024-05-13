use std::rc::Rc;

use piston::{ButtonState, MouseButton};
use graphics::DrawState;
use opengl_graphics::GlGraphics;
use graphics::Viewport;

use crate::ui::{UI, UiController, ButtonEvent, ScrollEvent};
use crate::GameWorld;
use crate::screen_to_world_pos;
use crate::player_inventory_view::PlayerInventoryView;


pub struct Game {
}


impl UiController for Game {
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
                        ui.root.head.clear();

                        let pos = screen_to_world_pos(&ui, &world.map.player.position, &ui.mouse_state.position);
                        let map = &mut world.map;
                        let option = map.find_nearest_object(map.selected_layer, &pos);

                        match option {
                            None => {
                                // nothing clicked -> move player
                                map.has_selection = false;
                            },
                            Some(idx) => {
                                // pick up the item?
                                return true;
                            }
                        }
                    }

                    if event.args.button == piston::Button::Keyboard(piston::Key::I) {
                        let piv = PlayerInventoryView::new(((ui.window_size[0] - 500) / 2) as i32, 10, 
                                                           world.player_inventory.clone(),
                                                           &world.layer_tileset[6].shallow_copy());
                        ui.root.head.add_child(Rc::new(piv));
                    }        
                },
                Some(comp) => {
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

        false
    }


    fn draw_overlay(&mut self, viewport: Viewport, gl: &mut GlGraphics, ds: &DrawState, ui: &mut UI, world: &mut Self::Appdata) {
        ui.font_14.draw(viewport, gl, ds, 10, 20, "Game testing mode", &[1.0, 1.0, 1.0, 1.0]);
    }

}


impl Game {

    pub fn new() -> Game {
        Game {
        }
    }
}