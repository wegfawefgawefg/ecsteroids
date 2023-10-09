use glam::Vec2;
pub use legion::*;
use raylib::prelude::*;

use crate::{
    rendering::{execute_render_command_buffer, RenderCommandBuffer},
    state::State,
};

pub struct PlayingInputs {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub shoot: bool,
}

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

pub fn process_events_and_input(rl: &mut RaylibHandle, state: &mut State) {
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ESCAPE) {
        state.running = false;
    }

    let mut inputs = PlayingInputs {
        left: false,
        right: false,
        up: false,
        down: false,
        shoot: false,
    };
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_LEFT)
        || rl.is_key_down(raylib::consts::KeyboardKey::KEY_A)
    {
        inputs.left = true;
    }
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_RIGHT)
        || rl.is_key_down(raylib::consts::KeyboardKey::KEY_D)
    {
        inputs.right = true;
    }
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_UP)
        || rl.is_key_down(raylib::consts::KeyboardKey::KEY_W)
    {
        inputs.up = true;
    }
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_DOWN)
        || rl.is_key_down(raylib::consts::KeyboardKey::KEY_S)
    {
        inputs.down = true;
    }
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_SPACE) {
        inputs.shoot = true;
    }

    state.resources.insert(inputs);
}

pub fn step(rl: &mut RaylibHandle, state: &mut State) {
    let mouse_pos_rl = rl.get_mouse_position();
    let mouse_pos = Vec2::new(mouse_pos_rl.x, mouse_pos_rl.y);
    state.resources.insert(mouse_pos);

    state
        .playing_schedule
        .execute(&mut state.ecs, &mut state.resources);
}

pub fn draw(state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    let mut render_command_buffer = state.resources.get_mut::<RenderCommandBuffer>().unwrap();
    execute_render_command_buffer(d, &mut render_command_buffer);
}
