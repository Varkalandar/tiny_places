extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate freetype;
extern crate image;
extern crate rodio;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use graphics::{Context, DrawState, draw_state::Blend, Ellipse, Image, ImageSize, Transformed, clear};
use graphics::math::Matrix2d;
use piston::{ButtonState, MouseButton};
use vecmath::{vec2_add, vec2_len, vec2_scale, vec2_sub, Vector2};

use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, 
                    UpdateArgs, UpdateEvent, 
                    ButtonArgs, ButtonEvent,
                    MouseCursorEvent, MouseScrollEvent};
use piston::window::WindowSettings;

use std::path::Path;
use std::cmp::Ordering;

mod item;
mod inventory;
mod map;
mod editor;
mod game;
mod ui;
mod sound;

#[path = "ui/player_inventory_view.rs"]
mod player_inventory_view;

use map::{Map, MAP_GROUND_LAYER, MAP_OBJECT_LAYER, MAP_CLOUD_LAYER};
use ui::{UI, UiController, TileSet, Tile, MouseMoveEvent, ScrollEvent};
use editor::MapEditor;
use game::Game;
use item::ItemFactory;
use inventory::{Inventory, Slot};
use sound::SoundPlayer;

// Game structures

pub struct GameWorld {
    map: Map,
    layer_tileset: [TileSet; 7],

    player_inventory: Inventory,

    speaker: SoundPlayer,
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

    ui: UI,

    world: GameWorld,
    controllers: GameControllers,
}


impl App {
    
    fn new(opengl: OpenGL, window_size: [u32; 2]) -> App {
        
        // let texture = Texture::from_path(Path::new("resources/map/map_soft_grass.png"), &TextureSettings::new()).unwrap();
        // let texture = Texture::from_path(Path::new("resources/map/map_dark_technoland.png"), &TextureSettings::new()).unwrap();
        let texture = Texture::from_path(Path::new("resources/map/map_puzzle_technoland.png"), &TextureSettings::new()).unwrap();

        let ground_tiles = TileSet::load("../tiny_places_client/resources/grounds", "map_objects.tica");
        let decoration_tiles = TileSet::load("../tiny_places_client/resources/objects", "map_objects.tica");
        let item_tiles = TileSet::load("../tiny_places_client/resources/items", "items.tica");
        let cloud_tiles = TileSet::load("../tiny_places_client/resources/clouds", "map_objects.tica");
        let creature_tiles = TileSet::load("../tiny_places_client/resources/creatures", "creatures.tica");
        let player_tiles = TileSet::load("../tiny_places_client/resources/players", "players.tica");
        let projectile_tiles = TileSet::load("../tiny_places_client/resources/projectiles", "projectiles.tica");

        let layer_tileset = [
            ground_tiles,
            decoration_tiles,
            cloud_tiles,
            creature_tiles,
            player_tiles,
            projectile_tiles,
            item_tiles,
            ];        


        let ui = UI::new(window_size);
        let map = Map::new("map_puzzle_technoland.png"); 
        let editor = MapEditor::new();
        let game = Game::new(&ui, &layer_tileset[6]);

        let mut inv = Inventory::new();

        let mut factory = ItemFactory::new();
        let demo_item = factory.make_item(0);
        inv.put_item(demo_item, Slot::Bag);

        let laser = factory.make_item(1);
        inv.put_item(laser, Slot::RWing);

        let engine = factory.make_item(2);
        inv.put_item(engine, Slot::Bag);


        App {        

            gl: GlGraphics::new(opengl),
            map_texture: texture,
            
            ui,
            world: GameWorld {
                map,
                layer_tileset,
                player_inventory: inv,
                speaker: SoundPlayer::new(),

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

            fn draw_layer(gl: &mut GlGraphics, c: Context, ds: DrawState, window_center: &Vector2<f64>, world: &GameWorld, layer_id: usize) {
                let player_position = &world.map.player_position();
                let mut objects = Vec::new();

                for (_key, mob) in &world.map.layers[layer_id] {
                    objects.push(mob);
                }

                objects.sort_unstable_by(|a, b| -> Ordering {
                    let ap = a.position[0] + a.position[1] * 10000.0;
                    let bp = b.position[0] + b.position[1] * 10000.0;

                    if ap > bp {
                        Ordering::Greater
                    } else if ap < bp {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                });

                for mob in objects {
                    let tileset_id = mob.visual.tileset_id;
                    let set = &world.layer_tileset[tileset_id];
                    
                    let tile = set.tiles_by_id.get(&mob.visual.current_image_id).unwrap();

                    let tf = build_transform(c.transform, &mob.position, mob.scale, tile.foot, player_position, window_center);        
    
                    // mark selected item with an ellipse
                    if world.map.has_selection && 
                       layer_id == world.map.selected_layer &&
                       mob.uid == world.map.selected_item {
                        let ellp = Ellipse::new([1.0, 0.9, 0.3, 0.3]); 
                        ellp.draw([-40 as f64, -20 as f64, 80 as f64, 40 as f64], &ds, 
                                  tf.trans(tile.foot[0], tile.foot[1]), gl);
                    }
    
                    let image = build_image(tile, &mob.visual.color);
                    image.draw(&tile.tex, &ds, tf, gl);

                    // fake shine for glowing projectiles
                    if tileset_id == 5 {

                        let glow_tile = &world.layer_tileset[2].tiles_by_id[&21]; // cloud set

                        let tf = build_transform(c.transform, &mob.position, 0.9, glow_tile.foot, player_position, window_center).trans(-170.0, -50.0);
                        let image = build_image(glow_tile, &[0.5, 0.375, 0.2, 1.0]);
                        image.draw(&glow_tile.tex, &ds.blend(Blend::Add), tf, gl);
                    }
                }    
            }

            // Clear the screen.
            clear([0.0, 0.0, 0.0, 1.0], gl);

            let player_position = &self.world.map.player_position();
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

            // draw ground decorations (flat)
            draw_layer(gl, c, ds, &window_center, &self.world, MAP_GROUND_LAYER);

            // draw shadows (flat)
            // TODO
            
            // draw decorations (upright things)
            draw_layer(gl, c, ds, &window_center, &self.world, MAP_OBJECT_LAYER);

            // draw lights
            // TODO

            // draw clouds
            draw_layer(gl, c, ds, &window_center, &self.world, MAP_CLOUD_LAYER);
            
        });

        {
            let world = &mut self.world;
            let ui = &mut self.ui;
            self.controllers.current().draw(viewport, &mut self.gl, &ds, ui, world);    
            self.controllers.current().draw_overlay(viewport, &mut self.gl, &ds, ui, world);    
        }
    }


    fn update(&mut self, args: &UpdateArgs) {
        let world = &mut self.world;
        self.controllers.current().update(world, args.dt);
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

        // editor/game swtich must be handled here, the button pres is not handed down ATM

        if event.args.state == ButtonState::Release {
            if event.args.button == piston::Button::Keyboard(piston::Key::E) {    
                self.controllers.edit = true;
                println!("Switching to editor mode.");
            }
            if event.args.button == piston::Button::Keyboard(piston::Key::G) {                        
                self.controllers.edit = false;
                println!("Switching to game mode.");
            }        
        }

        // now the ordinary events

        let window_center: Vector2<f64> = self.ui.window_center(); 
        let controller = &mut self.controllers.current();
        let world = &mut self.world;
        let ui = &mut self.ui;

        let consumed = controller.handle_button_event(ui, &event, world);

        if event.args.state == ButtonState::Release && !consumed {
            if event.args.button == piston::Button::Mouse(MouseButton::Left) {
                self.move_player(window_center);            
            }
        }
    }    
    
    
    fn mouse_cursor(&mut self, args: &[f64; 2]) {
        
        self.ui.mouse_state.position = *args;

        let event = MouseMoveEvent {
            mx: self.ui.mouse_state.position[0] as i32,
            my: self.ui.mouse_state.position[1] as i32,
        };

        let controller = &mut self.controllers.current();
        let world = &mut self.world;
        let ui = &mut self.ui;

        controller.handle_mouse_move_event(ui, &event, world);
    }
    

    fn mouse_scroll(&mut self, args: &[f64; 2]) {
        println!("Mouse scroll event {:?}", args);

        let event = ScrollEvent {
            dx: args[0],
            dy: args[1],
            mx: self.ui.mouse_state.position[0] as i32,
            my: self.ui.mouse_state.position[1] as i32,
        };

        let controller = &mut &mut self.controllers.current();
        let world = &mut self.world;
        let ui = &mut self.ui;

        controller.handle_scroll_event(ui, &event, world);
    }


    fn move_player(&mut self, window_center: Vector2<f64>) {
        
        let screen_direction = vec2_sub(self.ui.mouse_state.position, window_center);
        
        // world coordinates have y components double as large
        // as screen coordinates
        let direction = [screen_direction[0], screen_direction[1] * 2.0];
        
        let distance = vec2_len(direction);

        let player = self.world.map.layers[MAP_OBJECT_LAYER].get_mut(&self.world.map.player_id).unwrap();

        let time = distance / player.attributes.speed; // pixel per second

        player.move_time_left = time;
        player.velocity = vec2_scale(direction, 1.0/time);

        let dest = vec2_add(player.position, direction);

        let d = player.visual.orient(direction[0], direction[1]);
        player.visual.current_image_id = player.visual.base_image_id + d;

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


pub fn build_transform(transform: Matrix2d<f64>, position: &Vector2<f64>, scale: f64, foot: Vector2<f64>, player_position: &Vector2<f64>, window_center: &Vector2<f64>) -> [[f64; 3]; 2] {
    let rel_pos_x = position[0] - player_position[0];        
    let rel_pos_y = position[1] - player_position[1];  

    transform
        .trans(window_center[0], window_center[1])
        .trans(rel_pos_x, rel_pos_y * 0.5)
        .scale(scale, scale)
        .trans(-foot[0], -foot[1])
}


pub fn build_image(tile: &Tile, color: &[f32; 4]) -> Image {
    Image::new()
        .rect([0.0, 0.0, tile.size[0], tile.size[1]])
        .color(*color)        
}


fn main() {
    
    let window_size = [1200, 770];

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Rusty Tiny Places", window_size)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .vsync(true)
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
