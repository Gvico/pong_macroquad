#![windows_subsystem = "windows"]   // Disables console

use macroquad::prelude::*;

mod paddle;
mod ball;
mod game;
use game::Game;

/*
    To do :

    - Global sounds_enable

    - Ball reset update?
    - AI paddle speed relative to ball speed?
    - Speeds depending on window size?
*/

#[derive(PartialEq)]
pub enum Gamemode {
    TwoPlayer,
    OnePlayer,
    Hidden,
}

#[macroquad::main("Macroquad Pong")]
async fn main() {
    set_pc_assets_folder("assets");

    let mut game = Game::new(Gamemode::OnePlayer);
    game.sounds_load().await;

    loop {
        clear_background(GRAY);

        game.update();
        game.draw();

        match mode_change(game.gamemode()) {
            Some(new_mode) => {
                game = Game::new(new_mode);
                game.sounds_load().await;
            },
            None => ()
        }

        next_frame().await;
    }
}

fn mode_change(gamemode: &Gamemode) -> Option<Gamemode> {
    if is_key_pressed(KeyCode::Space) {
        match gamemode {
            Gamemode::OnePlayer => Some(Gamemode::TwoPlayer),
            Gamemode::TwoPlayer => Some(Gamemode::Hidden),
            Gamemode::Hidden => Some(Gamemode::OnePlayer)
        }
    }
    else {
        None
    }
}

/*async fn mode_change(gamemode: &Gamemode) -> Option<Gamemode> {
    let new_mode: Option<Gamemode>;

    if is_key_pressed(KeyCode::KpAdd) {
        new_mode = match gamemode {
            Gamemode::OnePlayer => Some(Gamemode::TwoPlayer),
            Gamemode::TwoPlayer => Some(Gamemode::Hidden),
            Gamemode::Hidden => Some(Gamemode::OnePlayer)
        };

        while !(is_key_released(KeyCode::KpAdd)) {
            draw_text("Macroquad Pong", screen_width() / 2. - 305., 100., 100., WHITE);
            draw_text("Macroquad Pong", screen_width() / 2. - 152., screen_height() / 2., 50., WHITE);
            
            next_frame().await;
        }
    }
    else {
        new_mode = None;
    }

    new_mode
}*/