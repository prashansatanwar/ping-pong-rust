use crate::ball::*;
use crate::draw::*;
use crate::paddle::*;
use piston_window::Glyphs;
use piston_window::Key;
use piston_window:: {
    color::*, types::Color, Context, G2d
};

const PLAYER_A_COLOR: Color = BLUE;
const PLAYER_B_COLOR: Color = TEAL;

const BOARD_COLOR: [f32; 4] = BLACK;
const BORDER_COLOR: [f32; 4] = GRAY;
const GAME_OVER_COLOR: Color = [0.0, 0.6, 0.0, 0.5];

const MARGIN: f64 = 1.0;
const PADDLE_SIZE: f64 = 7.0;

const MAX_SCORE: u8 = 5;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down
}

// #[derive(Clone, Debug)]
pub struct Game {
    player_a: Paddle,
    player_b: Paddle,

    ball: Ball,

    size: [f64; 2],

    game_over: bool,
    restart: bool,

    active_key_a: Option<Key>,
    active_key_b: Option<Key>,

    score_a: u8,
    score_b: u8,

}

impl Game {
    pub fn new(size: [f64;2]) -> Self {
        Self {
            player_a: Paddle::new([size[0] - 3.0 - MARGIN, (size[1]/2.0 - PADDLE_SIZE/2.0)], PADDLE_SIZE, PLAYER_A_COLOR),
            player_b: Paddle::new([3.0, (size[1]/2.0 - PADDLE_SIZE/2.0)], PADDLE_SIZE, PLAYER_B_COLOR),

            ball: Ball::new([size[0]/2.0, size[1]/2.0], [30.0,10.0]),

            size,

            game_over: false,
            restart: false,

            active_key_a: None,
            active_key_b: None,

            score_a: 0,
            score_b: 0,
            
        }
    }

    pub fn key_pressed(&mut self, key: Key) { 
        if self.game_over {

            match key {
                Key::R => self.restart = true,
                _ => return
            }

            
            return;
        }

        match key {
            Key::Down | Key::Up => self.active_key_a = Some(key),
            Key::S | Key::W => self.active_key_b = Some(key),
            _ => return
        }

    }

    pub fn key_released(&mut self) {

        if self.active_key_a.is_some() {
            self.active_key_a = None;
        }

        if self.active_key_b.is_some() {
            self.active_key_b = None;
        }
    
    }

    pub fn draw(&self, con: &Context, g: &mut G2d, cache: &mut Glyphs) {
        let w = self.size[0];
        let h = self.size[1];

        draw_rectangle(BORDER_COLOR, [0.0, 0.0], w, MARGIN, con, g);
        draw_rectangle(BORDER_COLOR, [0.0, 0.0], MARGIN, h, con, g);

        draw_rectangle(BORDER_COLOR, [0.0, h-MARGIN], w, MARGIN, con, g);
        draw_rectangle(BORDER_COLOR, [w-MARGIN, 0.0], MARGIN, h, con, g);

        draw_rectangle(BOARD_COLOR, [MARGIN, MARGIN], w-MARGIN*2.0, h-MARGIN*2.0, con, g);

        if !self.game_over {   
            self.player_a.draw(con, g);
            self.player_b.draw(con, g);
            self.ball.draw(con, g);
        }


        if self.game_over {
            draw_rectangle(GAME_OVER_COLOR, [0.0, 0.0], w, h, con, g);

            if self.score_a == 0 && self.score_b == 0 {
                // do nothing
            }
            else if self.score_a > self.score_b {
                draw_text(format!("PLAYER A WINS").as_str(), [self.size[0]/3.0, self.size[1]/2.0 - 6.0], con, g, cache);
            }
            else {
                draw_text(format!("PLAYER B WINS").as_str(), [self.size[0]/3.0, self.size[1]/2.0 - 6.0], con, g, cache);
            }

            draw_text(format!("GAME OVER").as_str(), [self.size[0]/3.0, self.size[1]/2.0 - 2.0], con, g, cache);
            draw_text(format!("To Restart press R").as_str(), [self.size[0]/3.0, self.size[1]/2.0], con, g, cache);
            draw_text(format!("To Exit press ESC").as_str(), [self.size[0]/3.0, self.size[1]/2.0 + 2.0], con, g, cache);
        }
        draw_text(format!("PLAYER B: {}",self.score_b).as_str(), [3.0, 4.0], con, g, cache);
        draw_text(format!("PLAYER A: {}",self.score_a).as_str(), [self.size[0]/2.0, 4.0], con, g, cache);
    
    }

    pub fn update(&mut self, dt: f64) -> bool {

        if self.game_over {
            if self.restart {
                self.game_over = false;
                self.score_a = 0;
                self.score_b = 0;
                self.restart = false;

                self.ball.set_position([self.size[0]/2.0, self.size[1]/2.0]);
                self.player_a.set_position([self.size[0] - 3.0 - MARGIN, (self.size[1]/2.0 - PADDLE_SIZE/2.0)]);
                self.player_b.set_position([3.0, (self.size[1]/2.0 - PADDLE_SIZE/2.0)]);

            }
        }

        self.update_ball(dt);
        self.update_player_a(self.get_dir());
        self.update_player_b(self.get_dir());

        false
    }
    
    fn get_dir(&self) -> Option<Direction> {

        let mut direction: Option<Direction> = None;

        
        if self.active_key_a.is_some() {
            // println!("player a {:?}", self.active_key_a);
            match self.active_key_a {
                Some(Key::Up) => direction = Some(Direction::Up),
                Some(Key::Down) => direction = Some(Direction::Down),
                _ => direction = None,
            };
        }
        
        if self.active_key_b.is_some() {
            // println!("player b {:?}", self.active_key_b);
            match self.active_key_b {
                Some(Key::W) => direction = Some(Direction::Up),
                Some(Key::S) => direction = Some(Direction::Down),
                _ => direction = None,
            };
        };

        direction
    }

    fn update_player_a(&mut self, dir: Option<Direction>) {
        let min_y = MARGIN;
        let max_y = self.size[1] - MARGIN - PADDLE_SIZE;
        
        if self.active_key_a.is_some() {
            if let Some(direction) = dir {
                match direction {
                    Direction::Up => self.player_a.move_up(min_y),    
                    Direction::Down => self.player_a.move_down(max_y),  
                }
            }
        }
    }

    fn update_player_b(&mut self, dir: Option<Direction>) {
        let min_y = MARGIN;
        let max_y = self.size[1] - MARGIN - PADDLE_SIZE;
        
        
        if self.active_key_b.is_some() {
            if let Some(direction) = dir {
                match direction {
                    Direction::Up => self.player_b.move_up(min_y),    
                    Direction::Down => self.player_b.move_down(max_y),  
                }
            }
        }

    }
    
    fn update_ball(&mut self, dt: f64) {
        let next_position = self.ball.get_new_position(dt);

        if !self.game_over {
            if next_position[0] > self.size[0] - MARGIN {
                self.score_b = self.score_b + 1;
            }
    
            if next_position[0] < MARGIN {
                self.score_a = self.score_a + 1;
            }
        }


        if next_position[0] > self.size[0] - MARGIN || next_position[0] < MARGIN  {
            if self.score_a >= MAX_SCORE || self.score_b >= MAX_SCORE {
                self.game_over = true;
                return;
            }

            self.ball.set_position([self.size[0]/2.0, self.size[1]/2.0]);

            if self.score_a > self.score_b {
                self.ball.set_velocity([30.0,0.0]);
            }
            else {
                self.ball.set_velocity([-30.0,0.0]);
            }

            return;

        } 

        if next_position[1] > self.size[1] - MARGIN*2.0 || next_position[1] < MARGIN {
            self.ball.change_direction_y();
        }

        // Player A collision

        if next_position[0] >= self.player_a.get_position()[0]-1.0 && next_position[1] >= self.player_a.get_position()[1] && next_position[1] <= self.player_a.get_position()[1] + PADDLE_SIZE {
            
            self.ball.increase_velocity_y(-5.0);
            self.ball.change_direction_x()
        }

        // Player B collision 

        if next_position[0] <= self.player_b.get_position()[0]+1.0 && next_position[1] >= self.player_b.get_position()[1] && next_position[1] <= self.player_b.get_position()[1] + PADDLE_SIZE {
            
            self.ball.increase_velocity_y(-5.0);
            self.ball.change_direction_x()
        }


        self.ball.set_position(next_position);

    }

}






