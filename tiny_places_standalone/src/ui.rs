#[path = "ui/tileset.rs"]
mod tileset;

#[path = "ui/font.rs"]
mod font;

use std::rc::Rc;

use graphics::{draw_state::DrawState, Rectangle, Viewport, Image};
use opengl_graphics::GlGraphics;
use piston::ButtonArgs;

pub use tileset::*;
pub use font::UiFont;


pub struct UiArea {
    pub x: i32, 
    pub y: i32,
    pub w: i32,
    pub h: i32,
}


impl UiArea {
    fn contains(&self, x: i32, y:i32) -> bool {
        x >= self.x && y >= self.y && x <= self.x + self.w && y <= self.y + self.h  
    }
}


pub struct UiComponent {
    pub area: UiArea,
    pub userdata: usize, 
    pub head: Rc<dyn UiHead>,    
}


pub struct UiContainer {
    pub area: UiArea, 
    
    pub children: Vec<Rc<UiComponent>>,
}


impl UiContainer {
    pub fn draw(&self, viewport: Viewport, gl: &mut GlGraphics) {
        for i in 0..self.children.len() {
            let child = &self.children[i];    
            let a = &child.area;

            child.head.draw(viewport, gl, self.area.x + a.x, self.area.y + a.y, a.w, a.h);
        }
    }

    pub fn handle_button_event(&self, event: &ButtonEvent) -> Option<&UiComponent> {

        // println!("Container handles button event");

        for child in &self.children {
            let rel_x = event.mx - self.area.x;
            let rel_y = event.my - self.area.y;

            println!("click at relpos {}, {} area {}, {}, {}, {}",
                rel_x, rel_y, child.area.x, child.area.y, child.area.w, child.area.h);
            if child.area.contains(rel_x, rel_y) {
                println!("Found a child to handle event");
                return Some(&child);
            }
        }

        None
    }
}


pub struct UI
{
    pub root: Option<UiContainer>,
    font_10: Rc<UiFont>,
    font_14: Rc<UiFont>,
    
    pub window_size: [u32; 2]
}


impl UI {
    pub fn new(window_size: [u32; 2]) -> UI {
        
        UI { 
            window_size,
            root: None,
            font_10: Rc::new(UiFont::new(10)),
            font_14: Rc::new(UiFont::new(14)),
        }
    }

    pub fn make_container(&self, x: i32, y: i32, w: i32, h: i32) -> UiContainer {

        UiContainer {
            area: UiArea {
                x, 
                y,
                w,
                h,                
            }, 
        
            children: Vec::new(),
        }        
    }
    

    pub fn make_button(&self, x: i32, y: i32, w: i32, h: i32, label: &str, userdata: usize) -> UiComponent {
        let button = UiButton {
            font: self.font_14.clone(),
            label: label.to_string(),    
        };
        
        UiComponent {
            area: UiArea {
                x, 
                y,
                w,
                h,                
            }, 
            userdata,
            head: Rc::new(button),
        }        
    }


    pub fn make_icon(&self, x: i32, y: i32, w: i32, h: i32, 
                     tile: &Rc<Tile>, label: &str, userdata: usize) -> Rc<UiComponent> {
        let icon = UiIcon {
            font: self.font_10.clone(),
            label: label.to_string(),
            tile: tile.clone(),
            userdata,
        };
        
        Rc::new(UiComponent {
            area: UiArea {
                x, 
                y,
                w,
                h,                
            }, 
            userdata,
            head: Rc::new(icon),
        })        
    }

    
    pub fn draw(&self, viewport: Viewport, gl: &mut GlGraphics) {
        match &self.root {
            None => { }
            Some(cont) => {
                cont.draw(viewport, gl);
            }
        }
    }

    pub fn handle_button_event(&self, event: &ButtonEvent) -> Option<&UiComponent> {

        match &self.root {
            None => { 
            }
            Some(cont) => {
                return cont.handle_button_event(event);
            }
        }

        None
    }
}



pub struct ButtonEvent<'a> {
    pub args: &'a ButtonArgs,
    pub mx: i32,
    pub my: i32,
}


pub trait UiHead {

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



pub struct UiIcon
{
    pub font: Rc<UiFont>,
    pub label: String,
    pub tile: Rc<Tile>,
    pub userdata: usize,
}


impl UiHead for UiIcon
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
}
