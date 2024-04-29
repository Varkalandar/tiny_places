use std::rc::Rc;

use graphics::{draw_state::DrawState, Rectangle, Viewport};
use opengl_graphics::GlGraphics;
use piston::ButtonArgs;

mod font;
pub use font::UiFont;


pub struct ButtonEvent<'a> {
    pub args: &'a ButtonArgs,
    pub mx: i32,
    pub my: i32,
}


pub trait UiHead {
    
    fn handle_button_event(&self, event: &ButtonEvent) -> bool {
        false
    }

    fn draw(&self, _viewport: Viewport, _gl: &mut GlGraphics, _x: i32, _y: i32, _w: i32, _h: i32) {
    } 
}


pub struct UiButton {
    pub font: Rc<UiFont>,
    pub label: String,
}


impl UiHead for UiButton {
    
    fn draw(&self, viewport: Viewport, gl: &mut GlGraphics, x: i32, y: i32, w: i32, h: i32) {

        gl.draw(viewport, |c, gl| {

            let rect = Rectangle::new([1.0, 0.5, 0.0, 1.0]); 
            rect.draw([x as f64, y as f64, w as f64, h as f64], &DrawState::new_alpha(), c.transform, gl)
        });

        let label_width = self.font.calc_string_width(&self.label) as i32;
        let label_x = x + (w - label_width) / 2;
        let label_y = y + (h - self.font.lineheight) / 2;

        self.font.draw(viewport, gl, label_x, label_y, &self.label);
    } 
}