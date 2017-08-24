pub mod game_engine{
    
     #[derive(Debug)]
    pub struct Pointf {
        pub x: f64,
        pub y: f64
    }
    
    #[derive(Debug)]
    pub enum Kinectic {
        _Yes,
        No(Pointf),
    }

    #[derive(Debug)]
    pub enum CollisionTypes {
        BoundingBox(Pointf,Pointf),//middle point of rect, and point is distance in both x and y led from middlepoint
        BoindingCircle(Pointf,f64),// middlepoint of circle, and radius
        None
    }

    #[derive(Debug)]
    pub struct Physics2D {
        pub mass: f64,
        pub use_gravity: bool,
        pub is_kinectic: Kinectic,
        pub collision_type: CollisionTypes
    }

    pub struct Rectf64{
        pub left: f64,
        pub top: f64,
        pub right: f64,
        pub bottom: f64
    }
    impl Rectf64{
        pub fn new(pos: & Pointf, offset: & Pointf, size:& Pointf) -> Rectf64{
                Rectf64{left:pos.x+offset.x -size.x,top:pos.y+offset.y -size.y, right:pos.x + offset.x + size.x, bottom: pos.y+offset.y + size.y}
        }
        
        pub fn intersects(&self, other: &Rectf64) -> bool{
            if !(other.left > self.right || other.right < self.left || other.top > self.bottom || other.bottom < self.top){
                
                return true
            }
            false
        }
    }

    impl Physics2D {

        pub fn update(&mut self, pos : &mut Pointf, delta_time: &f64){
            if let Kinectic::No(ref mut point) = self.is_kinectic{
                    point.y  =  point.y + 9.82 * delta_time;
                    pos.y += point.y*delta_time;
            }
            
        }
        pub fn add_jump_force(&mut self, force: f64){

            if let Kinectic::No(ref mut point) = self.is_kinectic{
                point.y -= force;
            }
        }
        pub fn collision(&mut self, pos: &mut Pointf, other: &mut Physics2D, other_pos: & Pointf) -> bool{

            if let CollisionTypes::BoundingBox(ref offset, ref size) = self.collision_type{
                if let CollisionTypes::BoundingBox(ref  other_offset,ref  other_size) = other.collision_type{
                        let first_rect = Rectf64::new(pos, offset, size);
                        let second_rect =Rectf64::new(other_pos, other_offset, other_size);
                        if first_rect.intersects(&second_rect){
                            if let Kinectic::No(ref mut point) = self.is_kinectic {
                                pos.y -= point.y;
                                point.y *= -0.8;
                            }
                            return true
                                
                        }
                }
            }
            false
        }
        // add code here
    }

    

    #[derive(Debug)]
    pub enum ObjectUsingPhysics {
        Yes(Physics2D),
        _None,
    }

    
}