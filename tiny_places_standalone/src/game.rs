use graphics::DrawState;
use opengl_graphics::GlGraphics;
use graphics::Viewport;

use crate::ui::{UI, UiController, ButtonEvent, ScrollEvent};
use crate::GameWorld;

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
/*
        ui.font_14.draw(viewport, gl, ds, 10, 20, "Use keys 1 .. 3 to select map layers", &[1.0, 1.0, 1.0, 1.0]);

        let layer_msg = 
            "Selected layer: ".to_string() + &world.map.selected_layer.to_string() + 
            "  Selected tile: " + &self.selected_tile_id.to_string();

        ui.font_14.draw(viewport, gl, ds, 10, (ui.window_size[1] - 24) as i32, &layer_msg, &[1.0, 1.0, 1.0, 1.0]);
*/
    }

}


impl Game {

    pub fn new() -> Game {
        Game {
        }
    }
}