use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::ops::Add;
use game_object::GameObjectTrait;
use game_engine::GameEngine::game_engine::Pointf;
use floor::Floor;
use player::Player;
use background::Background;
use sdl2;

pub struct LevelLoader<'a> {
    level_names: Vec<String>,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
}

impl<'a> LevelLoader<'a> {
    pub fn new(
        texture_creator2: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    ) -> LevelLoader<'a> {
        LevelLoader {
            level_names: Vec::new(),
            texture_creator: texture_creator2,
        }
    }
    pub fn get_level_names(&self) -> Vec<String> {
        self.level_names.clone()
    }
    pub fn get_game_objects_for_level(&self, level: &String) -> Vec<Box<GameObjectTrait + 'a>> {
        let background = Background::new(Pointf { x: 0.0, y: 0.0 }, &self.texture_creator);
        let object = Player::new(
            Pointf { x: 320.0, y: 240.0 },
            String::from("player"),
            &self.texture_creator);
        let object2 = Floor::new(
            Pointf { x: 320.0, y: 550.0 },
            String::from("floor1"),
            &self.texture_creator,
        );
        let object3 = Floor::new(
            Pointf { x: 600.0, y: 480.0 },
            String::from("floor2"),
            &self.texture_creator,
        );
        let mut vec: Vec<Box<GameObjectTrait>> = Vec::new();
        vec.push(Box::new(background));
        vec.push(Box::new(object));
        vec.push(Box::new(object2));
        vec.push(Box::new(object3));
        vec
    }
    pub fn load_level_names(&mut self) {
        let mut f = File::open(Path::new("Assets/levels.txt")).expect("ERROR: COULD NOT FIND level.txt!! ABORTING!!!");
        let mut file = BufReader::new(&f);
        let mut reading_level = false;
        for line in file.lines() {
            let line = line.unwrap();
            if line.starts_with("#") {
                continue;
            }
            if reading_level {
                if line.starts_with("name: ") {
                    let mut string = String::new();
                    string.push_str(&line[6..]);
                    self.level_names.push(string);
                }
            }
            else if line.starts_with("[level]") {
                reading_level = true;
            }
            else if line.starts_with("[/level]") {
                reading_level = false;
            }
            else if line.starts_with("[") {
                reading_level = false;
            }
            //println!("{}", line);
        }
    }
}