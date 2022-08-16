use macroquad::prelude::*;

const BALL_RADIUS: f32 = 5.;
const BALL_INIT_SPEED: f32 = 500.;
const BALL_MAX_SPEED: f32 = 1000.;

#[derive(PartialEq)]
pub enum Outside {
    Left,
    Right,
    None
}

pub struct Ball {
    pos_x: f32,
    pos_y: f32,
    vel: f32,
    vel_x: f32,
    vel_y: f32
}

impl Ball {
    pub fn new() -> Ball {
        Ball {
            pos_x: screen_width() / 2.,
            pos_y: screen_height() / 2.,
            vel: BALL_INIT_SPEED,
            vel_x: BALL_INIT_SPEED * (std::f32::consts::PI / 18.).cos(),
            vel_y: BALL_INIT_SPEED * (std::f32::consts::PI / 18.).sin(),
        }
    }

    pub fn update(&mut self) {
        self.pos_x += self.vel_x * get_frame_time();
        self.pos_y += self.vel_y * get_frame_time();
    }

    pub fn is_outside(&self) -> Outside {
        if self.pos_x < 0. {
            Outside::Left
        }
        else if self.pos_x > screen_width() {
            Outside::Right
        }
        else {
            Outside::None
        }
    }

    pub fn speedup(&mut self) {
        if self.vel < BALL_MAX_SPEED {
            self.vel += BALL_INIT_SPEED * 0.00625;
        }
    }

    pub fn reset(&mut self) {
        self.pos_x = screen_width() / 2.;
        self.pos_y = screen_height() / 2.;

        self.vel_y = 0.;
        self.vel_x = self.vel;
    }

    pub fn collision_wall(&self) -> bool {
        if (self.pos_y < 0.) | (self.pos_y > screen_height()) {
            return true;
        }
        return false;
    }

    pub fn bounce_y(&mut self) {
        self.vel_y *= -1.;

        if self.pos_y < 0. {
            self.pos_y = 0.;
        }
        else if self.pos_y > screen_height() {
            self.pos_y = screen_height();
        }
    }

    pub fn bounce_angle(&mut self, angle: f32, paddle_x: f32) {
        self.vel_x = self.vel * angle.cos();
        self.vel_y = self.vel * -angle.sin();

        self.pos_x = paddle_x;
    }

    pub fn pos_x(&self) -> f32 {
        self.pos_x
    }

    pub fn pos_y(&self) -> f32 {
        self.pos_y
    }

    pub fn vel_x(&self) -> f32 {
        self.vel_x
    }

    pub fn draw(&self) {
        draw_circle(self.pos_x, self.pos_y, BALL_RADIUS, WHITE);
    }
}