#[path = "ui/components.rs"]
mod components;
use std::rc::Rc;

use components::{UiHead, UiButton};
use graphics::Viewport;
use opengl_graphics::GlGraphics;


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


/*
impl UiComponent {
    fn is_inside(&self, x: i32, y:i32) -> bool {
        x >= self.x && y >= self.y && x <= self.x + self.w && y <= self.y + self.h  
    }
}
*/

pub struct UI
{
    pub root: Option<UiContainer>,
}


impl UI {
    pub fn new() -> UI {
        UI { 
            root: None,
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

