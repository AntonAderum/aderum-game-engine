use GameEngine::game_engine::pointf::Pointf;
use GameEngine::game_engine::circle::Circle;

pub struct Rectf64 {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub width: f64,
    pub height: f64,
}
impl Rectf64 {
    pub fn new(pos: &Pointf, offset: &Pointf, size: &Pointf) -> Rectf64 {
        let left = pos.x + offset.x - size.x;
        let top = pos.y + offset.y - size.y;
        let right = pos.x + offset.x + size.x;
        let bottom = pos.y + offset.y + size.y;
        Rectf64 {
            left: left,
            top: top,
            right: right,
            bottom: bottom,
            width: right - left,
            height: bottom - top,
        }
    }
    pub fn new_empty() -> Rectf64 {
        Rectf64 {
            left: 0.0,
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            width: 0.0,
            height: 0.0,
        }
    }

    pub fn intersects(&self, other: &Rectf64) -> bool {
        !(other.left > self.right || other.right < self.left || other.top > self.bottom ||
              other.bottom < self.top)
    }
    pub fn intersects_circle(&self, other: &Circle) -> bool {

        let half_width = self.width / 2.0;
        let half_height = self.height / 2.0;
        let cx = (other.middle_point.x - self.left - half_width).abs();
        let x_dist = half_width + other.radius;

        if cx > x_dist {
            return false;
        }
        let cy = (other.middle_point.y - self.top - half_height).abs();
        let y_dist = half_height + other.radius;
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
        let max_corner_dist_sq = other.radius * other.radius;
        x_corner_dist_sq + y_corner_dist_sq <= max_corner_dist_sq

    }
}
