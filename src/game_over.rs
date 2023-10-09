use raylib::RaylibHandle;

use crate::state::{GameMode, State};

pub fn process_events_and_input(rl: &mut RaylibHandle, state: &mut State) {
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ESCAPE) {
        state.running = false;
    }

    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE) {
        if let Some(mut transition_to) = state.resources.get_mut::<Option<GameMode>>() {
            *transition_to = Option::Some(GameMode::Title);
        }
    }
}
