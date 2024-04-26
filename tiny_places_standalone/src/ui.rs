#[path = "ui/components.rs"]
mod components;

#[path = "ui/font.rs"]
mod font;

use std::rc::Rc;

use components::{UiHead, UiButton};
use font::UiFont;
use opengl_graphics::GlGraphics;
use graphics::Viewport;


pub struct UiArea {
    pub x: i32, 
    pub y: i32,
    pub w: i32,
    pub h: i32,
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
    pub fn draw(&self, viewport: &Viewport, gl: &mut GlGraphics) {
        for i in 0..self.children.len() {
            let child = &self.children[i];    
            let a = &child.area;
            child.head.draw(viewport, gl, a.x, a.y, a.w, a.h);
        }
    }
}


pub struct UI
{
    pub root: Option<UiContainer>,
    font: UiFont,
}


impl UI {
    pub fn new() -> UI {
        
        UI { 
            root: None,
            font: UiFont::new(),
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
        UiComponent {
            area: UiArea {
                x, 
                y,
                w,
                h,                
            }, 
        
            head: Box::new(UiButton {} ),
        }        
    }
    
    
    pub fn draw(&self, viewport: &Viewport, gl: &mut GlGraphics) {
        match &self.root {
            None => { }
            Some(cont) => {
                cont.draw(viewport, gl);
            }
        }
    }
}

