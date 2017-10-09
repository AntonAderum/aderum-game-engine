

use std::collections::HashMap;
use sdl2::keyboard::Keycode;
use game_engine::GameEngine::game_engine::game_engine_manager::GameEngineManager;
use game_engine::GameEngine::game_engine::game_object_trait::GameManagerTrait;

use level_loader::LevelLoader;

pub struct GameManager {}

impl GameManagerTrait for GameManager {
    fn init(&mut self, manager: &mut GameEngineManager) {
        let mut ll = LevelLoader::new(manager.texture_creator);
        ll.load_level_names();
        let level_names = ll.get_level_names();
        let game_objects = ll.get_game_objects_for_level(&level_names[0]);
        for game_object in game_objects {
            manager.add_game_object(game_object);
            //self.obj_vec.push(game_object)
        }
    }
    fn pre_update(
        &mut self,
        _delta_time: &f64,
        _keyboard_input: &HashMap<Keycode, bool>,
        manager: &mut GameEngineManager,
    ) {
    }
    fn post_update(
        &mut self,
        _delta_time: &f64,
        _keyboard_input: &HashMap<Keycode, bool>,
        manager: &mut GameEngineManager,
    ) {
    }
}
