use GameEngine::game_engine::pointf::Pointf;
use GameEngine::game_engine::Physics2D;


pub struct Hit_Info<'a> {
    pub obj_a: &'a mut Physics2D,
    pub obj_b: &'a mut Physics2D,
    pub penetration: f64,
    pub normal: Pointf,
}

impl<'a> Hit_Info<'a> {
    pub fn resolve_collision(&mut self, pos: &mut Pointf, other_pos: &mut Pointf) {
        let rv = Pointf {
            x: self.obj_b.velocity.x - self.obj_a.velocity.x,
            y: self.obj_b.velocity.y - self.obj_a.velocity.y,
        };
        // // Calculate relative velocity in terms of the normal direction
        let velAlongNormal = rv.dot_product(&self.normal); // DotProduct( rv, normal )
        println!("normal {}", self.normal.y);
        // // Do not resolve if velocities are separating
        if velAlongNormal > 0.0 {
            println!("No Coll");
            return;
        }



        // // Calculate restitution
        let e = f64::min(
            self.obj_a.material.bounciness,
            self.obj_b.material.bounciness,
        );

        // // Calculate impulse scalar
        let mut j = -(1.0 + e) * velAlongNormal;
        j /= ((1.0 / self.obj_a.mass) + (1.0 / self.obj_b.mass));

        // // Apply impulse
        let impulse = Pointf {
            x: j * self.normal.x,
            y: j * self.normal.y,
        };
        println!("impulse {} '    '  {}", impulse.x, impulse.y);
        if !self.obj_a.is_kinectic {
            self.obj_a.velocity.x -= 1.0 / self.obj_a.mass * impulse.x;
            self.obj_a.velocity.y -= 1.0 / self.obj_a.mass * impulse.y;
        }
        if !self.obj_b.is_kinectic {
            self.obj_b.velocity.x += 1.0 / self.obj_b.mass * impulse.x;
            self.obj_b.velocity.y += 1.0 / self.obj_b.mass * impulse.y;
        }
        self.position_correction(pos, other_pos);
    }

    fn position_correction(&mut self, pos: &mut Pointf, other_pos: &mut Pointf) {
        let inv_mas = 1.0 / self.obj_a.mass;
        let other_inv_mass = 1.0 / self.obj_b.mass;
        let percent = 0.2; // usually 20% to 80%
        let slop = 0.01; // usually 0.01 to 0.1
        let val = f64::max(self.penetration - slop, 0.0) / (inv_mas + other_inv_mass) * percent;
        let correction = Pointf {
            x: val * self.normal.x,
            y: val * self.normal.y,
        };
        if !self.obj_a.is_kinectic {
            pos.x -= inv_mas * correction.x;
            pos.y -= inv_mas * correction.y;
        }
        if !self.obj_b.is_kinectic {
            other_pos.x += other_inv_mass * correction.x;
            other_pos.y += other_inv_mass * correction.y;
        }
    }
}
