use glam::Vec2;
pub use legion::*;
use raylib::prelude::*;

use crate::{
    components::{CTransform, Gun, InputControlled, Physics, Player},
    state::{GameMode, State},
    DIMS,
};

pub fn process_events_and_input(rl: &mut RaylibHandle, state: &mut State) {
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ESCAPE) {
        state.running = false;
    }

    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE) {
        state.game_mode = GameMode::Playing;
    }
}

pub fn step(rl: &mut RaylibHandle, rlt: &mut RaylibThread, state: &mut State) {}

pub fn draw(state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    // draw the title screen
    // name is ecsstroids

    let mut cursor = Vec2::new(DIMS.x as f32 * 0.28, DIMS.y as f32 * 0.4);
    let title = "ECStroids!";
    let size = 20;
    d.draw_text(title, cursor.x as i32, cursor.y as i32, size, Color::WHITE);
    cursor.y += size as f32 * 1.5;

    let subtitle = "press space to start";
    let size = 1;
    d.draw_text(
        subtitle,
        cursor.x as i32,
        cursor.y as i32,
        size,
        Color::WHITE,
    );
}
