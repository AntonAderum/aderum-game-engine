use game_object::GameObject;
use sdl2::rect::Rect;
use game_engine::GameEngine::game_engine::pointf::Pointf;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use game_object::GameObjectTrait;
use game_engine::GameEngine::game_engine::physics2d::Physics2D;
use game_engine::GameEngine::game_engine::ObjectUsingPhysics;
use game_engine::GameEngine::game_engine::CollisionTypes;
use game_engine::GameEngine::game_engine::Material;
use sdl2::render::Texture as SdlTexture;
use game_engine::GameEngine::game_engine::camera::Camera;
extern crate sdl2;
use std::path::Path;

pub struct Floor<'a> {
    pub game_object: GameObject,
    pub texture: SdlTexture<'a>,
}

impl<'a> Floor<'a> {
    pub fn new(
        pos: Pointf,
        name: String,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    ) -> Floor<'a> {

        let size = Pointf { x: 1.0, y: 1.0 };
        let physics = Physics2D {
            mass: 100.0,
            use_gravity: false,
            is_kinectic: true,
            velocity: Pointf { x: 0.0, y: 0.0 },
            collision_type: CollisionTypes::BoundingBox(
                Pointf { x: 0.0, y: 0.0 },
                Pointf { x: 64.0, y: 64.0 },
            ),
            material: Material {
                bounciness: 0.25,
                friction: 0.0,
            },
            id: name,
        };
        let gam = GameObject {
            position: pos,
            rotation: 0.0,
            size: size,
            speed: 0.0,
            object_using_physics: ObjectUsingPhysics::Yes(physics),
            canjump: true,
            color: Color::RGB(0, 153, 0),
        };
        let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("Assets/floor.bmp")).unwrap();
        let floor = Floor {
            game_object: gam,
            texture: texture_creator
                .create_texture_from_surface(&temp_surface)
                .unwrap(),
        };
        return floor;
    }
}

impl<'a> GameObjectTrait for Floor<'a> {
    fn draw(&self, _camera: &mut Camera) {

        let mut inner_rect = Rect::new(
            self.game_object.position.x as i32 - 64,
            self.game_object.position.y as i32 - 64,
            128,
            128,
        );
        _camera.DrawFullTexture(&self.texture, &mut inner_rect);
        _camera.DrawRec(&mut inner_rect);
    }

    fn collision_enter(&mut self, _other: &GameObject) {}
    fn collision_stay(&mut self, _other: &GameObject) {}
    fn collision_exit(&mut self, _other: &GameObject) {}
    // fn GetObjectUsingPhysics(&mut self) -> ObjectUsingPhysics{
    //     self.game_object.object_using_physics
    // }
    fn get_game_object<'c>(&'c mut self) -> &mut GameObject {
        &mut self.game_object
    }
}
