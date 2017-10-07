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
    fn get_string(&self, line: &String, key: &str) -> String {
        let mut string = String::new();
        string = string.add(&line[key.len()..]);
        string
    }
    fn get_f64(&self, line: &String, key: &str) -> f64 {
        let mut string = String::new();
        string = string.add(&line[key.len()..]);
        let _f64 = string.parse::<f64>().unwrap();
        _f64
    }
    fn read_player(&self, player: &mut (String, f64, f64), line: &String) {
        if line.starts_with("id: ") {
            player.0 = self.get_string(&line, "id: ");
        } else if (line.starts_with("pos_x: ")) {
            player.1 = self.get_f64(&line, "pos_x: ");
        } else if (line.starts_with("pos_y: ")) {
            player.2 = self.get_f64(&line, "pos_y: ");
        }
    }
    fn read_floor(&self, floor: &mut (String, f64, f64), line: &String) {
        if line.starts_with("id: ") {
            floor.0 = self.get_string(&line, "id: ");
        } else if (line.starts_with("pos_x: ")) {
            floor.1 = self.get_f64(&line, "pos_x: ");
        } else if (line.starts_with("pos_y: ")) {
            floor.2 = self.get_f64(&line, "pos_y: ");
        }
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
                    self.read_player(&mut player, &lines[i]);
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
                    self.read_floor(&mut floor, &lines[i]);
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