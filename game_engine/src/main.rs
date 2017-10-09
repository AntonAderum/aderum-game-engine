extern crate sdl2;
extern crate game_engine;
mod game_manager;
mod background;
mod floor;
mod player;
mod level_loader;

use game_engine::GameEngine::game_engine::game_engine_manager::GameEngineManager;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashMap;

use game_engine::GameEngine::game_engine::camera::Camera;
pub fn main() {
    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 1280, 880)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
    let mut camera = Camera::new(0.0, 0.0, &mut canvas);
    let mut game_manager = game_manager::GameManager {};
    let mut game_engine_manager = GameEngineManager::new(&texture_creator, &mut camera);
    game_engine_manager.init(&mut game_manager);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut keyboard_state: HashMap<Keycode, bool> = HashMap::new();
    let mut coll_info: HashMap<String, game_engine::GameEngine::game_engine::CollInfoType> =
        HashMap::new();
    let mut del: f64 = 0.0;
    let mut old_time = std::time::Instant::now();
    'running: loop {

        if handle_events(&mut event_pump, &mut keyboard_state) {
            break 'running;
        }
        game_engine_manager.update(&del, &keyboard_state, &mut coll_info, &mut game_manager);
        game_engine_manager.draw();

        let new_time = std::time::Instant::now();
        let nanoseconds = (new_time - old_time).subsec_nanos();
        let ticks_per_second = (1_000_000_000u32 / nanoseconds) as f64;
        del = 0.1 / ticks_per_second; /////FAAAAAAIL HERE
        //println!("del{}", ticks_per_second);
        old_time = new_time;

    }
}

fn handle_events(
    events: &mut sdl2::EventPump,
    keyboard_state: &mut HashMap<sdl2::keyboard::Keycode, bool>,
) -> bool {
    for item in events.poll_iter() {
        match item {
            Event::Quit { .. } => return true,
            Event::KeyDown { keycode: Some(keycode), .. } => {
                if keycode == sdl2::keyboard::Keycode::Escape {
                    return true;
                }
                keyboard_state.insert(keycode, true);
            }
            Event::KeyUp { keycode: Some(keycode), .. } => {
                keyboard_state.insert(keycode, false);
            }

            _ => (),
        }
    }
    false
}
