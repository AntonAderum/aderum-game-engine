use game_object::GameObject;
use sdl2::rect::Rect;
use game_engine::GameEngine::game_engine::Pointf;
use sdl2::video::Window;
use sdl2::render::Canvas;
use std::collections::HashMap;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use game_object::GameObjectTrait;
use game_engine::GameEngine::game_engine::ObjectUsingPhysics;
use game_engine::GameEngine::game_engine::Kinectic;
use game_engine::GameEngine::game_engine::CollisionTypes;
use game_engine::GameEngine::game_engine::Physics2D;

#[derive(Debug)]
pub struct Player {
    pub game_object: GameObject,
}
impl Player {
    pub fn new() -> Player {

        let pos = Pointf { x: 320.0, y: 240.0 };
        let size = Pointf { x: 1.0, y: 1.0 };
        let physics = Physics2D {
            mass: 1.0,
            use_gravity: true,
            is_kinectic: Kinectic::No(Pointf { x: 0.0, y: 0.0 }),
            collision_type: CollisionTypes::BoundingBox(
                Pointf { x: 64.0, y: 64.0 },
                Pointf { x: 64.0, y: 64.0 },
            ),
            id: String::from("obj1"),
        };
        let gam = GameObject {
            position: pos,
            rotation: 0.0,
            size: size,
            speed: 150.0,
            object_using_physics: ObjectUsingPhysics::Yes(physics),
            canjump: true,
            color: Color::RGB(200, 153, 204),
        };
        let pl = Player { game_object: gam };
        return pl;
    }
}

impl GameObjectTrait for Player {
    fn update(&mut self, delta_time: &f64, keyboard_input: &HashMap<Keycode, bool>) {
        match keyboard_input.get(&Keycode::W) {
            Some(o) => {
                if *o && self.game_object.canjump {
                    match self.game_object.object_using_physics {
                        ObjectUsingPhysics::Yes(ref mut phys) => {
                            phys.add_jump_force(100.0);
                            self.game_object.canjump = false;
                        }
                        _ => self.game_object.canjump = true,
                    }
                }
            }
            None => {
                self.game_object.canjump = true;
            }
        }

        if let Some(o) = keyboard_input.get(&Keycode::S) {
            if *o {
                self.game_object.position.y += self.game_object.speed * delta_time;
            }
        }
        if let Some(o) = keyboard_input.get(&Keycode::D) {
            if *o {
                self.game_object.position.x += self.game_object.speed * delta_time;
            }
        }
        if let Some(o) = keyboard_input.get(&Keycode::A) {
            if *o {
                self.game_object.position.x -= self.game_object.speed * delta_time;
            }
        }
    }
    fn draw(&self, rend: &mut Canvas<Window>) {
        // Create centered Rect, draw the outline of the Rect in our dark blue color.
        let border_rect = Rect::new(
            self.game_object.position.x as i32 - 64,
            self.game_object.position.y as i32 - 64,
            128,
            128,
        );
        let _ = rend.draw_rect(border_rect);

        // Create a smaller centered Rect, filling it in the same dark blue.
        let inner_rect = Rect::new(
            self.game_object.position.x as i32 - 60,
            self.game_object.position.y as i32 - 60,
            128,
            128,
        );
        let _ = rend.fill_rect(inner_rect);
    }

    fn collision_enter(&mut self, _other: &GameObject) {}
    fn collision_stay(&mut self, _other: &GameObject) {}
    fn collision_exit(&mut self, _other: &GameObject) {}

    fn get_game_object<'a>(&'a mut self) -> &mut GameObject {
        &mut self.game_object
    }
}
