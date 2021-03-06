extern crate sdl2;
use std::collections::HashMap;
use self::sdl2::keyboard::Keycode;
use GameEngine::game_engine::game_object_trait::GameObjectTrait;
use GameEngine::game_engine::game_object_trait::GameManagerTrait;
use GameEngine::game_engine::ObjectUsingPhysics;
use GameEngine::game_engine::CollInfoType;
use GameEngine::game_engine::pointf::Pointf;
use GameEngine::game_engine::camera::Camera;
use self::sdl2::render::Texture as SdlTexture;
use self::sdl2::render::TextureCreator;
use self::sdl2::video::WindowContext;
use std::path::Path;

pub struct GameEngineManager<'a> {
    obj_vec: Vec<Box<GameObjectTrait + 'a>>,
    pub texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    camera: &'a mut Camera<'a>,
}

pub fn checko_collision<'a>(
    this: &mut Box<GameObjectTrait + 'a>,
    other: &mut Box<GameObjectTrait + 'a>,
    coll_info: &mut HashMap<String, CollInfoType>,
    delta_time: &f64,
) -> CollInfoType {
    let mut coll_type = CollInfoType::None;

    let this_obj = this.get_game_object();
    let other_obj = other.get_game_object();
    match this_obj.object_using_physics {
        ObjectUsingPhysics::Yes(ref mut physics) => {
            if let ObjectUsingPhysics::Yes(ref mut other_physics) = other_obj.object_using_physics {
                coll_type = physics.collision(
                    &mut this_obj.position,
                    other_physics,
                    &mut other_obj.position,
                    coll_info,
                );
            }
        }
        _ => {}
    }
    coll_type

}

pub fn send_collision_report<'a>(
    this: &mut Box<GameObjectTrait + 'a>,
    other: &mut Box<GameObjectTrait + 'a>,
    coll_type: CollInfoType,
) {
    match coll_type {
        CollInfoType::Enter => {
            this.collision_enter(other.get_game_object());
            other.collision_enter(this.get_game_object());
        }
        CollInfoType::Stay => {
            this.collision_stay(other.get_game_object());
            other.collision_stay(this.get_game_object());
        }
        CollInfoType::Exit => {
            this.collision_exit(other.get_game_object());
            other.collision_exit(this.get_game_object());
        }
        CollInfoType::None => (),
    }
}

impl<'a> GameEngineManager<'a> {
    pub fn new(
        texture_creator2: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        camera: &'a mut Camera<'a>,
    ) -> GameEngineManager<'a> {
        GameEngineManager {
            obj_vec: Vec::new(),
            texture_creator: texture_creator2,
            camera: camera,
        }
    }
    pub fn create_texture(&self, filePath: String) -> SdlTexture {
        let temp_surface = sdl2::surface::Surface::load_bmp(Path::new(&filePath)).unwrap();
        self.texture_creator
            .create_texture_from_surface(&temp_surface)
            .unwrap()
    }

    pub fn add_game_object(&mut self, obj: Box<GameObjectTrait + 'a>) {
        self.obj_vec.push(obj);
    }

    pub fn remove_gameobject(&mut self, obj: &mut Box<GameObjectTrait + 'a>) {
        for x in 0..self.obj_vec.len() {
            let id = self.obj_vec[x].get_game_object().get_id();
            if (id == obj.get_game_object().get_id()) {
                self.obj_vec.remove(id as usize);
                break;
            }
        }
    }
    pub fn init<T: GameManagerTrait>(&mut self, game_manager: &mut T) {
        game_manager.init(self);
    }

    fn focus_camera(&mut self) {
        let obj = self.obj_vec[1].get_game_object();
        let pos = obj.get_position();
        self.camera.SetOffset(pos.x, pos.y);
    }
    pub fn update<T: GameManagerTrait>(
        &mut self,
        delta_time: &f64,
        keyboard_input: &HashMap<Keycode, bool>,
        coll_info: &mut HashMap<String, CollInfoType>,
        game_manager: &mut T,
    ) {
        game_manager.pre_update(delta_time, keyboard_input, self);
        for item in self.obj_vec.iter_mut() {
            item.update(delta_time, keyboard_input);
            let obj = item.get_game_object();
            if let ObjectUsingPhysics::Yes(ref mut physics) = obj.object_using_physics {
                physics.update(&mut obj.position, delta_time);
            }
        }
        self.focus_camera();

        for x in 0..self.obj_vec.len() - 1 {
            let (fir, sec) = self.obj_vec.split_at_mut(x + 1);
            for y in 0..sec.len() {
                let coll_type = checko_collision(&mut fir[x], &mut sec[y], coll_info, delta_time);
                send_collision_report(&mut fir[x], &mut sec[y], coll_type);
            }
        }
        game_manager.post_update(delta_time, keyboard_input, self);

    }

    pub fn draw(&mut self) {
        self.camera.Clear();

        for item in self.obj_vec.iter() {
            item.draw(&mut self.camera);
        }
        self.camera.Present();
    }
}
