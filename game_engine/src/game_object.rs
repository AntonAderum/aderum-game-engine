
extern crate std;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::collections::HashMap;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use game_engine::GameEngine::game_engine::ObjectUsingPhysics;
use game_engine::GameEngine::game_engine::pointf::Pointf;
use std::fmt;

pub trait GameObjectTrait {
    fn update(&mut self, _delta_time: &f64, _keyboard_input: &HashMap<Keycode, bool>) {}
    fn draw(&self, _rend: &mut Canvas<Window>) {}

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

#[derive(Debug)]
pub struct GameObject {
    pub position: Pointf,
    pub size: Pointf,
    pub rotation: f64,
    pub speed: f64,
    pub object_using_physics: ObjectUsingPhysics,
    pub canjump: bool,
    pub color: Color,
}


// impl GameObjectTrait for GameObject{
//     fn update(&mut self, delta_time: &f64, keyboard_input: &HashMap<Keycode,bool>){

//     }
//     fn draw(& self, rend : &mut Canvas<Window>){

//     }

//     fn get_game_object<'a>(&'a mut self) -> &mut GameObject{
//        self
//     }
// }
