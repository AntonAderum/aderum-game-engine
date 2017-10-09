
extern crate std;
extern crate sdl2;
use self::sdl2::pixels::Color;
use GameEngine::game_engine::ObjectUsingPhysics;
use GameEngine::game_engine::pointf::Pointf;

static mut NEXTID: u32 = 0;
#[derive(Debug)]
pub struct GameObject {
    pub position: Pointf,
    pub size: Pointf,
    pub rotation: f64,
    pub speed: f64,
    pub object_using_physics: ObjectUsingPhysics,
    pub canjump: bool,
    pub color: Color,
    id: u32,
}

impl GameObject {
    pub fn new(
        pos: Pointf,
        size: Pointf,
        rotation: f64,
        speed: f64,
        object_using_physics: ObjectUsingPhysics,
        canjump: bool,
        color: Color,
    ) -> GameObject {
        unsafe {
            NEXTID += 1;
            GameObject {
                position: pos,
                size: size,
                rotation: rotation,
                speed: speed,
                object_using_physics: object_using_physics,
                canjump: canjump,
                color: color,
                id: NEXTID,
            }
        }
    }
    pub fn get_position(&mut self) -> &Pointf {
        &self.position
    }
    pub fn get_id(&mut self) -> u32 {
        self.id
    }
}
