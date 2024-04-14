use crate::draw::draw_block;
use piston_window::{color::*, types::Color, Context, G2d};

const BALL_COLOR: Color = WHITE;

pub struct Ball {
    position: [f64; 2],
    velocity: [f64; 2]
}

impl Ball {
    pub fn new(position: [f64;2], velocity: [f64;2]) -> Self {
        Self {
            position,
            velocity
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        draw_block(BALL_COLOR, self.position, con, g)
    }

    pub fn set_position(&mut self, new_position: [f64;2]) {
        self.position = new_position
    }

    pub fn set_velocity(&mut self, new_velocity: [f64;2]) {
        self.velocity = new_velocity
    }

    pub fn increase_velocity_y(&mut self, factor: f64) {
        self.velocity[1] += factor
    }

    pub fn change_direction_x(&mut self) {
        self.velocity[0] = self.velocity[0]*-1.0
    }

    pub fn change_direction_y(&mut self) {
        self.velocity[1] = self.velocity[1]*-1.0
    }

    pub fn get_new_position(&self, time: f64) -> [f64;2] {
        let dist_x = self.velocity[0] * time;
        let dist_y = self.velocity[1] * time;

        let new_x = self.position[0] + dist_x;
        let new_y = self.position[1] + dist_y;

        [new_x, new_y]
    } 

}