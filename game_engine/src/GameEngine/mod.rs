pub mod game_engine {
    use std::collections::HashMap;
    use std::collections::hash_map::Entry::{Occupied, Vacant};
    #[derive(Debug)]
    pub struct Pointf {
        pub x: f64,
        pub y: f64,
    }

    #[derive(Debug)]
    pub enum Kinectic {
        _Yes,
        No(Pointf),
    }

    #[derive(Debug)]
    pub enum CollisionTypes {
        BoundingBox(Pointf, Pointf), //offset, radius x and y -led
        BoindingCircle(Pointf, f64), // offset, and radius
        None,
    }
    // #[derive(Debug)]
    // pub struct Material{
    //     bounciness: f64,
    //     friction: f64
    // }

    #[derive(Debug)]
    pub struct Physics2D {
        pub mass: f64,
        pub use_gravity: bool,
        pub is_kinectic: Kinectic,
        pub collision_type: CollisionTypes,
        //pub material: Material,
        pub id: String,
    }

    pub struct Rectf64 {
        pub left: f64,
        pub top: f64,
        pub right: f64,
        pub bottom: f64,
    }
    impl Rectf64 {
        pub fn new(pos: &Pointf, offset: &Pointf, size: &Pointf) -> Rectf64 {
            Rectf64 {
                left: pos.x + offset.x - size.x,
                top: pos.y + offset.y - size.y,
                right: pos.x + offset.x + size.x,
                bottom: pos.y + offset.y + size.y,
            }
        }

        pub fn intersects(&self, other: &Rectf64) -> bool {
            if !(other.left > self.right || other.right < self.left || other.top > self.bottom ||
                     other.bottom < self.top)
            {

                return true;
            }
            false
        }
    }
    #[derive(Debug, Clone)]
    pub enum CollInfoType {
        None,
        Enter,
        Stay,
        Exit,
    }
    impl Physics2D {
        pub fn update(&mut self, pos: &mut Pointf, delta_time: &f64) {
            if let Kinectic::No(ref mut point) = self.is_kinectic {
                point.y += 0.82 * delta_time;
                pos.y += point.y;
            }

        }
        pub fn add_jump_force(&mut self, force: f64) {

            if let Kinectic::No(ref mut point) = self.is_kinectic {
                point.y -= force;
            }
        }

        fn remove_velocity(&mut self) {
            if let Kinectic::No(ref mut point) = self.is_kinectic {

                point.y = 0.0;
            }
        }
        fn add_normal_force(&mut self, pos: &mut Pointf, delta_time: &f64) {
            if let Kinectic::No(ref mut point) = self.is_kinectic {
                pos.y -= point.y;
                point.y -= 0.82 * delta_time;
                println!("point.y {}", point.y);
            }
        }

        pub fn collision(
            &mut self,
            pos: &mut Pointf,
            other: &mut Physics2D,
            other_pos: &mut Pointf,
            coll_info: &mut HashMap<String, CollInfoType>,
            delta_time: &f64,
        ) -> CollInfoType {
            let mut col = false;
            if let CollisionTypes::BoundingBox(ref offset, ref size) = self.collision_type {
                if let CollisionTypes::BoundingBox(ref other_offset, ref other_size) =
                    other.collision_type
                {
                    let first_rect = Rectf64::new(pos, offset, size);
                    let second_rect = Rectf64::new(other_pos, other_offset, other_size);
                    if first_rect.intersects(&second_rect) {
                        col = true;


                    }
                }
            }
            let mut val2 = CollInfoType::None;
            if col {
                let v = &other.id;
                match coll_info.entry(v.to_string()) {
                    Vacant(entry) => {

                        val2 = CollInfoType::Enter;
                        entry.insert(CollInfoType::Enter)
                    }
                    Occupied(mut entry) => {
                        match entry.get() {
                            &CollInfoType::Enter |
                            &CollInfoType::Stay => {
                                val2 = CollInfoType::Stay;
                                entry.insert(CollInfoType::Stay);
                            }

                            _ => {
                                val2 = CollInfoType::Enter;
                                entry.insert(CollInfoType::Enter);
                            }
                        }
                        entry.into_mut()
                    }
                };
            } else {
                let v = &other.id;
                match coll_info.entry(v.to_string()) {
                    Vacant(entry) => entry.insert(CollInfoType::None),
                    Occupied(mut entry) => {
                        match entry.get() {
                            &CollInfoType::Enter |
                            &CollInfoType::Stay => {
                                entry.insert(CollInfoType::Exit);
                                val2 = CollInfoType::Exit;
                            }

                            _ => {
                                entry.insert(CollInfoType::None);
                                val2 = CollInfoType::None;
                            }
                        }
                        entry.into_mut()
                    }
                };
            }
            match val2 {
                CollInfoType::Enter => {
                    self.remove_velocity();
                    other.remove_velocity();
                    println!("Enter");
                }
                CollInfoType::Stay => {
                    self.add_normal_force(pos, delta_time);
                    other.add_normal_force(other_pos, delta_time);
                    println!("Stay: ");
                }
                _ => (),
            }
            val2

        }
        // add code here
    }



    #[derive(Debug)]
    pub enum ObjectUsingPhysics {
        Yes(Physics2D),
        _None,
    }


}
