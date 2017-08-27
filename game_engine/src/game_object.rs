

use sdl2::rect::{Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::collections::HashMap;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use game_engine::GameEngine::game_engine::ObjectUsingPhysics;   
use game_engine::GameEngine::game_engine::Kinectic;
use game_engine::GameEngine::game_engine::Pointf;
use game_engine::GameEngine::game_engine::Physics2D;
use game_engine::GameEngine::game_engine::CollisionTypes;
    


pub trait GameObjectTrait{
    fn update(&mut self, delta_time: &f64, keyboard_input: &HashMap<Keycode,bool>){

    }
    fn draw(& self, rend : &mut Canvas<Window>){}

    fn CollisionEnter(&mut self, other: &GameObject){

    }
    fn CollisionStay(&mut self, other: &GameObject){
        
    }
    fn CollisionExit(&mut self, other: &GameObject){
        
    }
}

#[derive(Debug)]
pub struct GameObject {
    pub position: Pointf,
    pub size: Pointf,
    pub rotation: f64,
    pub speed: f64,
    pub object_using_physics : ObjectUsingPhysics,
    canjump:bool,
    pub color: Color,
}


impl GameObjectTrait for GameObject{
    fn update(&mut self, delta_time: &f64, keyboard_input: &HashMap<Keycode,bool>){
match keyboard_input.get(&Keycode::W){
                    Some(o) => {
                        if *o{

                         if self.canjump {
                            match self.object_using_physics {
                                ObjectUsingPhysics::Yes(ref mut phys) => {
                                    phys.add_jump_force(0.5);
                                    self.canjump = false;
                                },
                                _ => {self.canjump = true}
                            }
                         }
                        }
                        else{
                            self.canjump = true
                        }
                        
                    },
                    None => {self.canjump = true;},
                }

                if let Some(o) = keyboard_input.get(&Keycode::S){
                    if *o {
                            self.position.y += self.speed * delta_time;
                        }
                }
                if let Some(o) = keyboard_input.get(&Keycode::D){
                        if *o {
                            self.position.x += self.speed * delta_time;
                        }
                }
                if let Some(o) = keyboard_input.get(&Keycode::A){
                        if *o {
                            self.position.x -= self.speed * delta_time;
                        }
                }

    }
    fn draw(& self, rend : &mut Canvas<Window>){
        let _ = rend.set_draw_color(self.color);
        let border_rect = Rect::new(self.position.x as i32-64, self.position.y as i32-64, 128, 128);
        let _ = rend.draw_rect(border_rect);

        // Create a smaller centered Rect, filling it in the same dark blue.
        let inner_rect = Rect::new(self.position.x as i32 -60, self.position.y as i32-60, 128, 128);
        let _ = rend.fill_rect(inner_rect);
    }
}

impl GameObject{
     pub fn new() -> GameObject{
        
        let pos =  Pointf{x: 320.0, y:240.0};
        let size =  Pointf{x: 1.0, y:1.0};
        let physics = Physics2D{mass: 1.0, use_gravity: true, is_kinectic: Kinectic::No(Pointf{x:0.0, y:0.0}),collision_type: CollisionTypes::BoundingBox(Pointf{x:64.0,y:64.0},Pointf{x:64.0, y:64.0}),id:String::from("obj1")};
        let gam = GameObject{position: pos,rotation: 0.0, size: size, speed: 150.0, object_using_physics: ObjectUsingPhysics::Yes(physics), canjump:true, color: Color::RGB(200, 153, 204)};
        //let pl = Player{gameObject: gam};
        return gam;
    }
     pub fn new_floor(pos: Pointf) -> GameObject{
        
        //let pos =  Pointf{x: 320.0, y:550.0};
        let size =  Pointf{x: 1.0, y:1.0};
        let physics = Physics2D{mass: 1.0, use_gravity: false, is_kinectic: Kinectic::_Yes, collision_type: CollisionTypes::BoundingBox(Pointf{x:64.0,y:64.0},Pointf{x:64.0, y:64.0}), id:String::from("obj2")};
        let gam = GameObject{position: pos,rotation: 0.0, size: size, speed: 0.0, object_using_physics: ObjectUsingPhysics::Yes(physics), canjump:true,color: Color::RGB(0, 153, 0)};
       // let floor = Floor{game_object: gam};
        return gam;
    }
}
