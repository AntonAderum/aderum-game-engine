
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::collections::HashMap;
use sdl2::keyboard::Keycode;
use game_object::GameObject;
use game_object::GameObjectTrait;
use game_engine::GameEngine::game_engine::ObjectUsingPhysics;
use game_engine::GameEngine::game_engine::CollInfoType;
use game_engine::GameEngine::game_engine::Physics2D;
use game_engine::GameEngine::game_engine::Pointf;

#[derive(Debug)]
pub struct GameManager {
    
    obj_vec : Vec<GameObject>,
    
}
pub fn checko_collision( this: &mut GameObject, other: &mut GameObject, coll_info: &mut HashMap<String,CollInfoType>, delta_time: &f64){
                let mut coll_type = CollInfoType::None;
                match this.object_using_physics {
                    ObjectUsingPhysics::Yes(ref mut physics) => {
                    if let ObjectUsingPhysics::Yes( ref mut other_physics) = other.object_using_physics{
                        coll_type = physics.collision(&mut this.position, other_physics,&mut other.position, coll_info,delta_time);
                    }
                    },
                    _ => {}
                }   

            match coll_type{
                CollInfoType::Enter => {
                    this.CollisionEnter(other);
                    other.CollisionEnter(this);
                },
                CollInfoType::Stay => {
                    this.CollisionStay(other);
                    other.CollisionStay(this);
                },
                CollInfoType::Exit => {
                    this.CollisionExit(other);
                    other.CollisionExit(this);
                },
                CollInfoType::None => ()
            }
}


impl GameManager{
    pub fn init() -> GameManager{
        let object = GameObject::new();
        let mut obj_vec = Vec::new();
        obj_vec.push(object);
        let object = GameObject::new_floor(Pointf{x: 20.0, y:450.0});
        obj_vec.push(object);
        let object = GameObject::new_floor(Pointf{x: 320.0, y:550.0});
        obj_vec.push(object);
        let object = GameObject::new_floor(Pointf{x: 650.0, y:450.0});
        obj_vec.push(object);
        let manager = GameManager{obj_vec: obj_vec};
        return manager;
    }

    
    pub fn update(&mut self, delta_time: &f64, keyboard_input: &HashMap<Keycode,bool>, coll_info: &mut HashMap<String,CollInfoType>){
        
        for item in self.obj_vec.iter_mut(){
            item.update(delta_time, keyboard_input);
            if let ObjectUsingPhysics::Yes(ref mut physics) = item.object_using_physics {
                physics.update(&mut item.position,delta_time);
            }
        }    
        //let cl = self.obj_vec.clone();
        for x in 0..self.obj_vec.len()-1{
            let (fir,sec)  = self.obj_vec.split_at_mut(x+1);
            for y in 0..sec.len(){               
                checko_collision(&mut fir[x], &mut sec[y], coll_info,delta_time);   
            } 
        } 
    
    }

    pub fn draw(&self, rend : &mut Canvas<Window>){

        // Set the drawing color to a light blue.
        let _ = rend.set_draw_color(Color::RGB(101, 208, 246));

        // Clear the buffer, using the light blue color set above.
        let _ = rend.clear();

        // Set the drawing color to a darker blue.
        let _ = rend.set_draw_color(Color::RGB(0, 153, 204));

        for item in self.obj_vec.iter(){
            item.draw(rend);
        }

        rend.present();
    }
}
