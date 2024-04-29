#[path = "ui/components.rs"]
mod components;

#[path = "tileset.rs"]
mod tileset;

use std::rc::Rc;

use opengl_graphics::GlGraphics;
use graphics::Viewport;

pub use components::ButtonEvent;
use components::{UiHead, UiButton, UiIcon, UiFont};
use crate::tileset::Tile;

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
    pub head: Box<dyn UiHead>,    
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

    pub fn handle_button_event(&self, event: &ButtonEvent) -> bool {
        for child in &self.children {
            if child.area.contains(event.mx, event.my) {

            }
        }
        

        false
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
    

    pub fn make_button(&self, x: i32, y: i32, w: i32, h: i32) -> UiComponent {
        let button = UiButton {
            font: self.font_14.clone(),
            label: "Hello World!".to_string(),    
        };
        
        UiComponent {
            area: UiArea {
                x, 
                y,
                w,
                h,                
            }, 
        
            head: Box::new(button),
        }        
    }
    

    pub fn make_icon(&self, x: i32, y: i32, w: i32, h: i32, tile: &Rc<Tile>, label: &str) -> UiComponent {
        let icon = UiIcon {
            font: self.font_10.clone(),
            label: label.to_string(),
            tile: tile.clone(),    
        };
        
        UiComponent {
            area: UiArea {
                x, 
                y,
                w,
                h,                
            }, 
        
            head: Box::new(icon),
        }        
    }

    
    pub fn draw(&self, viewport: Viewport, gl: &mut GlGraphics) {
        match &self.root {
            None => { }
            Some(cont) => {
                cont.draw(viewport, gl);
            }
        }
    }

    pub fn handle_button_event(&self, event: &ButtonEvent) -> bool {

        match &self.root {
            None => { }
            Some(cont) => {
                cont.handle_button_event(event);
            }
        }

        false
    }
}
