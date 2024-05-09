use std::path::Path;

use graphics::{draw_state::DrawState, Rectangle, Viewport, ImageSize, Image,};
use opengl_graphics::{GlGraphics, Texture, TextureSettings};

use crate::ui::{UiHead, UiComponent, UiArea};


pub struct PlayerInventoryView {
    area: UiArea,
    texture: Texture,
}


impl PlayerInventoryView {
    pub fn new(x: i32, y: i32) -> UiComponent {

        let texture = Texture::from_path(Path::new("resources/ui/inventory_bg.png"), &TextureSettings::new()).unwrap();

        let inv = PlayerInventoryView {
            area: UiArea {
                x, 
                y,
                w: 400,
                h: 600,                
            },
            
            texture,
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
            /*
            let rect = Rectangle::new([0.0, 0.0, 0.0, 0.3]); 
            rect.draw([xp as f64, yp as f64, area.w as f64, area.h as f64], draw_state, c.transform, gl);
            */

            let tf = c.transform;
            let m_image   = 
                Image::new()
                    .rect([xp as f64, yp as f64, self.texture.get_width() as f64, self.texture.get_height() as f64])
                    .color([1.0, 1.0, 1.0, 0.9]);
            m_image.draw(&self.texture, draw_state, tf, gl);

        });
    }
}