use macroquad::{audio, prelude::*};

use crate::Gamemode;
use crate::paddle::{Paddle, self, PADDLE_WIDTH, PADDLE_HEIGHT};
use crate::ball::{Ball, self};

const MAX_ANGLE_COEFF: f32 = std::f32::consts::PI / 150.;   // Max angle = 60 degrees

pub struct Game {
    ball: Ball,
    player: Paddle,
    opponent: Paddle,
    score_player: u8,
    score_opponent: u8,
    game_mode: Gamemode,
    sounds_enable: bool,
    sound_ping: Option<audio::Sound>,
    sound_pong: Option<audio::Sound>,
    sound_point: Option<audio::Sound>,
}

impl Game {
    pub fn new(game_mode: Gamemode) -> Game {
        Game {
            ball: Ball::new(),
            player: Paddle::new(paddle::Side::Left),
            opponent: Paddle::new(paddle::Side::Right),
            score_player: 0,
            score_opponent: 0,
            game_mode: game_mode,
            sounds_enable: true,
            sound_ping: None,
            sound_pong: None,
            sound_point: None,
        }
    }

    pub async fn sounds_load(&mut self) {
        self.sound_ping = match audio::load_sound("ping.ogg").await {
            Ok(sound) => Some(sound),
            _ => None,
        };
        self.sound_pong = match audio::load_sound("pong.ogg").await {
            Ok(sound) => Some(sound),
            _ => None,
        };
        self.sound_point = match audio::load_sound("point.ogg").await {
            Ok(sound) => Some(sound),
            _ => None,
        }
    }

    fn sounds_toggle(&mut self) {
        if is_key_pressed(KeyCode::S) {
            self.sounds_enable = !self.sounds_enable;
        }
    }
    
    fn sound_play(&self, sound: &Option<audio::Sound>) {
        if self.sounds_enable {
            match sound {
                Some(sound) => audio::play_sound_once(*sound),
                _ => (),
            }
        }
    }

    fn collision(ball: &Ball, paddle: &Paddle) -> bool {
        if ball.pos_y() > paddle.pos_y() && ball.pos_y() < paddle.pos_y() + PADDLE_HEIGHT {
            if paddle.side() == &paddle::Side::Left && ball.pos_x() < paddle.pos_x() + PADDLE_WIDTH {
                return true;
            }
            if paddle.side() == &paddle::Side::Right && ball.pos_x() > paddle.pos_x() {
                return true;
            }
        }
        return false;
    }

    fn score(&mut self) {
        if self.ball.is_outside() == ball::Outside::Left {
            self.score_opponent += 1;
            if self.score_opponent == u8::MAX {
                self.score_opponent = 0;
            }
            self.sound_play(&self.sound_point);
            self.ball.reset();
        }
        else if self.ball.is_outside() == ball::Outside::Right {
            self.score_player += 1;
            if self.score_player == u8::MAX {
                self.score_player = 0;
            }
            self.sound_play(&self.sound_point);
            self.ball.reset();
        }
    }

    pub fn update(&mut self) {
        self.sounds_toggle();

        self.player.update_human();

        if self.game_mode == Gamemode::TwoPlayer {
            self.opponent.update_human();
        }
        else if self.game_mode == Gamemode::OnePlayer || self.game_mode == Gamemode::Hidden {
            self.opponent.update_ai(self.ball.pos_x(), self.ball.pos_y(), self.ball.vel_x());
        }

        self.ball.update();

        if self.ball.collision_wall() {
            self.ball.bounce_y();
            self.sound_play(&self.sound_pong);
        }
        if Self::collision(&self.ball, &self.player) {
            self.ball.bounce_angle(MAX_ANGLE_COEFF * self.player.delta_y(self.ball.pos_y()), self.player.pos_x() + PADDLE_WIDTH);
            self.ball.speedup();
            self.sound_play(&self.sound_ping);
        }
        else if Self::collision(&self.ball, &self.opponent) {
            self.ball.bounce_angle(std::f32::consts::PI + MAX_ANGLE_COEFF * -self.opponent.delta_y(self.ball.pos_y()), self.opponent.pos_x());
            self.ball.speedup();
            self.sound_play(&self.sound_ping);
        }

        self.score();
    }

    pub fn draw(&self) {
        self.player.draw();
        self.opponent.draw();
        self.ball.draw();

        if self.game_mode == Gamemode::Hidden {
            draw_rectangle(screen_width() / 2., 0., screen_width() / 2., screen_height(), DARKGRAY);
        }

        draw_line(screen_width() / 2., 0., screen_width() / 2., screen_height(), 2., WHITE);

        draw_text(&self.score_player.to_string(), screen_width() / 6., screen_height() / 6., 50., WHITE);
        draw_text(&self.score_opponent.to_string(), 5. * screen_width() / 6., screen_height() / 6., 50., WHITE);
    }

    pub fn gamemode(&self) -> &Gamemode {
        &self.game_mode
    }
}