use std::rc::Rc;

use graphics::{draw_state::DrawState, Rectangle, Viewport};
use opengl_graphics::GlGraphics;

mod font;
pub use font::UiFont;

pub trait UiHead {
    /*
    fn is_inside(&self, x: i32, y:i32) -> bool {
        x >= self.area.x && y >= self.y && x <= self.x + self.w && y <= self.y + self.h  
    }
    */
    
    fn handle_event(&self) -> bool {
        false
    } 

    fn draw(&self, _viewport: Viewport, _gl: &mut GlGraphics, _x: i32, _y: i32, _w: i32, _h: i32) {
    } 
}


pub struct UiButton {
    pub font: Rc<UiFont>,
}


impl UiHead for UiButton {
    
    fn draw(&self, viewport: Viewport, gl: &mut GlGraphics, x: i32, y: i32, w: i32, h: i32) {

        gl.draw(viewport, |c, gl| {

            let rect = Rectangle::new([1.0, 0.5, 0.0, 1.0]); 
            rect.draw([x as f64, y as f64, w as f64, h as f64], &DrawState::new_alpha(), c.transform, gl)
            // rectangle([1.0, 0.5, 0.0, 1.0], rect, c.transform, gl);
            
        });

        self.font.draw(viewport, gl, x, y, "Hello World!");
    } 
}