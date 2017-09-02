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
use sdl2::render::Texture as SdlTexture;
extern crate sdl2;
use std;
use std::path::Path;
//#[derive(Debug)]
pub struct Player<'a> {
    pub game_object: GameObject,
    pub texture: SdlTexture<'a>,
    anim: f64,
}
impl<'a> Player<'a> {
    pub fn new(
        name: String,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    ) -> Player<'a> {

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
            id: name,
        };
        let gam = GameObject {
            position: pos,
            rotation: 0.0,
            size: size,
            speed: 2550.0,
            object_using_physics: ObjectUsingPhysics::Yes(physics),
            canjump: true,
            color: Color::RGB(200, 153, 204),
        };

        let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("Assets/hero.bmp")).unwrap();
        Player {
            game_object: gam,
            texture: texture_creator
                .create_texture_from_surface(&temp_surface)
                .unwrap(),
            anim: 0.0,
        }
    }
}
impl<'a> std::fmt::Debug for Player<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Gameobject trait")
    }
}

impl<'a> GameObjectTrait for Player<'a> {
    fn update(&mut self, delta_time: &f64, keyboard_input: &HashMap<Keycode, bool>) {
        self.anim += *delta_time * 100.0;
        match keyboard_input.get(&Keycode::W) {
            Some(o) => {
                if *o {
                    if self.game_object.canjump {
                        match self.game_object.object_using_physics {
                            ObjectUsingPhysics::Yes(ref mut phys) => {
                                phys.add_jump_force(0.4);
                                self.game_object.canjump = false;
                            }
                            _ => self.game_object.canjump = true,
                        }
                    }
                } else {
                    self.game_object.canjump = true;
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
        let inner_rect = Rect::new(
            self.game_object.position.x as i32 - 60,
            self.game_object.position.y as i32 - 60,
            128,
            128,
        );
        let rect = Rect::new(64 * (self.anim as i32 % 8), 0, 50, 100);
        rend.copy(&self.texture, rect, inner_rect);
    }

    fn collision_enter(&mut self, _other: &GameObject) {}
    fn collision_stay(&mut self, _other: &GameObject) {}
    fn collision_exit(&mut self, _other: &GameObject) {}

    fn get_game_object<'c>(&'c mut self) -> &mut GameObject {
        &mut self.game_object
    }
}
