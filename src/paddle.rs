use macroquad::prelude::*;

pub const PADDLE_WIDTH: f32 = 10.;
pub const PADDLE_HEIGHT: f32 = 100.;
const PADDLE_PADDING: f32 = 20.;
const PADDLE_HUMAN_SPEED: f32 = 500.;
const PADDLE_MAX_SPEED: f32 = 600.;

#[derive(PartialEq)]
pub enum Side {
    Left,
    Right
}

pub struct Paddle {
    pos_x: f32,
    pos_y: f32,
    vel_y: f32,
    side: Side,
}

impl Paddle {
    pub fn new(side: Side) -> Paddle {
        Paddle {
            pos_x: match side {
                Side::Left => PADDLE_PADDING,
                Side::Right => screen_width() - PADDLE_WIDTH - PADDLE_PADDING,
            },
            pos_y: screen_height() / 2. - PADDLE_HEIGHT / 2.,
            vel_y: 0.,
            side,
        }
    }

    pub fn update_human(&mut self) {
        if (is_key_down(match self.side {Side::Left => KeyCode::Up, Side::Right => KeyCode::Kp8})) & (self.pos_y > 0.) {
            self.pos_y -= PADDLE_HUMAN_SPEED * get_frame_time();
         }
        else if (is_key_down(match self.side {Side::Left => KeyCode::Down, Side::Right => KeyCode::Kp2})) & (self.pos_y < screen_height() - PADDLE_HEIGHT) {
            self.pos_y += PADDLE_HUMAN_SPEED * get_frame_time();
        }

        if self.side == Side::Right {
            self.pos_x = screen_width() - PADDLE_WIDTH - PADDLE_PADDING;
        }
    }

    pub fn update_ai(&mut self, ball_x: f32, ball_y: f32, ball_vel_x: f32) {
        if ball_vel_x > 0. && ball_x > screen_width() / 2. {
            if ball_y != self.pos_y + PADDLE_HEIGHT / 2. {
                let time_left = (self.pos_x - ball_x) / ball_vel_x;
                let dist_wanted = self.pos_y + PADDLE_HEIGHT / 2. - ball_y;
                let vel_wanted = -dist_wanted / time_left;

                if vel_wanted > PADDLE_MAX_SPEED {
                    self.vel_y = PADDLE_MAX_SPEED;
                }
                else if vel_wanted < -PADDLE_MAX_SPEED {
                    self.vel_y = -PADDLE_MAX_SPEED;
                }
                else {
                    self.vel_y = vel_wanted;
                }
            }
            else {
                self.vel_y = 0.;
            }
        }
        else {
            self.vel_y = 0.;
        }
        
        self.pos_y += self.vel_y * get_frame_time();

        if self.pos_y < 0. {
            self.pos_y = 0.;
        }
        else if self.pos_y + PADDLE_HEIGHT > screen_height() {
            self.pos_y = screen_height() - PADDLE_HEIGHT;
        }
    }

    pub fn delta_y(&self, ball_y: f32) -> f32 {
        self.pos_y + PADDLE_HEIGHT / 2. - ball_y
    }

    pub fn pos_x(&self) -> f32 {
        self.pos_x
    }

    pub fn pos_y(&self) -> f32 {
        self.pos_y
    }

    pub fn side(&self) -> &Side {
        &self.side
    }

    pub fn draw(&self) {
        draw_rectangle(self.pos_x, self.pos_y, PADDLE_WIDTH, PADDLE_HEIGHT, WHITE);
    }
}