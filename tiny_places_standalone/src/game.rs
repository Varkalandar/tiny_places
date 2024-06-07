use vecmath::{Vector2, vec2_sub, vec2_add, vec2_scale, vec2_normalized};

use piston::{ButtonState, MouseButton};
use graphics::DrawState;
use opengl_graphics::GlGraphics;
use graphics::Viewport;

use crate::ui::{UI, UiController, ButtonEvent, MouseMoveEvent, ScrollEvent};
use crate::sound::Sound;
use crate::GameWorld;
use crate::screen_to_world_pos;
use crate::player_inventory_view::PlayerInventoryView;
use crate::TileSet;
use crate::MAP_OBJECT_LAYER;
use crate::map::MoveEndAction;

pub struct Game {
    piv: PlayerInventoryView,
    show_inventory: bool,
}


impl UiController for Game {
    type Appdata = GameWorld;

    /**
     * @return true if this controller could handle the event, false to pass the event to other controllers
     */
     fn handle_button_event(&mut self, ui: &mut UI, event: &ButtonEvent, world: &mut Self::Appdata) -> bool {
        // first pass the even to the UI. if there is a component
        // trigered it will consume the event. Events which are not
        // consumed by the UI will be handed to the game core

        let comp = ui.handle_button_event(&event);

        if event.args.state == ButtonState::Release {

            match comp {
                None => {

                    let pos = screen_to_world_pos(&ui, &world.map.player_position(), &ui.mouse_state.position);
                    
                    if event.args.button == piston::Button::Mouse(MouseButton::Left) {
                        ui.root.head.clear();

                        let map = &mut world.map;
                        let option = map.find_nearest_object(map.selected_layer, &pos, 100.0, 0);

                        match option {
                            None => {
                                // nothing clicked -> move player
                                map.has_selection = false;
                            },
                            Some(_idx) => {
                                // pick up the item?
                                return true;
                            }
                        }
                    }

                    if event.args.button == piston::Button::Mouse(MouseButton::Right) {
                        let id = world.map.player_id;
                        self.fire_projectile(world, id, MAP_OBJECT_LAYER, 25, pos, 200.0)
                    }

                    if event.args.button == piston::Button::Keyboard(piston::Key::I) {
                        self.show_inventory = true;
                    }        
                },
                Some(_comp) => {
                }
            }
        
        }

        if self.show_inventory {
            return self.piv.handle_button_event(event, &ui.mouse_state, world);
        }

        false
    }


    fn handle_mouse_move_event(&mut self, ui: &mut UI, event: &MouseMoveEvent, world: &mut Self::Appdata) -> bool {
        let _comp = ui.handle_mouse_move_event(event);

        if self.show_inventory {
            self.piv.handle_mouse_move_event(event, &ui.mouse_state, &mut world.player_inventory);
        }

        false
    }


    /**
     * @return true if this controller could handle the event, false to pass the event to other controllers
     */
    fn handle_scroll_event(&mut self, ui: &mut UI, event: &ScrollEvent, _world: &mut Self::Appdata) -> bool {

        let _comp = ui.handle_scroll_event(event);

        false
    }


    fn draw(&mut self, viewport: Viewport, gl: &mut GlGraphics, ds: &DrawState, ui: &mut UI, world: &mut Self::Appdata) {
        ui.draw(viewport, gl);
 
        if self.show_inventory {
            self.piv.draw(viewport, gl, ds, 0, 10, &world.player_inventory)
        }
    }


    fn draw_overlay(&mut self, viewport: Viewport, gl: &mut GlGraphics, ds: &DrawState, ui: &mut UI, _world: &mut Self::Appdata) {
        ui.font_14.draw(viewport, gl, ds, 10, 20, "Game testing mode", &[1.0, 1.0, 1.0, 1.0]);
    }


    fn update(&mut self, world: &mut Self::Appdata, dt: f64) {
        let map = &mut world.map;
        let rng = &mut world.rng;
        let speaker = &mut world.speaker;
        map.update(dt, rng, speaker);
    }
}


impl Game {

    pub fn new(ui: &UI, item_tiles: &TileSet) -> Game {
        let piv = PlayerInventoryView::new((ui.window_size[0] as i32) / 2, 0,
        &ui.font_14,
        &item_tiles.shallow_copy());
    
        Game {
            piv,
            show_inventory: false,
        }
    }


    pub fn fire_projectile(&mut self, world: &mut GameWorld, shooter_id: u64, layer: usize, projectile_type: usize, fire_at: Vector2<f64>, speed: f64) {
        println!("Adding projectile with type {} fired at {:?}", projectile_type, fire_at);

        world.speaker.play_sound(Sound::FireballLaunch);

        let map = &mut world.map;

        let shooter = &map.layers[layer][&shooter_id];
        let np = vec2_sub(fire_at, shooter.position);
    
        let dir = vec2_normalized(np);
        let velocity = vec2_scale(dir, speed);

        let start_pos = vec2_add(shooter.position, vec2_scale(dir, 80.0));

        let mut projectile = map.factory.create_mob(projectile_type, 5, start_pos, 1.0);
        projectile.velocity = velocity;
        projectile.move_time_left = 2.0;
        projectile.move_end_action = MoveEndAction::RemoveFromMap;
        projectile.attributes.is_projectile = true;

        let offset = projectile.visual.orient(velocity[0], velocity[1]);
        projectile.visual.current_image_id = projectile.visual.base_image_id + offset;

        map.layers[layer].insert(projectile.uid, projectile);
    } 
}
