use game_object::GameObject;
use game_engine::GameEngine::game_engine::pointf::Pointf;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use game_object::GameObjectTrait;
use game_engine::GameEngine::game_engine::ObjectUsingPhysics;
use sdl2::render::Texture as SdlTexture;
use game_engine::GameEngine::game_engine::camera::Camera;
extern crate sdl2;
use std::path::Path;

pub struct Background<'a> {
    pub game_object: GameObject,
    pub texture: SdlTexture<'a>,
}

impl<'a> Background<'a> {
    pub fn new(
        pos: Pointf,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    ) -> Background<'a> {

        let size = Pointf { x: 1.0, y: 1.0 };
        let gam = GameObject {
            position: pos,
            rotation: 0.0,
            size: size,
            speed: 0.0,
            object_using_physics: ObjectUsingPhysics::None,
            canjump: false,
            color: Color::RGB(0, 153, 0),
        };
        let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("Assets/background.bmp"))
            .unwrap();
        let floor = Background {
            game_object: gam,
            texture: texture_creator
                .create_texture_from_surface(&temp_surface)
                .unwrap(),
        };
        return floor;
    }
}

impl<'a> GameObjectTrait for Background<'a> {
    fn draw(&self, _camera: &mut Camera) {
        _camera.DrawBackground(&self.texture);
        //rend.copy(&self.texture, None, None);
    }

    fn collision_enter(&mut self, _other: &GameObject) {}
    fn collision_stay(&mut self, _other: &GameObject) {}
    fn collision_exit(&mut self, _other: &GameObject) {}
    fn get_game_object<'c>(&'c mut self) -> &mut GameObject {
        &mut self.game_object
    }
}
