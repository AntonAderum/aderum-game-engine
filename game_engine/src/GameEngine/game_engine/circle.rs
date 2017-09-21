use GameEngine::game_engine::pointf::Pointf;
use GameEngine::game_engine::Rectf64;

pub struct Circle {
    pub middle_point: Pointf,
    pub radius: f64,
}
impl Circle {
    pub fn new(pos: &Pointf, offset: &Pointf, radius: f64) -> Circle {
        Circle {
            middle_point: Pointf {
                x: pos.x + offset.x,
                y: pos.y + offset.y,
            },
            radius: radius,
        }
    }
    pub fn intersects_unoptimized(&self, other: &Circle) -> bool {
        let r = self.radius + other.radius;
        r < self.middle_point.distance(&other.middle_point)
    }

    pub fn intersects_optimized(&self, other: &Circle) -> bool {
        let mut r = self.radius + other.radius;
        r *= r;
        r <
            (self.middle_point.x + other.middle_point.x).powf(2.0) +
                (self.middle_point.y + other.middle_point.y).powf(2.0)
    }
    pub fn intersects_rect(&self, other: &Rectf64) -> bool {
        let half_width = other.width / 2.0;
        let half_height = other.height / 2.0;
        let cx = (self.middle_point.x - other.left - half_width).abs();
        let x_dist = half_width + self.radius;

        if cx > x_dist {
            return false;
        }
        let cy = (self.middle_point.y - other.top - half_height).abs();
        let y_dist = half_height + self.radius;
        if cy > y_dist {
            return false;
        }
        if cx <= half_width || cy <= half_height {
            return true;
        }
        let x_corner_dist = cx - half_width;
        let y_corner_dist = cy - half_height;
        let x_corner_dist_sq = x_corner_dist * x_corner_dist;
        let y_corner_dist_sq = y_corner_dist * y_corner_dist;
        let max_corner_dist_sq = self.radius * self.radius;
        x_corner_dist_sq + y_corner_dist_sq <= max_corner_dist_sq
    }
}
