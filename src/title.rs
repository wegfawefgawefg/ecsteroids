use glam::Vec2;
pub use legion::*;
use raylib::prelude::*;

use crate::{
    state::{GameMode, State},
    DIMS,
};

pub struct TitleInputs {
    pub confirm: bool,
}

pub fn process_events_and_input(rl: &mut RaylibHandle, state: &mut State) {
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ESCAPE) {
        state.running = false;
    }
    let mut title_inputs = TitleInputs { confirm: false };
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE) {
        title_inputs.confirm = true;

        if let Some(mut transition_to) = state.resources.get_mut::<Option<GameMode>>() {
            *transition_to = Option::Some(GameMode::Playing);
        }
    }

    state.resources.insert(title_inputs);
}

pub fn step(_rl: &mut RaylibHandle, state: &mut State) {
    state
        .title_schedule
        .execute(&mut state.ecs, &mut state.resources);
}

pub fn draw(_state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
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
