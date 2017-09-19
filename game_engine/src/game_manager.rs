
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::collections::HashMap;
use sdl2::keyboard::Keycode;
use game_object::GameObjectTrait;
use game_engine::GameEngine::game_engine::ObjectUsingPhysics;
use game_engine::GameEngine::game_engine::CollInfoType;
use game_engine::GameEngine::game_engine::pointf::Pointf;
use game_engine::GameEngine::game_engine::camera::Camera;
use floor::Floor;
use player::Player;
use background::Background;
use sdl2;

pub struct GameManager<'a> {
    obj_vec: Vec<Box<GameObjectTrait + 'a>>,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
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


impl<'a> GameManager<'a> {
    pub fn new(
        texture_creator2: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        camera: &'a mut Camera<'a>,
    ) -> GameManager<'a> {
        GameManager {
            obj_vec: Vec::new(),
            texture_creator: texture_creator2,
            camera: camera,
        }
    }
    pub fn init(&mut self) {
        let background = Background::new(Pointf { x: 0.0, y: 0.0 }, &self.texture_creator);
        self.obj_vec.push(Box::new(background));
        let player = Player::new(String::from("player"), &self.texture_creator);
        self.obj_vec.push(Box::new(player));
        let floor = Floor::new(
            Pointf { x: 320.0, y: 550.0 },
            String::from("floor1"),
            &self.texture_creator,
        );
        self.obj_vec.push(Box::new(floor));
        let floor = Floor::new(
            Pointf { x: 600.0, y: 480.0 },
            String::from("floor2"),
            &self.texture_creator,
        );
        self.obj_vec.push(Box::new(floor));
        let floor = Floor::new(
            Pointf { x: 900.0, y: 480.0 },
            String::from("floor2"),
            &self.texture_creator,
        );
        self.obj_vec.push(Box::new(floor));

        let floor = Floor::new(
            Pointf {
                x: 1100.0,
                y: 480.0,
            },
            String::from("floor2"),
            &self.texture_creator,
        );
        self.obj_vec.push(Box::new(floor));

        let floor = Floor::new(
            Pointf {
                x: 1300.0,
                y: 480.0,
            },
            String::from("floor2"),
            &self.texture_creator,
        );
        self.obj_vec.push(Box::new(floor));

    }

    fn focus_camera(&mut self) {
        let obj = self.obj_vec[1].get_game_object();
        let pos = obj.get_position();
        self.camera.SetOffset(pos.x, pos.y);
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
        self.focus_camera();

        for x in 0..self.obj_vec.len() - 1 {
            let (fir, sec) = self.obj_vec.split_at_mut(x + 1);
            for y in 0..sec.len() {
                let coll_type = checko_collision(&mut fir[x], &mut sec[y], coll_info, delta_time);
                send_collision_report(&mut fir[x], &mut sec[y], coll_type);
            }
        }

    }

    pub fn draw(&mut self) {
        self.camera.Clear();

        for item in self.obj_vec.iter() {
            item.draw(self.camera);
        }
        self.camera.Present();
        //rend.present();
    }
}
