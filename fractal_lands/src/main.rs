extern crate sdl2;
extern crate freetype;
extern crate image;
extern crate rodio;
extern crate rand;

use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::render::WindowCanvas;
use sdl2::render::TextureAccess;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::render::CanvasBuilder;

use image::GenericImageView;
use image::Pixel;

use vecmath::{vec2_add, vec2_len, vec2_scale, vec2_sub, Vector2};
use rand::SeedableRng;


use std::time::SystemTime;
use std::fs::read_to_string;
use std::path::Path;
use std::cmp::Ordering;

mod item;
mod creature;
mod inventory;
mod projectile;
mod map;
mod editor;
mod game;
mod ui;
mod sound;
mod particle_driver;
mod animation;
mod mob_group;
mod player_inventory_view;

use map::{Map, MAP_GROUND_LAYER, MAP_OBJECT_LAYER, MAP_CLOUD_LAYER};
use ui::{UI, UiController, TileSet, Tile, MouseMoveEvent, ScrollEvent};
use editor::MapEditor;
use game::Game;
use item::ItemFactory;
use inventory::{Inventory, Slot};
use sound::SoundPlayer;

const MAP_RESOURCE_PATH: &str = "resources/map/";
const CREATURE_TILESET: usize = 3;
const PROJECTILE_TILESET: usize = 5;
const ANIMATION_TILESET: usize = 7;

// Game structures

pub struct GameWorld {
    map: Map,
    layer_tileset: [TileSet; 8],

    player_inventory: Inventory,

    speaker: SoundPlayer,

    rng: rand::rngs::StdRng,

    map_texture: Texture,
    map_backdrop: Texture,
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
    creator: TextureCreator<WindowContext>,
    canvas: WindowCanvas,

    ui: UI,

    world: GameWorld,
    controllers: GameControllers,

    update_time: SystemTime,
}


impl App {
    
    fn new(window: Window, window_size: [u32; 2]) -> App {

        let canvas_builder = CanvasBuilder::new(window);
        let canvas = 
            canvas_builder
                .accelerated()
                .present_vsync()
                .build()
                .unwrap();

        let creator = canvas.texture_creator();

        let map_image_file = "map_wasteland.png";
        let map_backdrop_file = "backdrop_red_blue.png";

        let map_texture = load_texture(&creator, &(MAP_RESOURCE_PATH.to_string() + map_image_file));
        let map_backdrop = load_texture(&creator, &(MAP_RESOURCE_PATH.to_string() + map_backdrop_file));
/*
        unsafe {
            let raw = map_backdrop.raw();
            
            // sdl2::sys::SDL_SetTextureBlendMode(raw, sdl2::sys::SDL_BlendMode::SDL_BLENDMODE_BLEND);

            let m = sdl2::sys::SDL_ScaleMode::SDL_ScaleModeLinear;
            sdl2::sys::SDL_SetTextureScaleMode(raw, m);
        }
*/

        let ground_tiles = TileSet::load(&creator, "../tiny_places_client/resources/grounds", "map_objects.tica");
        let decoration_tiles = TileSet::load(&creator, "../tiny_places_client/resources/objects", "map_objects.tica");
        let item_tiles = TileSet::load(&creator, "../tiny_places_client/resources/items", "items.tica");
        let cloud_tiles = TileSet::load(&creator, "../tiny_places_client/resources/clouds", "map_objects.tica");
        let creature_tiles = TileSet::load(&creator, "../tiny_places_client/resources/creatures", "creatures.tica");
        let player_tiles = TileSet::load(&creator, "../tiny_places_client/resources/players", "players.tica");
        let projectile_tiles = TileSet::load(&creator, "../tiny_places_client/resources/projectiles", "projectiles.tica");
        let animation_tiles = TileSet::load(&creator, "../tiny_places_client/resources/animations", "animations.tica");

        let layer_tileset = [
            ground_tiles,
            decoration_tiles,
            cloud_tiles,
            creature_tiles,
            player_tiles,
            projectile_tiles,
            item_tiles,
            animation_tiles,
            ];        

        let rng = rand::rngs::StdRng::seed_from_u64(12345678901);
        let mut map = Map::new("Demo Map", map_image_file, map_backdrop_file);
        map.load("start.map");

        let ui = UI::new(&creator, window_size);
        
        let editor = MapEditor::new();

        let inventory_bg = load_texture(&creator, "resources/ui/inventory_bg.png");
        let game = Game::new(inventory_bg, &ui, &layer_tileset[6]);

        let mut inv = Inventory::new();

        let mut factory = ItemFactory::new();
        let demo_item = factory.create(0);
        inv.put_item(demo_item, Slot::Bag);

        let laser = factory.create(1);
        inv.put_item(laser, Slot::RWing);

        let engine = factory.create(2);
        inv.put_item(engine, Slot::Bag);

        for plugin_no in 3..10 {
            let plugin = factory.create(plugin_no);
            inv.put_item(plugin, Slot::Bag);
        }

        App {        
            creator,
            canvas,

            ui,

            world: GameWorld {
                map,
                layer_tileset,
                player_inventory: inv,
                speaker: SoundPlayer::new(),

                rng,

                map_texture,
                map_backdrop,
            },

            controllers: GameControllers {
                editor,
                game,
                edit: true,
            },

            update_time: SystemTime::now(),
        }
    }


    fn update(&mut self) {
        let world = &mut self.world;

        let now = SystemTime::now();
        let difference = now.duration_since(self.update_time);

        if difference.is_ok() {
            self.update_time = now;
            self.controllers.current().update(world, difference.unwrap().as_secs_f64());
        }
    }

    fn render(&mut self) {

        let world = &self.world;
        let canvas = &mut self.canvas;

        let width = self.ui.window_size[0];
        let height = self.ui.window_size[1];
        let window_center = [width / 2, height / 2];

        let player_position = &world.map.player_position();
        let player_x = player_position[0] as i32;
        let player_y = player_position[1] as i32;

        let offset_x = width as i32 / 2 - player_x;
        let offset_y = height as i32 / 2 - player_y / 2;

        // background image, parallax scrolling at 0.5 times map scroll amount
        let back_off_x = - player_x / 2;
        let back_off_y = - player_y / 4;

        canvas.clear();
        canvas.set_draw_color(Color::RGBA(200, 200, 200, 255));
        
        let s = Rect::new(back_off_x, back_off_y, 600, 450);
        let d = Rect::new(0, 0, width, height);
        canvas.copy(&world.map_backdrop, Some(s), Some(d)).unwrap();        

        canvas.present();
    }


/*
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

                    // println!("Accessing mob {} with tile {} from tileset {}", mob.uid, mob.visual.current_image_id, tileset_id);

                    let set = &world.layer_tileset[tileset_id];                    
                    let tile = set.tiles_by_id.get(&mob.visual.current_image_id).unwrap();

                    let tf = build_transform(&c.transform, &mob.position, mob.visual.scale, tile.foot, player_position, window_center);        
    
                    // mark selected item with an ellipse
                    if world.map.has_selection && 
                       layer_id == world.map.selected_layer &&
                       mob.uid == world.map.selected_item {
                        
                        let size = tile.size[0] * 0.75;
                        let ellp = Ellipse::new([1.0, 0.95, 0.9, 0.1]);

                        ellp.draw([-size, -size * 0.5, size * 2.0, size], &ds, 
                                  tf.trans(tile.foot[0], tile.foot[1]), gl);
                    }
    
                    let image = build_image(tile, mob.visual.color);
                    image.draw(&tile.tex, &ds.blend(mob.visual.blend), tf, gl);

                    // fake shine for glowing projectiles
                    if tileset_id == 5 {

                        let glow_tile = &world.layer_tileset[2].tiles_by_id[&21]; // cloud set

                        let tf = build_transform(&c.transform, &mob.position, 0.9, glow_tile.foot, player_position, window_center).trans(-170.0, -50.0);
                        let image = build_image(glow_tile, mob.visual.glow);
                        image.draw(&glow_tile.tex, &ds.blend(sdl2::render::BlendMode::Add), tf, gl);
                    }

                    // particle effects
                    mob.visual.particles.for_each_particle(|particles, last_particle_mark| {
                        
                        for i in 0..last_particle_mark {
                            let p = &particles[i];

                            if p.active {
                                // println!("p.tex={} pos {}, {}", p.tex_id, p.xpos, p.ypos);

                                let set = mob.visual.particles.spawn_tile_set;
                                let tile = &world.layer_tileset[set].tiles_by_id.get(&p.tex_id).unwrap();
                                let tf = build_transform(&c.transform, &mob.position, 1.0, tile.foot, player_position, window_center);
        
                                // world coordinates to screen coordinates
                                let xp = p.xpos;
                                let yp = (p.ypos - p.zpos) * 0.5;
                                // let glow = (1.0 - p.age / p.lifetime) as f32;
                                let fade = quadratic_fade(p.age / p.lifetime);
                                
                                let image = build_image(tile, [p.color[0]*fade, p.color[1]*fade, p.color[2]*fade, 1.0]);
                                image.draw(&tile.tex, &ds.blend(sdl2::render::BlendMode::Add), tf.trans(xp, yp), gl);
                            }
                        }
                    });
                }    
            }

            let world = &self.world;
            let player_position = &world.map.player_position();
            let window_center: Vector2<f64> = [args.window_size[0] * 0.5, args.window_size[1] * 0.5];

            let offset_x = window_center[0] * 0.5 - player_position[0];
            let offset_y = window_center[1] - player_position[1] * 0.5;

            // background image, parallax scrolling at 0.5 times map scroll amount
            let back_tf = c.transform.trans(- player_position[0]*0.5, - player_position[1] * 0.25).scale(2.0, 2.0);
            let back_image = 
                Image::new()
                    .rect([0.0, 0.0, world.map_backdrop.get_width() as f64, world.map_backdrop.get_height() as f64])
                    .color([0.8, 0.8, 0.8, 1.0]);
            back_image.draw(&world.map_backdrop, &ds, back_tf, gl);

            // The map is displayed 2 times as big as source image to conserve memory
            // for the map background a high detail level is not needed, that is
            // provided by decorations will are drawn in full resolution
            let map_tf = c.transform.trans(offset_x, offset_y).scale(2.0, 2.0);
            
            let map_image   = 
                Image::new()
                    .rect([0.0, 0.0, world.map_texture.get_width() as f64, world.map_texture.get_height() as f64])
                    .color([0.8, 0.8, 0.8, 1.0]);                    
            map_image.draw(&world.map_texture, &ds, map_tf, gl);

            // draw ground decorations (flat)
            draw_layer(gl, c, ds, &window_center, world, MAP_GROUND_LAYER);

            // draw decorations (upright things)
            draw_layer(gl, c, ds, &window_center, world, MAP_OBJECT_LAYER);

            // draw clouds
            draw_layer(gl, c, ds, &window_center, world, MAP_CLOUD_LAYER);
        });

        {
            let world = &mut self.world;
            let ui = &mut self.ui;
            self.controllers.current().draw(viewport, &mut self.gl, &ds, ui, world);    
            self.controllers.current().draw_overlay(viewport, &mut self.gl, &ds, ui, world);    
        }
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
            if event.args.button == Button::Keyboard(Keycode::E) {    
                self.controllers.edit = true;
                println!("Switching to editor mode.");
            }
            if event.args.button == Button::Keyboard(Keycode::G) {                        
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
            if event.args.button == Button::Mouse(MouseButton::Left) {
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
    */


    fn move_player(&mut self, window_center: Vector2<f64>) {
        
        let screen_direction = vec2_sub(self.ui.mouse_state.position, window_center);
        
        // world coordinates have y components double as large
        // as screen coordinates
        let direction = [screen_direction[0], screen_direction[1] * 2.0];
        
        let distance = vec2_len(direction);

        let player = self.world.map.layers[MAP_OBJECT_LAYER].get_mut(&self.world.map.player_id).unwrap();
        let attributes = player.creature.as_ref().unwrap();
        let time = distance / attributes.base_speed; // pixel per second

        player.move_time_left = time;
        player.velocity = vec2_scale(direction, 1.0/time);

        let dest = vec2_add(player.position, direction);

        let d = player.visual.orient(direction);
        player.visual.current_image_id = player.visual.base_image_id + d;

        println!("  moving {} pixels over {} seconds, destination is {:?}", distance, time, dest);        
    }
}


pub fn read_lines(pathname: &str) -> Vec<String> {
    let path = Path::new(pathname);    
    let rs = read_to_string(path).unwrap();
    let mut lines = Vec::new();
    
    for line in rs.lines() {
        lines.push(line.to_string());
    }

    lines
}


pub fn parse_rgba(color_str: &str) -> [f32; 4] {
    let mut color_iter = color_str.split(" ");

    let mut color: [f32; 4] = [0.0; 4];
    for i in 0..4 {
        color[i] = color_iter.next().unwrap().parse::<f32>().unwrap();
    }

    color
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

/*
pub fn build_transform(transform: &Matrix2d<f64>, position: &Vector2<f64>, scale: f64, foot: Vector2<f64>, player_position: &Vector2<f64>, window_center: &Vector2<f64>) -> [[f64; 3]; 2] {
    let rel_pos_x = position[0] - player_position[0];        
    let rel_pos_y = position[1] - player_position[1];  

    transform
        .trans(window_center[0], window_center[1])
        .trans(rel_pos_x, rel_pos_y * 0.5)
        .scale(scale, scale)
        .trans(-foot[0], -foot[1])
}


pub fn build_image(tile: &Tile, color: [f32; 4]) -> Image {
    Image::new()
        .rect([0.0, 0.0, tile.size[0], tile.size[1]])
        .color(color)        
}
*/


pub fn texture_from_data<T>(creator: &TextureCreator<T>, data: &[u8], width: u32, height: u32) -> Texture {
    let mut tex =
        creator 
        .create_texture(PixelFormatEnum::RGBA8888, 
        TextureAccess::Static, 
        width, height).unwrap();

    let r = Rect::new(0, 0, width, height);
    tex.update(Some(r), data, width as usize * 4).unwrap();
    
    tex
}


pub fn load_texture<T>(creator: &TextureCreator<T>, filename: &str) -> Texture {

    // println!("{}", filename);

    let image = image::open(filename).unwrap();
    let width = image.width();
    let height = image.height();

    let mut tex =
        creator 
            .create_texture(PixelFormatEnum::RGBA8888, 
                            TextureAccess::Static,
                            width, 
                            height)
            .unwrap();

    let mut data: Vec<u8> = Vec::with_capacity((width * height) as usize);

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y).to_rgba();

            // texture channels are ARGB order

            data.push(pixel[3]);
            data.push(pixel[2]);
            data.push(pixel[1]);
            data.push(pixel[0]);
        }
    }

    let r = Rect::new(0, 0, width, height);
    tex.update(Some(r), &data, width as usize * 4).unwrap();

    tex
}


fn quadratic_fade(x: f64) -> f32 {
    (1.0 - (x*x)) as f32
}


fn game_loop(app: &mut App, sdl_context: &mut Sdl) {

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut go = true;

    while go {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {go = false;},
                _ => {}
            }
        }

        app.update();
        app.render();
    }
}

fn main() {
    
    let window_size = [1200, 770];

    let mut sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Fractal Lands 0.0.1", window_size[0], window_size[1])
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string()).unwrap();

    // Create a new game and run it.
    let mut app = App::new(window, window_size);

    game_loop(&mut app, &mut sdl_context);
}
