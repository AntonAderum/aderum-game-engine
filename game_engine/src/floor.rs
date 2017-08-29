use game_object::GameObject;
use sdl2::rect::Rect;
use game_engine::GameEngine::game_engine::Pointf;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use game_object::GameObjectTrait;
use game_engine::GameEngine::game_engine::Physics2D;
use game_engine::GameEngine::game_engine::ObjectUsingPhysics;
use game_engine::GameEngine::game_engine::Kinectic;
use game_engine::GameEngine::game_engine::CollisionTypes;

#[derive(Debug)]
pub struct Floor {
    pub game_object: GameObject,
}

impl Floor {
    pub fn new(pos: Pointf, name: String) -> Floor {

        let size = Pointf { x: 1.0, y: 1.0 };
        let physics = Physics2D {
            mass: 1.0,
            use_gravity: false,
            is_kinectic: Kinectic::_Yes,
            collision_type: CollisionTypes::BoundingBox(
                Pointf { x: 64.0, y: 64.0 },
                Pointf { x: 64.0, y: 64.0 },
            ),
            id: name,
        };
        let gam = GameObject {
            position: pos,
            rotation: 0.0,
            size: size,
            speed: 0.0,
            object_using_physics: ObjectUsingPhysics::Yes(physics),
            canjump: true,
            color: Color::RGB(0, 153, 0),
        };
        let floor = Floor { game_object: gam };
        return floor;
    }
}

impl GameObjectTrait for Floor {
    fn draw(&self, rend: &mut Canvas<Window>) {
        // Create centered Rect, draw the outline of the Rect in our dark blue color.
        let border_rect = Rect::new(
            self.game_object.position.x as i32 - 64,
            self.game_object.position.y as i32 - 64,
            128,
            128,
        );
        let _ = rend.draw_rect(border_rect);

        // Create a smaller centered Rect, filling it in the same dark blue.
        let inner_rect = Rect::new(
            self.game_object.position.x as i32 - 60,
            self.game_object.position.y as i32 - 60,
            128,
            128,
        );
        let _ = rend.fill_rect(inner_rect);
    }

    fn collision_enter(&mut self, _other: &GameObject) {}
    fn collision_stay(&mut self, _other: &GameObject) {}
    fn collision_exit(&mut self, _other: &GameObject) {}
    // fn GetObjectUsingPhysics(&mut self) -> ObjectUsingPhysics{
    //     self.game_object.object_using_physics
    // }
    fn get_game_object<'a>(&'a mut self) -> &mut GameObject {
        &mut self.game_object
    }
}
