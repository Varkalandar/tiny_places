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
    pub head: Box<dyn UiHead>,    
}


pub struct UI
{
    pub root: Option<UiComponent>,
    font_10: Rc<UiFont>,
    font_14: Rc<UiFont>,
    
    pub window_size: [u32; 2],
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

    pub fn make_container(&self, x: i32, y: i32, w: i32, h: i32) -> UiComponent {
        let container = UiContainer {
            area: UiArea {
                x, 
                y,
                w,
                h,                
            }, 
            children: Vec::new(),
        };

        UiComponent {
            head: Box::new(container),
        }        
    }
    

    pub fn make_button(&self, x: i32, y: i32, w: i32, h: i32, label: &str, _userdata: usize) -> UiComponent {
        let button = UiButton {
            area: UiArea {
                x, 
                y,
                w,
                h,                
            }, 
            font: self.font_14.clone(),
            label: label.to_string(),    
        };
        
        UiComponent {
            head: Box::new(button),
        }        
    }


    pub fn make_icon(&self, x: i32, y: i32, w: i32, h: i32, 
                     tile: &Rc<Tile>, label: &str, userdata: usize) -> Rc<UiComponent> {
        let icon = UiIcon {
            area: UiArea {
                x, 
                y,
                w,
                h,                
            }, 
            font: self.font_10.clone(),
            label: label.to_string(),
            tile: tile.clone(),
            userdata,
        };
        
        Rc::new(UiComponent {
            head: Box::new(icon),
        })        
    }

    
    pub fn make_scrollpane(&self, x: i32, y: i32, w: i32, h: i32, child: UiComponent) -> UiComponent {
        let scrollpane = UiScrollpane {
            area: UiArea {
                x, 
                y,
                w,
                h,                
            }, 
            child,
            offset_x: 0,
            offset_y: 0,
            scroll_step_x: 8,
            scroll_step_y: 8,
        };
        
        UiComponent {
            head: Box::new(scrollpane),
        }        
    }


    pub fn draw(&mut self, viewport: Viewport, gl: &mut GlGraphics) {
        match &self.root {
            None => { }
            Some(comp) => {
                let draw_state = DrawState::new_alpha().scissor([0, 0, self.window_size[0], self.window_size[1]]);
                comp.head.draw(viewport, gl, &draw_state, 0, 0);
            }
        }
    }


    pub fn handle_button_event(&mut self, event: &ButtonEvent) -> Option<&dyn UiHead> {

        match &mut self.root {
            None => { 
            }
            Some(comp) => {
                return comp.head.handle_button_event(event);
            }
        }

        None
    }


    pub fn handle_scroll_event(&mut self, event: &ScrollEvent) -> Option<&dyn UiHead> {

        match &mut self.root {
            None => { 
            }
            Some(comp) => {
                return comp.head.handle_scroll_event(event);
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

impl ButtonEvent <'_> {
    fn translate(&self, x: i32, y: i32) -> ButtonEvent {
        ButtonEvent {
            args: self.args,
            mx: self.mx + x,
            my: self.my + y,
        }
    }
}


pub struct ScrollEvent {
    pub dx: i32,
    pub dy: i32,
    pub mx: i32,
    pub my: i32,
}


pub trait UiHead {

    fn area(&self) -> &UiArea {
        &UiArea {
            x: 0, y: 0, w: 0, h:0,
        }
    }

    fn draw(&self, _viewport: Viewport, _gl: &mut GlGraphics, _draw_state: &DrawState, _x: i32, _y: i32) {
    } 

    fn handle_button_event(&mut self, _event: &ButtonEvent) -> Option<&dyn UiHead> {
        println!("This component cannot handle button events.");
        None
    }

    fn handle_scroll_event(&mut self, _event: &ScrollEvent) -> Option<&dyn UiHead> {
        None
    }

    fn add_child(&mut self, child: Rc<UiComponent>) {
        println!("This component cannot store children.");
    }

    fn get_userdata(&self) -> usize {
        0
    }
}


pub struct UiContainer {
    pub area: UiArea,
    pub children: Vec<Rc<UiComponent>>,
}


impl UiContainer {

    fn find_child_at(&mut self, x: i32, y: i32) -> Option<&mut UiComponent> {

        let rel_x = x - self.area.x;
        let rel_y = y - self.area.y;

        for child in &mut self.children {
            let area = &child.head.area();

            if area.contains(rel_x, rel_y) {
                println!("Found a child at {}, {}", x, y);

                return Rc::<UiComponent>::get_mut(child);
            }
        }

        None
    }
}


impl UiHead for UiContainer {
    
    fn area(&self) -> &UiArea {
        &self.area
    }


    fn draw(&self, viewport: Viewport, gl: &mut GlGraphics, draw_state: &DrawState, x: i32, y: i32) {
        // draw only children which are inside visible area
        let scissor = draw_state.scissor.unwrap();

        for i in 0..self.children.len() {
            let child = &self.children[i];    
            let a = child.head.area();
            let cx = x + self.area.x + a.x;
            let cy = y + self.area.y + a.y;

            if cx + a.w >= scissor[0] as i32 && cy + a.h >= scissor[1] as i32 &&
               cx <= (scissor[0] + scissor[2]) as i32 && cy <= (scissor[1] + scissor[3]) as i32 {

                child.head.draw(viewport, gl, draw_state, cx, cy);
            }
        }
    }


    fn handle_button_event(&mut self, event: &ButtonEvent) -> Option<&dyn UiHead> {

        let option = self.find_child_at(event.mx, event.my);
                
        match option {
            None => {
                println!("  error: component is not mutable");
            },
            Some(child) => {
                return child.head.handle_button_event(event);
            }
        }

        None
    }


    fn handle_scroll_event(&mut self, event: &ScrollEvent) -> Option<&dyn UiHead> {

        let option = self.find_child_at(event.mx, event.my);
                
        match option {
            None => {
                println!("  error: component is not mutable");
            },
            Some(child) => {
                return child.head.handle_scroll_event(event);
            }
        }

        None
    }


    fn add_child(&mut self, child: Rc<UiComponent>) {
        self.children.push(child);
    }
}


pub struct UiButton {
    pub area: UiArea,
    pub font: Rc<UiFont>,
    pub label: String,
}


impl UiHead for UiButton {
    
    fn area(&self) -> &UiArea {
        &self.area
    }


    fn draw(&self, viewport: Viewport, gl: &mut GlGraphics, draw_state: &DrawState, x: i32, y: i32) {

        let area = self.area();

        gl.draw(viewport, |c, gl| {

            let rect = Rectangle::new([1.0, 0.5, 0.0, 1.0]); 
            rect.draw([x as f64, y as f64, area.w as f64, area.h as f64], draw_state, c.transform, gl)
        });

        let label_width = self.font.calc_string_width(&self.label) as i32;
        let label_x = x + (area.w - label_width) / 2;
        let label_y = y + (area.h - self.font.lineheight) / 2;

        self.font.draw(viewport, gl, draw_state, label_x, label_y, &self.label, &[1.0, 1.0, 1.0, 1.0]);
    } 
}


pub struct UiIcon
{
    pub area: UiArea,
    pub font: Rc<UiFont>,
    pub label: String,
    pub tile: Rc<Tile>,
    pub userdata: usize,
}


impl UiHead for UiIcon
{

    fn area(&self) -> &UiArea {
        &self.area
    }


    fn draw(&self, viewport: Viewport, gl: &mut GlGraphics, draw_state: &DrawState, x: i32, y: i32) {
        let area = self.area();

        gl.draw(viewport, |c, gl| {

            
            let rect = Rectangle::new([0.1, 0.1, 0.1, 1.0]); 
            rect.draw([x as f64, y as f64, area.w as f64, area.h as f64], draw_state, c.transform, gl);

            let tw = self.tile.size[0] * 0.25;
            let th = self.tile.size[1] * 0.25;

            let y_base = y + area.h - 26; // space for a label below the icon image

            let image_x = x + (area.w - tw as i32) / 2;
            let image_y = y_base - th as i32;

            let image   = 
                Image::new()
                    .rect([image_x as f64, image_y as f64, tw, th])
                    .color([1.0, 1.0, 1.0, 1.0]);
            image.draw(&self.tile.tex, draw_state, c.transform, gl);
        });

        let label_width = self.font.calc_string_width(&self.label) as i32;
        let label_x = x + (area.w - label_width) / 2;
        let label_y = y + area.h - self.font.lineheight;

        self.font.draw(viewport, gl, draw_state, label_x, label_y, &self.label, &[0.4, 0.6, 0.7, 1.0]);
    } 


    fn handle_button_event(&mut self, _event: &ButtonEvent) -> Option<&dyn UiHead> {
        Some(self)
    }


    fn get_userdata(&self) -> usize {
        self.userdata
    }
}


pub struct UiScrollpane
{
    pub area: UiArea,
    child: UiComponent,
    offset_x: i32,
    offset_y: i32,
    scroll_step_x: i32,
    scroll_step_y: i32
}


impl UiHead for UiScrollpane
{

    fn area(&self) -> &UiArea {
        &self.area
    }


    fn draw(&self, viewport: Viewport, gl: &mut GlGraphics, draw_state: &DrawState, x: i32, y: i32) {
        let area = self.area();
        let xp = x + area.x;
        let yp = y + area.y;

        gl.draw(viewport, |c, gl| {

            let rect = Rectangle::new([0.3, 0.2, 0.1, 0.5]);

            rect.draw([xp as f64, yp as f64, area.w as f64, area.h as f64], draw_state, c.transform, gl);
        });

        let scissor_state = draw_state.scissor([xp as u32, yp as u32, area.w as u32, area.h as u32]);
        self.child.head.draw(viewport, gl, &scissor_state, xp + self.offset_x, yp + self.offset_y);
    }


    fn handle_scroll_event(&mut self, event: &ScrollEvent) -> Option<&dyn UiHead> {
        self.offset_x += event.dx * self.scroll_step_x;
        self.offset_y += event.dy * self.scroll_step_y;

        println!("Scrollpane, new scroll offset is {}, {}", self.offset_x, self.offset_y);

        Some(self)
    }

    fn handle_button_event(&mut self, event: &ButtonEvent) -> Option<&dyn UiHead> {

        // paging keys
        if event.args.button == piston::Button::Keyboard(piston::Key::PageDown) {
            self.offset_y -= self.area.h;
            return Some(self);
        }

        if event.args.button == piston::Button::Keyboard(piston::Key::PageUp) {
            self.offset_y += self.area.h;
            return Some(self);
        }

        self.child.head.handle_button_event(&event.translate(-self.area.x-self.offset_x, -self.area.y-self.offset_y))
    }

}
