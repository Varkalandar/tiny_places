use piston::{ButtonState, MouseButton};
use graphics::DrawState;
use opengl_graphics::GlGraphics;
use graphics::Viewport;

use crate::ui::{UI, UiController, ButtonEvent, MouseMoveEvent, ScrollEvent};
use crate::GameWorld;
use crate::screen_to_world_pos;
use crate::player_inventory_view::PlayerInventoryView;
use crate::TileSet;

pub struct Game {
    piv: PlayerInventoryView,
    show_inventory: bool,
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
                            Some(_idx) => {
                                // pick up the item?
                                return true;
                            }
                        }
                    }

                    if event.args.button == piston::Button::Keyboard(piston::Key::I) {
                        self.show_inventory = true;
                    }        
                },
                Some(_comp) => {
                }
            }
        }

        if self.show_inventory {
            return self.piv.handle_button_event(event, &ui.mouse_state, &mut world.player_inventory);
        }

        false
    }


    fn handle_mouse_move_event(&mut self, ui: &mut UI, event: &MouseMoveEvent, world: &mut Self::Appdata) -> bool {
        let _comp = ui.handle_mouse_move_event(event);

        if self.show_inventory {
            self.piv.handle_mouse_move_event(event, &ui.mouse_state, &mut world.player_inventory);
        }

        false
    }


    /**
     * @return true if this controller could handle the event, false to pass the event to other controllers
     */
    fn handle_scroll_event(&mut self, ui: &mut UI, event: &ScrollEvent, _world: &mut Self::Appdata) -> bool {

        let _comp = ui.handle_scroll_event(event);

        false
    }


    fn draw(&mut self, viewport: Viewport, gl: &mut GlGraphics, ds: &DrawState, ui: &mut UI, world: &mut Self::Appdata) {
        ui.draw(viewport, gl);
 
        if self.show_inventory {
            self.piv.draw(viewport, gl, ds, 0, 10, &world.player_inventory)
        }
    }


    fn draw_overlay(&mut self, viewport: Viewport, gl: &mut GlGraphics, ds: &DrawState, ui: &mut UI, _world: &mut Self::Appdata) {
        ui.font_14.draw(viewport, gl, ds, 10, 20, "Game testing mode", &[1.0, 1.0, 1.0, 1.0]);
    }


    fn update(&mut self, _world: &mut Self::Appdata) {

    }
}


impl Game {

    pub fn new(ui: &UI, item_tiles: &TileSet) -> Game {
        let piv = PlayerInventoryView::new((ui.window_size[0] as i32) / 2, 0,
        &ui.font_14,
        &item_tiles.shallow_copy());
    
        Game {
            piv,
            show_inventory: false,
        }
    }
}