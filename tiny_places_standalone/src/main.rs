extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate freetype;
extern crate image;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use graphics::{Context, DrawState, Ellipse, Image, ImageSize, Transformed, clear};
use piston::{ButtonState, MouseButton};
use vecmath::{vec2_add, vec2_len, vec2_scale, vec2_sub, Vector2};

use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, 
                    UpdateArgs, UpdateEvent, 
                    ButtonArgs, ButtonEvent,
                    MouseCursorEvent, MouseScrollEvent};
use piston::window::WindowSettings;

use std::path::Path;

mod item;
mod map;
mod mob;
mod editor;
mod game;
mod ui;

use map::{Map, MapObject, MAP_GROUND_LAYER, MAP_DECO_LAYER, MAP_CLOUD_LAYER};
use ui::{UI, UiController, TileSet, Tile, ScrollEvent};
use editor::MapEditor;
use game::Game;


pub struct GameWorld {
    map: Map,
    layer_tileset: [TileSet; 7],
}

pub struct GameControllers {
    editor: MapEditor,
    game: Game,
    edit: bool,    
}

impl GameControllers {
    fn current(&mut self) -> &mut dyn UiController<Appdata = GameWorld> {
        if self.edit {
            &mut self.editor
        }
        else {
            &mut self.game
        }
    }
}


pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    
    map_texture: Texture,
    player_texture: Texture,

    ui: UI,

    world: GameWorld,
    controllers: GameControllers,
}


impl App {
    
    fn new(opengl: OpenGL, window_size: [u32; 2]) -> App {
        
        let texture = Texture::from_path(Path::new("resources/map/map_soft_grass.png"), &TextureSettings::new()).unwrap();
        let player_texture = Texture::from_path(Path::new("../tiny_places_client/resources/creatures/9-vortex.png"), &TextureSettings::new()).unwrap();

        let ground_tiles = TileSet::load("../tiny_places_client/resources/grounds", "map_objects.tica");
        let decoration_tiles = TileSet::load("../tiny_places_client/resources/objects", "map_objects.tica");
        let cloud_tiles = TileSet::load("../tiny_places_client/resources/clouds", "map_objects.tica");

        let mut layer_tileset = [
            ground_tiles,
            decoration_tiles,
            cloud_tiles,
            TileSet::new(),
            TileSet::new(),
            TileSet::new(),
            TileSet::new(),
            ];        


        let mut ui = UI::new(window_size);
        let map = Map::new(); 
        let editor = MapEditor::new();
        let game = Game::new();

        App {        

            gl: GlGraphics::new(opengl),
            map_texture: texture,
            player_texture: player_texture,
            
            ui,
            world: GameWorld {
                map,
                layer_tileset,
            },
            controllers: GameControllers {
                editor,
                game,
                edit: true,
            }
        }
    }

    
    fn render(&mut self, args: &RenderArgs) {
        let viewport = args.viewport();
        let ds = DrawState::new_alpha();

        self.gl.draw(viewport, |c, gl| {

            fn build_transform(c: Context, thing: &MapObject, tile: &Tile, player_position: &Vector2<f64>, window_center: &Vector2<f64>) -> [[f64; 3]; 2] {
                let rel_pos_x = thing.position[0] - player_position[0];        
                let rel_pos_y = thing.position[1] - player_position[1];  
                let scale = thing.scale;

                c.transform
                    .trans(window_center[0], window_center[1])
                    .trans(rel_pos_x, rel_pos_y * 0.5)
                    .scale(scale, scale)
                    .trans(-tile.foot[0], - tile.foot[1])
            }

            fn build_image(tile: &Tile, color: &[f32; 4]) -> Image {
                Image::new()
                    .rect([0.0, 0.0, tile.size[0], tile.size[1]])
                    .color(*color)        
            }

            fn draw_layer(gl: &mut GlGraphics, c: Context, ds: DrawState, window_center: &Vector2<f64>, world: &GameWorld, layer_id: usize) {
                let player_position = &world.map.player.position;
                let set = &world.layer_tileset[layer_id];

                for idx in 0..world.map.layers[layer_id].len() {
                    let deco = &world.map.layers[layer_id][idx];
                    let tile = set.tiles_by_id.get(&deco.tile_id).unwrap();
                    let tf = build_transform(c, deco, tile, player_position, window_center);        
    
                    // mark selected item with an ellipse
                    if world.map.has_selection && 
                       layer_id == world.map.selected_layer &&
                       idx == world.map.selected_item {
                        let ellp = Ellipse::new([1.0, 0.9, 0.3, 0.3]); 
                        ellp.draw([-40 as f64, -20 as f64, 80 as f64, 40 as f64], &ds, 
                                  tf.trans(tile.foot[0], tile.foot[1]), gl);
                    }
    
                    let image = build_image(tile, &deco.color);
                    image.draw(&tile.tex, &ds, tf, gl);
                }    
            }



            // Clear the screen.
            clear([0.0, 0.0, 0.0, 1.0], gl);

            let player_position = &self.world.map.player.position;
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
            m_image.draw(&self.map_texture, &ds, map_tf, gl);

            let p_tf = c.transform.trans(window_center[0], window_center[1]).scale(0.5, 0.5);
            let p_image   = 
                Image::new()
                    .rect([0.0, 0.0, self.player_texture.get_width() as f64, self.player_texture.get_height() as f64])
                    .color([1.0, 0.8, 0.6, 1.0]);
            p_image.draw(&self.player_texture, &ds, p_tf, gl);

            // draw ground decorations (flat)
            draw_layer(gl, c, ds, &window_center, &self.world, MAP_GROUND_LAYER);

            // draw shadows (flat)
            // TODO
            
            // draw decorations (upright things)
            draw_layer(gl, c, ds, &window_center, &self.world, MAP_DECO_LAYER);

            // draw lights
            // TODO

            // draw clouds
            draw_layer(gl, c, ds, &window_center, &self.world, MAP_CLOUD_LAYER);
            
        });

        self.ui.draw(viewport, &mut self.gl);

        {
            let editor = &mut self.controllers.editor;
            let world = &mut self.world;
            let ui = &mut self.ui;
            self.controllers.editor.draw_overlay(viewport, &mut self.gl, &ds, ui, world);    
        }
    }


    fn update(&mut self, args: &UpdateArgs) {
        let map = &mut self.world.map;
        map.update(args.dt);
    }


    fn button(&mut self, args: &ButtonArgs) {
        println!("Button event {:?}", args);
        
        if args.state == ButtonState::Press {
            self.ui.mouse_state.record_drag_start();
        }

        let event = ui::ButtonEvent {
            args,
            mx: self.ui.mouse_state.position[0] as i32,
            my: self.ui.mouse_state.position[1] as i32,
        };
        let controller = &mut self.controllers.current();
        let world = &mut self.world;
        let ui = &mut self.ui;

        let consumed = controller.handle_button_event(ui, &event, world);

        if event.args.state == ButtonState::Release && !consumed {
            if event.args.button == piston::Button::Mouse(MouseButton::Left) {
                self.move_player();            
            }
        }
    }    
    
    
    fn mouse_cursor(&mut self, args: &[f64; 2]) {
        // println!("Mouse cursor event {:?}", args);
        
        self.ui.mouse_state.position = *args;
    }
    

    fn mouse_scroll(&mut self, args: &[f64; 2]) {
        println!("Mouse scroll event {:?}", args);

        let event = ScrollEvent {
            dx: args[0],
            dy: args[1],
            mx: self.ui.mouse_state.position[0] as i32,
            my: self.ui.mouse_state.position[1] as i32,
        };

        let editor = &mut self.controllers.editor;
        let world = &mut self.world;
        let ui = &mut self.ui;

        editor.handle_scroll_event(ui, &event, world);
    }


    fn move_player(&mut self) {
        let window_center: Vector2<f64> = [500.0, 375.0]; 
        
        let screen_direction = vec2_sub(self.ui.mouse_state.position, window_center);
        
        // world coordinates have y components double as large
        // as screen coordinates
        let direction = [screen_direction[0], screen_direction[1] * 2.0];
        
        let distance = vec2_len(direction);
        let time = distance / self.world.map.player.base_speed; // pixel per second

        let map = &mut self.world.map;
        let player = &mut map.player;
        player.move_over_time = time;
        player.speed = vec2_scale(direction, 1.0/time);

        let dest = vec2_add(player.position, direction);

        println!("  moving {} pixels over {} seconds, destination is {:?}", distance, time, dest);
        
    }
}


pub fn screen_to_world_pos(ui: &UI, player_pos: &Vector2<f64>, screen_pos: &Vector2<f64>) -> Vector2<f64>
{
    let rel_mouse_x = screen_pos[0] - (ui.window_size[0]/2) as f64;
    let rel_mouse_y = (screen_pos[1] - (ui.window_size[1]/2) as f64) * 2.0;

    // transform to world coordinates
    // it is relatrive to player position
    let world_pos = [rel_mouse_x + player_pos[0], rel_mouse_y + player_pos[1]];

    world_pos
}


fn main() {
    
    let window_size = [1000, 750];

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Rusty Tiny Places", window_size)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new(opengl, window_size);

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

        if let Some(args) = e.mouse_scroll_args() {
            app.mouse_scroll(&args);
        }
    }
    
}
