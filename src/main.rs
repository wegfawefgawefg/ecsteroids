use audio::Song;
use glam::UVec2;
use raylib::prelude::*;
use raylib::{ffi::SetTraceLogLevel, prelude::TraceLogLevel};
use rendering::RenderCommandBuffer;
use state::GameMode;
use systems::playing::state_changing::game_over;
use window_helpers::{center_window, scale_and_blit_render_texture_to_window};

mod audio;
mod components;
mod game_over;
mod message_stream;
mod playing;
mod rendering;
mod schedules;
mod state;
mod systems;
mod timer;
mod title;
mod window_helpers;

const DIMS: UVec2 = UVec2::new(240, 160);

const TIMESTEP: f32 = 1.0 / state::FRAMES_PER_SECOND as f32;
fn main() {
    let (mut rl, rlt) = raylib::init().title("raylib-rs-lowres-template").build();
    unsafe {
        SetTraceLogLevel(TraceLogLevel::LOG_WARNING as i32);
    }

    ////////////////    INIT GRAPHICS    ////////////////
    let window_dims = UVec2::new(1280, 720);
    let fullscreen = false;
    rl.set_window_size(window_dims.x as i32, window_dims.y as i32);
    if fullscreen {
        rl.toggle_fullscreen();
        rl.set_window_size(rl.get_screen_width(), rl.get_screen_height());
    }

    center_window(&mut rl, window_dims);
    let mouse_scale = DIMS.as_vec2() / window_dims.as_vec2();
    rl.set_mouse_scale(mouse_scale.x, mouse_scale.y);

    let mut render_texture = rl
        .load_render_texture(&rlt, DIMS.x, DIMS.y)
        .unwrap_or_else(|e| {
            println!("Error creating render texture: {}", e);
            std::process::exit(1);
        });

    ////////////////    INIT AUDIO    ////////////////
    let mut audio = audio::Audio::new(&mut rl, &rlt);
    audio
        .rl_audio_device
        .play_music_stream(&mut audio.songs[Song::Playing as usize]);

    ////////////////    INIT STATE    ////////////////
    let mut state = state::State::new();

    ////////////////    MAIN LOOP    ////////////////
    while state.running && !rl.window_should_close() {
        // handle state transitions
        let transition_to: Option<GameMode> = match state.resources.get::<Option<GameMode>>() {
            Some(transition_to_opt_ref) => *transition_to_opt_ref,
            _ => None,
        };

        // DANK WARNING: handle state transitions
        if let Some(transition_to) = transition_to {
            match transition_to {
                GameMode::Title => {
                    systems::title::init_state::init(&mut state);
                }
                GameMode::Playing => {
                    systems::playing::init_state::init(&mut state);
                }
                GameMode::GameOver => {}
            }
        }
        if let Some(transition_to) = transition_to {
            let new_game_mode = transition_to;
            state.resources.insert(new_game_mode);
        }
        if let Some(mut transition_to) = state.resources.get_mut::<Option<GameMode>>() {
            *transition_to = None;
        }

        let game_mode = *state.resources.get::<GameMode>().unwrap();
        match game_mode {
            GameMode::Title => {
                title::process_events_and_input(&mut rl, &mut state);
            }
            GameMode::Playing => {
                playing::process_events_and_input(&mut rl, &mut state);
            }
            GameMode::GameOver => {
                game_over::process_events_and_input(&mut rl, &mut state);
            }
        }

        let dt = rl.get_frame_time();
        state.time_since_last_update += dt;
        if state.time_since_last_update > TIMESTEP {
            state.time_since_last_update = 0.0;

            if let Some(mut render_command_buffer) =
                state.resources.get_mut::<RenderCommandBuffer>()
            {
                render_command_buffer.clear();
            }

            match game_mode {
                GameMode::Title => {
                    title::step(&mut rl, &mut state);
                }
                GameMode::Playing => {
                    playing::step(&mut rl, &mut state);
                }
                GameMode::GameOver => {
                    playing::step(&mut rl, &mut state);
                }
            }

            // UNMUTE THIS TO HEAR THE MUSIC
            // audio
            //     .rl_audio_device
            //     .update_music_stream(&mut audio.songs[Song::Playing as usize]);
        }

        let mut draw_handle = rl.begin_drawing(&rlt);
        {
            let low_res_draw_handle =
                &mut draw_handle.begin_texture_mode(&rlt, &mut render_texture);
            low_res_draw_handle.clear_background(Color::BLACK);

            match game_mode {
                GameMode::Title => {
                    title::draw(&state, low_res_draw_handle);
                }
                GameMode::Playing => {
                    playing::draw(&state, low_res_draw_handle);
                }
                GameMode::GameOver => {
                    playing::draw(&state, low_res_draw_handle);
                }
            }
        }
        scale_and_blit_render_texture_to_window(
            &mut draw_handle,
            &mut render_texture,
            fullscreen,
            window_dims,
        );
    }
}
