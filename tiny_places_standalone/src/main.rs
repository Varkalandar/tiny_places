extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate freetype;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::{ButtonState, MouseButton};
use vecmath::{vec2_add, vec2_len, vec2_scale, vec2_sub, Vector2};

use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, 
                    UpdateArgs, UpdateEvent, 
                    ButtonArgs, ButtonEvent,
                    MouseCursorEvent};
use piston::window::WindowSettings;

use graphics::draw_state::DrawState;
use std::path::Path;
use std::rc::Rc;

mod item;
mod map;
mod mob;
mod ui;

use item::Item;
use map::{Map, MapObject, Tile};
use mob::Mob;
use ui::UI;


struct MouseState {
    position: Vector2<f64>,
    drag_start: Vector2<f64>,    
}

impl MouseState {
    fn record_drag_start(&mut self) -> Vector2<f64> {
        self.drag_start = self.position;
        self.drag_start
    }
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    mouse_state: MouseState,
    
    map_texture: Texture,
    player_texture: Texture,
    
    map: Map,
    player: Mob,

    ui: UI,
}


impl App {
    
    fn new(opengl: OpenGL) -> App {

        
        let texture = Texture::from_path(Path::new("resources/map/map_soft_grass.png"), &TextureSettings::new()).unwrap();
        let player_texture = Texture::from_path(Path::new("../tiny_places_client/resources/creatures/9-vortex.png"), &TextureSettings::new()).unwrap();

        let player = Mob::new(1000.0, 1000.0);
        
        App {        

            gl: GlGraphics::new(opengl),
            mouse_state: MouseState{position: [0.0, 0.0], drag_start: [0.0, 0.0]},
            map_texture: texture,
            player_texture: player_texture,

            player: player,
            map: Map::new(),
            
            ui: UI::new(),
        }
    }

    
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        
        let viewport = &args.viewport();

        self.gl.draw(*viewport, |c, gl| {

            fn build_transform(c: Context, thing: &MapObject, player_position: &Vector2<f64>, window_center: &Vector2<f64>) -> [[f64; 3]; 2] {
                let rel_pos_x = thing.position[0] - player_position[0];        
                let rel_pos_y = thing.position[1] - player_position[1];        
                c.transform
                    .trans(window_center[0], window_center[1])
                    .trans(rel_pos_x, rel_pos_y * 0.5)
                    .scale(0.5, 0.5)
            }

            fn build_image(tile: &Tile) -> Image {
                Image::new()
                    .rect([0.0, 0.0, tile.size[0], tile.size[1]])
                    .color([1.0, 0.8, 0.6, 1.0])        
            }

            // Clear the screen.
            clear([0.0, 0.0, 0.0, 1.0], gl);

            let player_position = &self.player.position;
            let window_center: Vector2<f64> = [args.window_size[0] * 0.5, args.window_size[1] * 0.5];

            let offset_x = window_center[0] * 0.5 - player_position[0];
            let offset_y = window_center[1] - player_position[1] * 0.5;

            // The map is displayed 2 times as big as source image to conserve memory
            // for the map background a high detail level is not needed, that is
            // provided by decorations will are drawn in full resolution
            let map_tf = c.transform.trans(offset_x, offset_y).scale(2.0, 2.0);
            let m_image   = 
                Image::new()
                    .rect([0.0, 0.0, self.map_texture.get_width() as f64, self.map_texture.get_height() as f64])
                    .color([0.8, 0.8, 0.8, 1.0]);
            m_image.draw(&self.map_texture, &DrawState::new_alpha(), map_tf, gl);

            let p_tf = c.transform.trans(window_center[0], window_center[1]).scale(0.5, 0.5);
            let p_image   = 
                Image::new()
                    .rect([0.0, 0.0, self.player_texture.get_width() as f64, self.player_texture.get_height() as f64])
                    .color([1.0, 0.8, 0.6, 1.0]);
            p_image.draw(&self.player_texture, &DrawState::new_alpha(), p_tf, gl);

            // draw ground decorations (flat)
            // TODO

            // draw shadows (flat)
            // TODO
            
            // draw decorations (upright things)
            for deco in &self.map.decorations {
                let tile = self.map.decoration_tiles.tiles_by_id.get(&deco.id).unwrap();
                let image   = build_image(tile);
                let tf = build_transform(c, deco, player_position, &window_center);        
                image.draw(&tile.tex, &DrawState::new_alpha(), tf, gl);
            }

            // draw lights
            // TODO

            // draw clouds
            // TODO
            
        });

        self.ui.draw(viewport, &mut self.gl);
    }


    fn update(&mut self, args: &UpdateArgs) {
        self.player.move_by_time(args.dt);
    }


    fn button(&mut self, args: &ButtonArgs) {
        println!("Button event {:?}", args);
        
        if args.state == ButtonState::Press {
            self.mouse_state.record_drag_start();
        }
        
        if args.state == ButtonState::Release {
            if args.button == piston::Button::Mouse(MouseButton::Left) {
                self.move_player();            
            }
            
            if args.button == piston::Button::Mouse(MouseButton::Right) {
                let deco = self.make_deco();
                self.map.decorations.push(deco);
            }
            
            if args.button == piston::Button::Keyboard(piston::Key::Space) {
                self.show_test_dialog();       
            }        
        }
    }
    
    
    fn mouse_cursor(&mut self, args: &[f64; 2]) {
        // println!("Mouse cursor event {:?}", args);
        
        self.mouse_state.position = *args;
    }
    
    
    fn move_player(&mut self) {
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

    fn make_deco(&mut self) -> MapObject {
        
        let rel_mouse = vec2_sub(self.mouse_state.position, [500.0, 375.0]);

        // transform to world coordinates
        let mut world_pos = [rel_mouse[0], rel_mouse[1] * 2.0];
        
        // it is relatrive to player position
        world_pos = vec2_add(world_pos, self.player.position);

        println!("  creating deco at {:?}, player at {:?}", world_pos, self.player.position);

        let id = 1;
        MapObject::new(id, world_pos, 1.0)
    }

    
    /*
    fn draw_map(self, c: Context, gl: &mut GlGraphics, w05: f64, h05: f64) {
        
        // let tf = c.transform.trans(w05, h05).scale(0.5, 0.5);
        let tf = c.transform.trans(0.0, 0.0);

        for deco in self.map.decorations {
            let tile = self.map.decoration_tiles.tiles_by_id.get(&deco.id).unwrap();
            let image   = 
                Image::new()
                    .rect([0.0, 0.0, tile.size[0], tile.size[1]])
                    .color([1.0, 0.8, 0.6, 1.0]);
            image.draw(&tile.tex, &DrawState::new_alpha(), tf, gl);
        }
    }
    */
    
    fn show_test_dialog(&mut self) {
        let mut cont = self.ui.make_container(100, 100, 600, 400);
        let button = self.ui.make_button(100, 100, 300, 200);
        
        cont.children.push(Rc::new(button));
        
        self.ui.root = Some(cont);
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
