use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::ops::Add;
use game_object::GameObjectTrait;
use game_engine::GameEngine::game_engine::pointf::Pointf;
use floor::Floor;
use player::Player;
use background::Background;
use sdl2;

pub struct LevelLoader<'a> {
    level_names: Vec<(String, i32)>,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
}

struct LevelObject {
    player: (String, f64, f64),
    floors: Vec<(String, f64, f64)>,
    background: Vec<(f64, f64)>,
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
        self.level_names.clone().into_iter().map(|x| x.0).collect()
    }
    pub fn get_game_objects_for_level(&self, level: &String) -> Vec<Box<GameObjectTrait + 'a>> {
        let lines = self.lines_from_file(Path::new("Assets/levels.txt"));
        let level_starts_at = self.level_names.clone().into_iter().find(|x| &x.0 == level).unwrap().1 as usize;
        let mut i = level_starts_at;
        let mut reading_level = true;
        let mut reading_player = false;
        let mut reading_floor = false;
        let mut player = (String::new(), 0.0, 0.0);
        let mut floors: Vec<(String, f64, f64)> = Vec::new();
        while i < lines.len() {
            if lines[i].starts_with("#") {
                i += 1;
                continue;
            }
            reading_level = self.is_reading_x(&lines[i], "[level]", "[/level]", reading_level);
            while reading_level {
                reading_player = self.is_reading_x(&lines[i], "[player]", "[/player]", reading_player);
                while reading_player {
                    if lines[i].starts_with("id: ") {
                        let mut player_id = String::new();
                        player_id = player_id.add(&lines[i][4..]);
                        player.0 = player_id;
                    }                                       
                    else if (lines[i].starts_with("pos_x: ")) {
                        let mut pos_x = String::new();
                        pos_x = pos_x.add(&lines[i][7..]);
                        player.1 = pos_x.parse::<f64>().unwrap();
                    } else if (lines[i].starts_with("pos_y: ")) {
                        let mut pos_y = String::new();
                        pos_y = pos_y.add(&lines[i][7..]);
                        player.2 = pos_y.parse::<f64>().unwrap();
                    }
                    if i >= lines.len() {
                        reading_player = false;                        
                    } else {
                        i += 1;
                        reading_player = self.is_reading_x(&lines[i], "[player]", "[/player]", reading_player);
                    }
                }
                reading_floor = self.is_reading_x(&lines[i], "[floor]", "[/floor]", reading_floor);
                let mut read_floor = false;
                let mut floor = (String::new(), 0.0, 0.0); 
                while reading_floor {
                    if !read_floor {
                        read_floor = true;
                    }
                    if lines[i].starts_with("id: ") {
                        let mut floor_id = String::new();
                        floor_id = floor_id.add(&lines[i][4..]);
                        floor.0 = floor_id;
                    }                                       
                    else if (lines[i].starts_with("pos_x: ")) {
                        let mut pos_x = String::new();
                        pos_x = pos_x.add(&lines[i][7..]);
                        println!("floor pos_x {}", pos_x);
                        floor.1 = pos_x.parse::<f64>().unwrap();
                    } else if (lines[i].starts_with("pos_y: ")) {
                        let mut pos_y = String::new();
                        pos_y = pos_y.add(&lines[i][7..]);
                        println!("floor pos_y {}", pos_y);
                        floor.2 = pos_y.parse::<f64>().unwrap();
                    }
                    if i >= lines.len() {
                        reading_floor = false;                        
                    } else {
                        i += 1;
                        reading_floor = self.is_reading_x(&lines[i], "[floor]", "[/floor]", reading_floor);
                    }
                }
                if read_floor {
                    floors.push(floor);
                }
                if i >= lines.len() {
                    reading_level = false;
                } else {
                    i += 1;
                    reading_level = self.is_reading_x(&lines[i], "[level]", "[/level]", reading_level);
                }
            }
            i = lines.len() + 1;
        }
        let mut vec: Vec<Box<GameObjectTrait>> = Vec::new();
        vec.push(Box::new(Background::new(Pointf { x: 0.0, y: 0.0 }, &self.texture_creator)));
        vec.push(Box::new(Player::new(
                Pointf { x: player.1, y: player.2 },
                player.0,
                &self.texture_creator)
        ));
        for floor in floors {
            let floor_object = Floor::new(
                Pointf { x: floor.1, y: floor.2 },
                floor.0,
                &self.texture_creator,
            );
            vec.push(Box::new(floor_object));
        }
        vec
    }
    fn is_reading_x(&self, line: &String, starts_with: &'static str, ends_with: &'static str, reading_x: bool) -> bool {
        if line.starts_with(starts_with) {
            return true;
        }
        else if line.starts_with(ends_with) {
            return false;
        }
        reading_x
    }
    fn lines_from_file<P>(&self, filename: P) -> Vec<String>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename).expect("no such file");
        let buf = BufReader::new(file);
        buf.lines()
            .map(|l| l.expect("Could not parse line"))
            .collect()
    }
    pub fn load_level_names(&mut self) {
        let mut reading_level = false;
        let lines = self.lines_from_file(Path::new("Assets/levels.txt"));
        let mut i = 0;
        let running = true;
        while i < lines.len() {
            if (lines[i].starts_with("#")) {
                i += 1;
                continue;
            }
            reading_level = self.is_reading_x(&lines[i], "[level]", "[/level]", reading_level);
            let started_reading_level_at = i;
            while reading_level {
                if lines[i].starts_with("name: ") {
                    let mut string = String::new();
                    string.push_str(&lines[i][6..]);
                    self.level_names.push((string, started_reading_level_at as i32));
                }
                if (i >= lines.len()) {
                    reading_level = false;
                } else {
                    i += 1;
                    reading_level = self.is_reading_x(&lines[i], "[level]", "[/level]", reading_level);
                }
            }
            i += 1;
        }
        println!("{:?}", self.level_names);
    }
}