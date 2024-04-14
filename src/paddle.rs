use crate::draw::draw_rectangle;
use piston_window::{types::Color, Context, G2d};

pub struct Paddle {
    position: [f64; 2], //[x,y]
    size: f64,
    player_color: Color,
}

impl Paddle {
    pub fn new(position: [f64;2], size: f64, player_color: Color) -> Self {
        Self {
            position,
            size,
            player_color
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        draw_rectangle(self.player_color, self.position, 1.5, self.size, con, g);
    }

    pub fn get_position(&self) -> [f64;2] {
        self.position
    }

    pub fn set_position(&mut self, new_position: [f64;2]) {
        self.position = new_position;
    }

    fn set_position_y(&mut self, new_y: f64) {
        let new_position = [self.position[0], new_y];
        self.position = new_position;
    }

    pub fn move_up(&mut self, min_y: f64) {
        let new_y = self.position[1] - 0.5;

        if new_y > min_y {
            self.set_position_y(new_y);
        }
    }

    pub fn move_down(&mut self, max_y: f64) {
        let new_y = self.position[1] + 0.5;

        if new_y < max_y {
            self.set_position_y(new_y);
        }
    }

}
