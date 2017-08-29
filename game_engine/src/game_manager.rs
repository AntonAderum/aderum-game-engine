
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::collections::HashMap;
use sdl2::keyboard::Keycode;
use game_object::GameObjectTrait;
use game_engine::GameEngine::game_engine::ObjectUsingPhysics;
use game_engine::GameEngine::game_engine::CollInfoType;
use game_engine::GameEngine::game_engine::Pointf;
use floor::Floor;
use player::Player;

#[derive(Debug)]
pub struct GameManager {
    obj_vec: Vec<Box<GameObjectTrait>>,
}
pub fn checko_collision(
    this: &mut Box<GameObjectTrait>,
    other: &mut Box<GameObjectTrait>,
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
                    delta_time,
                );
            }
        }
        _ => {}
    }
    coll_type

}

pub fn send_collision_report(
    this: &mut Box<GameObjectTrait>,
    other: &mut Box<GameObjectTrait>,
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


impl GameManager {
    pub fn init() -> GameManager {
        let object = Player::new(String::from("player"));
        let object2 = Floor::new(Pointf { x: 320.0, y: 550.0 }, String::from("floor1"));
        let object3 = Floor::new(Pointf { x: 650.0, y: 450.0 }, String::from("floor2"));
        let manager =
            GameManager { obj_vec: vec![Box::new(object), Box::new(object2), Box::new(object3)] };
        return manager;
    }


    pub fn update(
        &mut self,
        delta_time: &f64,
        keyboard_input: &HashMap<Keycode, bool>,
        coll_info: &mut HashMap<String, CollInfoType>,
    ) {

        for item in self.obj_vec.iter_mut() {
            item.update(delta_time, keyboard_input);
            let obj = item.get_game_object();
            if let ObjectUsingPhysics::Yes(ref mut physics) = obj.object_using_physics {
                physics.update(&mut obj.position, delta_time);
            }
        }

        for x in 0..self.obj_vec.len() - 1 {
            let (fir, sec) = self.obj_vec.split_at_mut(x + 1);
            for y in 0..sec.len() {
                let coll_type = checko_collision(&mut fir[x], &mut sec[y], coll_info, delta_time);
                send_collision_report(&mut fir[x], &mut sec[y], coll_type);
            }
        }

    }

    pub fn draw(&self, rend: &mut Canvas<Window>) {

        // Set the drawing color to a light blue.
        let _ = rend.set_draw_color(Color::RGB(101, 208, 246));

        // Clear the buffer, using the light blue color set above.
        let _ = rend.clear();

        // Set the drawing color to a darker blue.
        let _ = rend.set_draw_color(Color::RGB(0, 153, 204));

        for item in self.obj_vec.iter() {
            item.draw(rend);
        }

        rend.present();
    }
}
