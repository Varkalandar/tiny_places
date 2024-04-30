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
