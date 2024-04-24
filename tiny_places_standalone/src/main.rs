extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use vecmath::{vec2_add, vec2_len, vec2_scale, vec2_sub, Vector2};

use piston::Position;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, 
                    UpdateArgs, UpdateEvent, 
                    ButtonArgs, ButtonEvent,
                    MouseCursorEvent};
use piston::window::WindowSettings;

use graphics::{Image, clear};
use graphics::draw_state::DrawState;
use graphics::rectangle::square;
use std::path::Path;

mod item;
mod map;
mod mob;

use item::Item;
use map::Map;
use mob::Mob;


struct MouseState {
    position: Vector2<f64>,    
}


pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    mouse_state: MouseState,
    rotation: f64,  // Rotation for the square.
    
    map_texture: Texture,
    player_texture: Texture,
    
    map: Map,
    player: Mob,
}


impl App {
    
    fn new(opengl: OpenGL) -> App {
        
        let texture = Texture::from_path(Path::new("resources/map/map_soft_grass.png"), &TextureSettings::new()).unwrap();
        let player_texture = Texture::from_path(Path::new("../tiny_places_client/resources/creatures/9-vortex.png"), &TextureSettings::new()).unwrap();

        let player = Mob::new(1000.0, 1000.0);
        
        App {        
            gl: GlGraphics::new(opengl),
            mouse_state: MouseState{position: [0.0, 0.0],},
            rotation: 0.0,
            map_texture: texture,
            player_texture: player_texture,

            player: player,
            map: Map::new(),
        }
    }

    
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        // const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        // let square = rectangle::square(0.0, 0.0, 50.0);
        // let rotation = self.rotation;
        // let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear([0.0, 0.0, 0.0, 1.0], gl);

            let w05 = args.window_size[0] * 0.5;
            let h05 = args.window_size[1] * 0.5;

            let offset_x = w05 * 0.5 - self.player.position[0];
            let offset_y = h05 - self.player.position[1] * 0.5;

            // The map is display 2 times as big as source image to conserve memory
            // for the map background a high detail level is not needed, that is
            // provided by decorations will are drawn in full resolution
            let map_tf = c.transform.trans(offset_x, offset_y).scale(2.0, 2.0);
//            image(&self.map_texture, map_tf, gl);
            let m_image   = 
                Image::new()
                    .rect([0.0, 0.0, self.map_texture.get_width() as f64, self.map_texture.get_height() as f64])
                    .color([0.8, 0.8, 0.8, 1.0]);
            m_image.draw(&self.map_texture, &DrawState::new_alpha(), map_tf, gl);


            let p_tf = c.transform.trans(w05, h05).scale(0.5, 0.5);
//            image(&self.player_texture, p_tf, gl);
            let p_image   = 
                Image::new()
                    .rect([0.0, 0.0, self.player_texture.get_width() as f64, self.player_texture.get_height() as f64])
                    .color([1.0, 0.8, 0.6, 1.0]);
            p_image.draw(&self.player_texture, &DrawState::new_alpha(), p_tf, gl);
/*
            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
*/            
            
        });
    }


    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
        
        self.player.move_by_time(args.dt);
    }


    fn button(&mut self, args: &ButtonArgs) {
        println!("Button event {:?}", args);
        
        let window_center: Vector2<f64> = [500.0, 375.0]; 
        
        let screen_direction = vec2_sub(self.mouse_state.position, window_center);
        
        // world coordinates have y components double as large
        // as screen coordinates
        let direction = [screen_direction[0], screen_direction[1] * 2.0];
        
        let distance = vec2_len(direction);
        let time = distance / self.player.base_speed; // pixel per second
        
        self.player.move_over_time = time;
        self.player.speed = vec2_scale(direction, 1.0/time);

        let dest = vec2_add(self.player.position, direction);

        println!("  moving {} pixels over {} seconds, destination is {:?}", distance, time, dest);
    }

    
    fn mouse_cursor(&mut self, args: &[f64; 2]) {
        // println!("Mouse cursor event {:?}", args);
        
        self.mouse_state.position = *args;
    }
}



fn main() {
    
    let mut item = Item::new();
    item.name = "First Item".to_string();
    item.print_debug();
    
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Rusty Tiny Places", [1000, 750])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new(opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(args) = e.button_args() {
            app.button(&args);
        }

        if let Some(args) = e.mouse_cursor_args() {
            app.mouse_cursor(&args);
        }
    }
    
}
