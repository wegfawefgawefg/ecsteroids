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
        state.game_mode = GameMode::Title;

        // // add a player
        // state.ecs.push((
        //     CTransform {
        //         pos: Vec2::new(100.0, 100.0),
        //         rot: Vec2::new(0.0, 1.0),
        //     },
        //     Physics {
        //         vel: Vec2::new(1.0, 1.0),
        //         rot_vel: 30.0,
        //     },
        //     InputControlled,
        //     Player,
        //     Gun {
        //         wants_to_shoot: false,
        //         fire_delay: 10,
        //         cooldown: 0,
        //     },
        // ));
    }
}

pub fn step(rl: &mut RaylibHandle, rlt: &mut RaylibThread, state: &mut State) {}

pub fn draw(state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    // draw the title screen
    // name is ecsstroids

    let mut cursor = Vec2::new(DIMS.x as f32 * 0.28, DIMS.y as f32 * 0.4);
    let title = "GAME OVER!";
    let size = 20;
    d.draw_text(title, cursor.x as i32, cursor.y as i32, size, Color::WHITE);
    cursor.y += size as f32 * 1.5;

    let subtitle = "we have to go back";
    let size = 1;
    d.draw_text(
        subtitle,
        cursor.x as i32,
        cursor.y as i32,
        size,
        Color::WHITE,
    );
}
