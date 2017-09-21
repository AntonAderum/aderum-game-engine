use GameEngine::game_engine::pointf::Pointf;
extern crate sdl2;
use self::sdl2::rect::Rect;
use self::sdl2::video::Window;
use self::sdl2::render::Canvas;
use self::sdl2::render::Texture as SdlTexture;
use self::sdl2::pixels::Color;
pub struct Camera<'a> {
    offset_x: f64,
    offset_y: f64,
    rend: &'a mut Canvas<Window>,
}

impl<'a> Camera<'a> {
    pub fn new(x: f64, y: f64, rend: &'a mut Canvas<Window>) -> Camera {
        Camera {
            offset_x: x,
            offset_y: y,
            rend: rend,
        }
    }
    pub fn DrawRec(&mut self, dest_rect: &mut Rect) {
        self.rend.draw_rect(Rect::new(
            dest_rect.x + self.offset_x as i32,
            dest_rect.y + self.offset_y as i32,
            dest_rect.width(),
            dest_rect.height(),
        ));
    }
    pub fn DrawPartOfTexture(
        &mut self,
        texture: &SdlTexture,
        src_rect: Rect,
        dest_rect: &mut Rect,
    ) {
        self.rend.copy(
            &texture,
            src_rect,
            Rect::new(
                dest_rect.x + self.offset_x as i32,
                dest_rect.y + self.offset_y as i32,
                dest_rect.width(),
                dest_rect.height(),
            ),
        );
    }
    pub fn DrawFullTexture(&mut self, texture: &SdlTexture, dest_rect: &mut Rect) {
        self.rend.copy(
            &texture,
            None,
            Rect::new(
                dest_rect.x + self.offset_x as i32,
                dest_rect.y + self.offset_y as i32,
                dest_rect.width(),
                dest_rect.height(),
            ),
        );
    }
    pub fn DrawBackground(&mut self, texture: &SdlTexture) {
        self.rend.copy(&texture, None, None);
    }

    pub fn Present(&mut self) {
        self.rend.present();
    }

    pub fn Clear(&mut self) {
        //  Set the drawing color to a light blue.
        let _ = self.rend.set_draw_color(Color::RGB(101, 208, 246));

        // // Clear the buffer, using the light blue color set above.
        let _ = self.rend.clear();

        // // Set the drawing color to a darker blue.
        let _ = self.rend.set_draw_color(Color::RGB(0, 153, 204));
    }
    pub fn SetOffset(&mut self, x: f64, y: f64) {
        let (sizex, sizey) = self.rend.window().size();
        let new_offset_x = (sizex / 2) as f64 - x;
        let new_offset_y = (sizey / 2) as f64 - y;
        let min_diff_x = 150.0;
        let min_diff_y = 15.0;
        if new_offset_x > self.offset_x && new_offset_x - self.offset_x > min_diff_x {
            self.offset_x = new_offset_x - min_diff_x;
        } else if self.offset_x > new_offset_x && self.offset_x - new_offset_x > min_diff_x {
            self.offset_x = new_offset_x + min_diff_x;
        }
        if new_offset_y > self.offset_y && new_offset_y - self.offset_y > min_diff_y {
            self.offset_y = new_offset_y - min_diff_y;
        } else if self.offset_y > new_offset_y && self.offset_y - new_offset_y > min_diff_y {
            self.offset_y = new_offset_y + min_diff_y;
        }
    }
}
