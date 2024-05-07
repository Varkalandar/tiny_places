use graphics::{draw_state::DrawState, Rectangle, Viewport,};
use opengl_graphics::GlGraphics;

use crate::ui::{UiHead, UiComponent, UiArea};


pub struct PlayerInventoryView {
    area: UiArea,
}


impl PlayerInventoryView {
    pub fn new(x: i32, y: i32) -> UiComponent {
        let inv = PlayerInventoryView {
            area: UiArea {
                x, 
                y,
                w: 400,
                h: 600,                
            }, 
        };

        UiComponent {
            head: Box::new(inv),
        }
    }
}


impl UiHead for PlayerInventoryView {


    fn draw(&self, viewport: Viewport, gl: &mut GlGraphics, draw_state: &DrawState, x: i32, y: i32) {
        let area = &self.area;
        let xp = x + area.x;
        let yp = y + area.y;

        gl.draw(viewport, |c, gl| {

            // placeholder

            let rect = Rectangle::new([0.0, 0.0, 0.0, 0.3]); 
            rect.draw([xp as f64, yp as f64, area.w as f64, area.h as f64], draw_state, c.transform, gl);
        });
    }
}