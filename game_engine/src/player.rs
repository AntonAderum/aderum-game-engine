use game_object::GameObject;
use sdl2::rect::Rect;
use game_engine::GameEngine::game_engine::pointf::Pointf;
use sdl2::video::Window;
use sdl2::render::Canvas;
use std::collections::HashMap;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use game_object::GameObjectTrait;
use game_engine::GameEngine::game_engine::ObjectUsingPhysics;
use game_engine::GameEngine::game_engine::CollisionTypes;
use game_engine::GameEngine::game_engine::physics2d::Physics2D;
use game_engine::GameEngine::game_engine::Material;
use sdl2::render::Texture as SdlTexture;
use game_engine::GameEngine::game_engine::camera::Camera;
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
        pos: Pointf,
        name: String,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    ) -> Player<'a> {

        let size = Pointf { x: 64.0, y: 64.0 };
        let physics = Physics2D {
            mass: 1.0,
            use_gravity: true,
            is_kinectic: false,
            velocity: Pointf { x: 0.0, y: 0.0 },
            collision_type: CollisionTypes::BoundingBox(
                Pointf { x: 0.0, y: 0.0 },
                Pointf {
                    x: size.x / 2.0,
                    y: size.y / 2.0,
                },
            ),
            material: Material {
                bounciness: 1.0,
                friction: 0.0,
            },
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

        let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("Assets/viking.bmp")).unwrap();
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
                                phys.add_jump_force(0.6);
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
                match self.game_object.object_using_physics {
                    ObjectUsingPhysics::Yes(ref mut phys) => {
                        phys.add_side_force(20.0, 0.3, delta_time);
                    }
                    _ => (),
                }
                //self.game_object.position.x += self.game_object.speed * delta_time;
            }
        }
        if let Some(o) = keyboard_input.get(&Keycode::A) {
            if *o {
                match self.game_object.object_using_physics {
                    ObjectUsingPhysics::Yes(ref mut phys) => {
                        phys.add_side_force(-20.0, 0.3, delta_time);
                    }
                    _ => (),
                }
                //self.game_object.position.x -= self.game_object.speed * delta_time;
            }
        }
    }
    fn draw(&self, camera: &mut Camera) {

        let x_size = self.game_object.size.x as i32;
        let y_size = self.game_object.size.y as i32;
        let mut dest_rect = Rect::new(
            self.game_object.position.x as i32 - x_size / 2,
            self.game_object.position.y as i32 - y_size / 2,
            x_size as u32,
            y_size as u32,
        );
        let src_rect = Rect::new(64 * (self.anim as i32 % 8) + 15, 30, 35, 34);
        camera.DrawRec(&mut dest_rect);
        camera.DrawPartOfTexture(&self.texture, src_rect, &mut dest_rect);
        //_camera.DrawRec(dest_rect);
        // rend.copy(&self.texture, src_rect, dest_rect);
        // rend.draw_rect(dest_rect);
    }

    fn collision_enter(&mut self, _other: &GameObject) {}
    fn collision_stay(&mut self, _other: &GameObject) {}
    fn collision_exit(&mut self, _other: &GameObject) {}

    fn get_game_object<'c>(&'c mut self) -> &mut GameObject {
        &mut self.game_object
    }
}

