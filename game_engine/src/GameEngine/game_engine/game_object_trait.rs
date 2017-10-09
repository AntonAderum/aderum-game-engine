
extern crate std;
extern crate sdl2;
use std::collections::HashMap;
use self::sdl2::keyboard::Keycode;
use GameEngine::game_engine::camera::Camera;
use GameEngine::game_engine::game_object::GameObject;
use GameEngine::game_engine::game_engine_manager::GameEngineManager;
use std::fmt;

pub trait GameObjectTrait {
    fn init(&mut self) {}
    fn update(&mut self, _delta_time: &f64, _keyboard_input: &HashMap<Keycode, bool>) {}
    fn draw(&self, _camera: &mut Camera) {}

    fn collision_enter(&mut self, _other: &GameObject) {}
    fn collision_stay(&mut self, _other: &GameObject) {}
    fn collision_exit(&mut self, _other: &GameObject) {}

    // fn GetObjectUsingPhysics( self) -> ObjectUsingPhysics;
    fn get_game_object(&mut self) -> &mut GameObject;
}

impl std::fmt::Debug for GameObjectTrait {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Gameobject trait")
    }
}

pub trait GameManagerTrait {
    fn init(&mut self, manager: &mut GameEngineManager) {}
    fn pre_update(
        &mut self,
        _delta_time: &f64,
        _keyboard_input: &HashMap<Keycode, bool>,
        manager: &mut GameEngineManager,
    ) {
    }
    fn post_update(
        &mut self,
        _delta_time: &f64,
        _keyboard_input: &HashMap<Keycode, bool>,
        manager: &mut GameEngineManager,
    ) {
    }
}

impl std::fmt::Debug for GameManagerTrait {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Gameobject trait")
    }
}
