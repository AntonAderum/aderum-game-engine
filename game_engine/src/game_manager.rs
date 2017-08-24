
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::collections::HashMap;
use sdl2::keyboard::Keycode;
use game_object::GameObject;
use game_object::GameObjectTrait;
use game_engine::GameEngine::game_engine::ObjectUsingPhysics;

#[derive(Debug)]
pub struct GameManager {
    
    obj_vec : Vec<GameObject>,
}


pub fn checko_collision( this: &mut GameObject, other: &mut GameObject){
                let mut col = false;
                match this.object_using_physics {
                ObjectUsingPhysics::Yes(ref mut physics) => {
                    match other.object_using_physics {
                        ObjectUsingPhysics::Yes(ref mut other_physics) => {
                            if physics.collision(&mut this.position, other_physics,& other.position){
                                col = true;
                                println!("{}","CRRRRRRRASH" );
                            }
                        },
                        _ => {}
                    
                }
                },
                _ => {}
            }
            if col
            {
                this.CollisionEnter(other);
            }
}
impl GameManager{
    pub fn init() -> GameManager{
        let object = GameObject::new();
        let mut obj_vec = Vec::new();
        obj_vec.push(object);
        let object = GameObject::new_floor();
        obj_vec.push(object);
        let manager = GameManager{obj_vec: obj_vec};
        return manager;
    }

    
    pub fn update(&mut self, delta_time: &f64, keyboard_input: &HashMap<Keycode,bool>){
        
        for item in self.obj_vec.iter_mut(){
            item.update(delta_time, keyboard_input);
            if let ObjectUsingPhysics::Yes(ref mut physics) = item.object_using_physics {
                physics.update(&mut item.position,delta_time);
            }
        }    
        //let cl = self.obj_vec.clone();
        for x in 0..self.obj_vec.len()-1{
                let (fir,sec)  = self.obj_vec.split_at_mut(x+1);
                checko_collision(&mut fir[x], &mut sec[0]);    
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
