

pub mod game_engine {

    pub mod pointf;
    pub mod circle;
    pub mod rectf64;
    pub mod hit_info;
    pub mod physics2d;
    use GameEngine::game_engine::circle::Circle;
    use GameEngine::game_engine::pointf::Pointf;
    use GameEngine::game_engine::rectf64::Rectf64;
    use GameEngine::game_engine::hit_info::Hit_Info;
    use GameEngine::game_engine::physics2d::Physics2D;


    pub fn clamp(min: f64, max: f64, value: f64) -> f64 {
        if value < min {
            return min;
        } else if value > max {
            return max;
        }
        value
    }


    #[derive(Debug, Clone)]
    pub enum CollInfoType {
        None,
        Enter,
        Stay,
        Exit,
    }

    #[derive(Debug)]
    pub enum CollisionTypes {
        BoundingBox(Pointf, Pointf), //offset, radius x and y -led
        BoindingCircle(Pointf, f64), // offset, and radius
        None,
    }
    #[derive(Debug)]
    pub struct Material {
        pub bounciness: f64,
        pub friction: f64,
    }

    pub struct HitInfoDetection<'a> {
        hit_info: Hit_Info<'a>,
        hit: bool,
    }


    #[derive(Debug)]
    pub enum ObjectUsingPhysics {
        Yes(Physics2D),
        None,
    }

}
