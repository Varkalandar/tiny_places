use graphics::{draw_state::DrawState, Rectangle, Viewport};
use opengl_graphics::GlGraphics;


pub trait UiHead {
    /*
    fn is_inside(&self, x: i32, y:i32) -> bool {
        x >= self.area.x && y >= self.y && x <= self.x + self.w && y <= self.y + self.h  
    }
    */
    
    fn handle_event(&self) -> bool {
        false
    } 

    fn draw(&self, viewport: &Viewport, gl: &mut GlGraphics, x: i32, y: i32, w: i32, h: i32) {
    } 
}


pub struct UiButton {
    
}

impl UiHead for UiButton {
    
    fn draw(&self, viewport: &Viewport, gl: &mut GlGraphics, x: i32, y: i32, w: i32, h: i32) {

        gl.draw(*viewport, |c, gl| {
            let rect = Rectangle::new([1.0, 0.5, 0.0, 1.0]); 
            rect.draw([100.0, 100.0, 500.0, 300.0], &DrawState::new_alpha(), c.transform, gl)
            // rectangle([1.0, 0.5, 0.0, 1.0], rect, c.transform, gl);
        });
    } 
}