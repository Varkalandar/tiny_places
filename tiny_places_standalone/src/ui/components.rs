use std::rc::Rc;

use graphics::{draw_state::DrawState, Rectangle, Viewport, Image};
use opengl_graphics::GlGraphics;
use piston::ButtonArgs;


#[path = "../tileset.rs"]
mod tileset;
mod font;

pub use font::UiFont;
use crate::tileset::Tile;

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

        self.font.draw(viewport, gl, label_x, label_y, &self.label, &[1.0, 1.0, 1.0, 1.0]);
    } 
}



pub struct UiIcon<F>
    where F: Fn(usize) -> usize,
{
    pub font: Rc<UiFont>,
    pub label: String,
    pub tile: Rc<Tile>,
    pub callback: F,
    pub userdata: usize,
}


impl <F> UiHead for UiIcon <F>
    where F: Fn(usize) -> usize,
{
    fn draw(&self, viewport: Viewport, gl: &mut GlGraphics, x: i32, y: i32, w: i32, h: i32) {

        gl.draw(viewport, |c, gl| {

            let rect = Rectangle::new([0.1, 0.1, 0.1, 1.0]); 
            rect.draw([x as f64, y as f64, w as f64, h as f64], &DrawState::new_alpha(), c.transform, gl);

            let tw = self.tile.size[0] * 0.25;
            let th = self.tile.size[1] * 0.25;

            let y_base = y + h - 26; // space for a label below the icon image

            let image_x = x + (w - tw as i32) / 2;
            let image_y = y_base - th as i32;

            let image   = 
                Image::new()
                    .rect([image_x as f64, image_y as f64, tw, th])
                    .color([1.0, 1.0, 1.0, 1.0]);
            image.draw(&self.tile.tex, &DrawState::new_alpha(), c.transform, gl);
        });

        let label_width = self.font.calc_string_width(&self.label) as i32;
        let label_x = x + (w - label_width) / 2;
        let label_y = y + h - self.font.lineheight;

        self.font.draw(viewport, gl, label_x, label_y, &self.label, &[0.4, 0.6, 0.7, 1.0]);
    } 

    fn handle_button_event(&self, event: &ButtonEvent) -> bool {
        (self.callback)(self.userdata);
        true
    }

}