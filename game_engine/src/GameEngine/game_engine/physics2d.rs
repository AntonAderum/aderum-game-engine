use GameEngine::game_engine::pointf::Pointf;
use GameEngine::game_engine::CollisionTypes;
use GameEngine::game_engine::Material;
use GameEngine::game_engine::hit_info::Hit_Info;
use GameEngine::game_engine::rectf64::Rectf64;
use GameEngine::game_engine::clamp;
use GameEngine::game_engine::HitInfoDetection;
use GameEngine::game_engine::circle::Circle;
use GameEngine::game_engine::CollInfoType;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

#[derive(Debug)]
pub struct Physics2D {
    pub mass: f64,
    pub use_gravity: bool,
    pub velocity: Pointf,
    pub is_kinectic: bool,
    pub collision_type: CollisionTypes,
    pub material: Material,
    pub id: String,
}

enum CollisionTypesOfTypes{
    BoxAndBox,
    BoxAndCircle,
    CircleAndCircle,
    CircleAndBox,
    None
}

impl Physics2D {
    pub fn update(&mut self, pos: &mut Pointf, delta_time: &f64) {
        if !self.is_kinectic {
            self.velocity.y += 9.82 * delta_time;
            pos.y += self.velocity.y;
            let b = self.velocity.x < 0.0;
            if self.velocity.x.abs() > 200.0 * delta_time {
                let newv = self.velocity.x.abs() - 200.0 * delta_time;
                if b {
                    self.velocity.x = -newv;
                } else {
                    self.velocity.x = newv;
                }
            } else {
                self.velocity.x = 0.0;
            }
            pos.x += self.velocity.x;
        }

    }
    pub fn add_jump_force(&mut self, force: f64) {

        if !self.is_kinectic {
            self.velocity.y -= force;
        }
    }

    pub fn add_side_force(&mut self, force: f64, max_force: f64) {

        if !self.is_kinectic {
            if self.velocity.x.abs() < max_force {
                self.velocity.x += force;
            }
        }
    }

    fn remove_velocity(&mut self) {
        if !self.is_kinectic {

            self.velocity.y = 0.0;
        }
    }
    fn add_normal_force(&mut self, pos: &mut Pointf, delta_time: &f64) {
        if !self.is_kinectic {
            pos.y -= self.velocity.y;
            self.velocity.y -= 9.82 * delta_time;
        }
    }

    fn circle_circle_hit_info<'a>(
        &'a mut self,
        pos: &mut Pointf,
        radius: &f64,
        other: &'a mut Physics2D,
        other_pos: &mut Pointf,
        other_radius: &f64,
    ) -> Hit_Info {

        let n = Pointf {
            x: other_pos.x - pos.x,
            y: other_pos.y - pos.y,
        };

        let mut r = radius + other_radius;
        r *= r;


        let d = n.length();

        if d != 0.0 {
            return Hit_Info {
                obj_a: self,
                obj_b: other,
                penetration: r - d,
                normal: Pointf {
                    x: n.x / d,
                    y: n.y / d,
                },
            };
        }

        Hit_Info {
            obj_a: self,
            obj_b: other,
            penetration: *radius,
            normal: Pointf { x: 1.0, y: 0.0 },
        }
    }

    fn box_box_hit_info<'a>(
        &'a mut self,
        pos: &mut Pointf,
        rect: &Rectf64,
        other: &'a mut Physics2D,
        other_pos: &mut Pointf,
        other_rect: &Rectf64,
    ) -> Hit_Info {


        let n = Pointf {
            x: other_pos.x - pos.x,
            y: other_pos.y - pos.y,
        };

        let a_extent = rect.width / 2.0;
        let b_extent = other_rect.width / 2.0;

        let x_overlap = a_extent + b_extent - n.x.abs();

        if x_overlap > 0.0 {

            let a_extent = rect.height / 2.0;
            let b_extent = other_rect.height / 2.0;

            let y_overlap = a_extent + b_extent - n.y.abs();

            if y_overlap > 0.0 {
                if x_overlap < y_overlap {
                    let normal: Pointf;
                    if n.x < 0.0 {
                        normal = Pointf { x: -1.0, y: 0.0 };
                    } else {
                        normal = Pointf { x: 1.0, y: 0.0 };
                    }
                    return Hit_Info {
                        obj_a: self,
                        obj_b: other,
                        normal: normal,
                        penetration: x_overlap,
                    };
                } else {
                    let normal: Pointf;
                    if n.y < 0.0 {
                        normal = Pointf { x: 0.0, y: -1.0 };
                    } else {
                        normal = Pointf { x: 0.0, y: 1.0 };
                    }
                    return Hit_Info {
                        obj_a: self,
                        obj_b: other,
                        normal: normal,
                        penetration: y_overlap,
                    };
                }
            }
        }
        Hit_Info {
            obj_a: self,
            obj_b: other,
            normal: Pointf { x: 0.0, y: 0.0 },
            penetration: 0.0,
        }
    }

    fn box_circle_hit_info<'a>(
        &'a mut self,
        pos: &mut Pointf,
        rect: &Rectf64,
        other: &'a mut Physics2D,
        other_pos: &mut Pointf,
        other_radius: &f64,
    ) -> Hit_Info {

        let n = Pointf {
            x: other_pos.x - pos.x,
            y: other_pos.y - pos.y,
        };

        let mut closest = Pointf { x: n.x, y: n.y };

        let x_extent = rect.width / 2.0;
        let y_extent = rect.height / 2.0;

        closest.x = clamp(-x_extent, x_extent, closest.x);
        closest.y = clamp(-y_extent, y_extent, closest.y);

        let mut inside = false;

        if n == closest {
            inside = true;

            if n.x.abs() > n.y.abs() {
                if closest.x > 0.0 {
                    closest.x = x_extent
                } else {
                    closest.x = -x_extent
                }
            }
            else {
                if closest.y > 0.0 {
                    closest.y = y_extent
                } else {
                    closest.y = -y_extent
                }
            }
        }

        let normal = Pointf {
            x: n.x - closest.x,
            y: n.y - closest.y,
        };
        let mut d = normal.length().powf(2.0);
        let r = other_radius;

        if d > r * r && !inside {
            return Hit_Info {
                obj_a: self,
                obj_b: other,
                penetration: 0.0,
                normal: Pointf { x: 0.0, y: 0.0 },
            };
        }

        d = d.sqrt();

        if inside {
            return Hit_Info {
                obj_a: self,
                obj_b: other,
                penetration: r - d,
                normal: Pointf { x: -n.x, y: -n.y },
            };
        } else {
            return Hit_Info {
                obj_a: self,
                obj_b: other,
                penetration: r - d,
                normal: n,
            };
        }

    }

    fn check_collision<'a>(
        &'a mut self,
        pos: &mut Pointf,
        other: &'a mut Physics2D,
        other_pos: &mut Pointf,
    ) -> HitInfoDetection {
        let mut first_rect = Rectf64::new_empty();
        let mut second_rect = Rectf64::new_empty();
        let mut first_circle_size = 0.0;
        let mut second_cirlcle_size = 0.0;
        let mut collision_type = CollisionTypesOfTypes::None;
        match self.collision_type {

            CollisionTypes::BoundingBox(ref offset, ref size) => {
                match other.collision_type {
                    CollisionTypes::BoundingBox(ref other_offset, ref other_size) => {
                        first_rect = Rectf64::new(pos, offset, size);
                        second_rect = Rectf64::new(other_pos, other_offset, other_size);
                        if first_rect.intersects(&second_rect) {
                            collision_type = CollisionTypesOfTypes::BoxAndBox;
                        }
                    }
                    CollisionTypes::BoindingCircle(ref other_offset, ref other_size) => {
                        first_rect = Rectf64::new(pos, offset, size);
                        let second_circle = Circle::new(other_pos, other_offset, *other_size);

                        second_cirlcle_size = *other_size;
                        if first_rect.intersects_circle(&second_circle) {
                            collision_type = CollisionTypesOfTypes::BoxAndCircle;
                        }
                    }
                    CollisionTypes::None => (),
                }
            }
            CollisionTypes::BoindingCircle(ref offset, ref size) => {
                match other.collision_type {
                    CollisionTypes::BoundingBox(ref other_offset, ref other_size) => {
                        let first_circle = Circle::new(pos, offset, *size);
                        second_rect = Rectf64::new(other_pos, other_offset, other_size);
                        first_circle_size = *size;
                        if first_circle.intersects_rect(&second_rect) {
                            collision_type = CollisionTypesOfTypes::CircleAndBox;

                        }
                    }
                    CollisionTypes::BoindingCircle(ref other_offset, ref other_size) => {
                        let first_circle = Circle::new(pos, offset, *size);
                        let second_circle = Circle::new(other_pos, other_offset, *other_size);
                        first_circle_size = *size;
                        second_cirlcle_size = *other_size;
                        if first_circle.intersects_optimized(&second_circle) {
                            collision_type = CollisionTypesOfTypes::CircleAndCircle;
                        }
                    }
                    CollisionTypes::None => (),
                }
            }
            CollisionTypes::None => (),
        }
        match collision_type {
            CollisionTypesOfTypes::BoxAndBox => {
                let hitinfo =
                    self.box_box_hit_info(pos, &first_rect, other, other_pos, &second_rect);
                return HitInfoDetection {
                    hit_info: hitinfo,
                    hit: true,
                };
            }
            CollisionTypesOfTypes::BoxAndCircle => {
                let hitinfo = self.box_circle_hit_info(
                    pos,
                    &first_rect,
                    other,
                    other_pos,
                    &second_cirlcle_size,
                );
                return HitInfoDetection {
                    hit_info: hitinfo,
                    hit: true,
                };
            }
            CollisionTypesOfTypes::CircleAndBox => {
                let hitinfo = other.box_circle_hit_info(
                    other_pos,
                    &second_rect,
                    self,
                    pos,
                    &first_circle_size,
                );
                return HitInfoDetection {
                    hit_info: hitinfo,
                    hit: true,
                };
            }
            CollisionTypesOfTypes::CircleAndCircle => {
                let hitinfo = self.circle_circle_hit_info(
                    pos,
                    &first_circle_size,
                    other,
                    other_pos,
                    &second_cirlcle_size,
                );
                return HitInfoDetection {
                    hit_info: hitinfo,
                    hit: true,
                };
            }
            _ => (),
        }
        HitInfoDetection {
            hit_info: Hit_Info {
                obj_a: self,
                obj_b: other,
                penetration: 0.0,
                normal: Pointf { x: 0.0, y: 0.0 },
            },
            hit: false,
        }
    }

    pub fn collision(
        &mut self,
        pos: &mut Pointf,
        other: &mut Physics2D,
        other_pos: &mut Pointf,
        coll_info: &mut HashMap<String, CollInfoType>
    ) -> CollInfoType {

        let mut hit_info_detection_obj = self.check_collision(pos, other, other_pos);

        let mut val2 = CollInfoType::None;
        if hit_info_detection_obj.hit {
            let v = &hit_info_detection_obj.hit_info.obj_b.id;
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
            let v = &hit_info_detection_obj.hit_info.obj_b.id;
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
            CollInfoType::Enter | CollInfoType::Stay => {
                hit_info_detection_obj.hit_info.resolve_collision(
                    pos,
                    other_pos,
                );
            }
            CollInfoType::Exit => (),
            _ => (),
        }
        val2

    }
}
