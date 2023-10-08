use audio::Song;
use glam::UVec2;
use rand::rngs::StdRng;
use rand::SeedableRng;
use raylib::prelude::*;
use raylib::{ffi::SetTraceLogLevel, prelude::TraceLogLevel};
use rendering::RenderCommandBuffer;
use state::GameMode;

mod audio;
mod components;
mod game_over;
mod playing;
mod rendering;
mod state;
mod systems;
mod title;

pub struct AsteroidSpawnTimer {
    pub spawn_interval: u32, // frames
    pub countdown: u32,
}

impl AsteroidSpawnTimer {
    pub fn new(spawn_interval: u32) -> Self {
        Self {
            spawn_interval,
            countdown: spawn_interval,
        }
    }
}

const DIMS: UVec2 = UVec2::new(240, 160);

const TIMESTEP: f32 = 1.0 / state::FRAMES_PER_SECOND as f32;
fn main() {
    let mut state = state::State::new();

    let (mut rl, mut rlt) = raylib::init().title("raylib-rs-lowres-template").build();
    unsafe {
        SetTraceLogLevel(TraceLogLevel::LOG_WARNING as i32);
    }

    let mut audio = audio::Audio::new(&mut rl, &rlt);
    audio
        .rl_audio_device
        .play_music_stream(&mut audio.songs[Song::Playing as usize]);

    let window_dims = UVec2::new(1280, 720);
    let fullscreen = false;
    rl.set_window_size(window_dims.x as i32, window_dims.y as i32);
    if fullscreen {
        rl.toggle_fullscreen();
        rl.set_window_size(rl.get_screen_width(), rl.get_screen_height());
    }

    center_window(&mut rl, window_dims);
    let mouse_scale = DIMS.as_vec2() / window_dims.as_vec2();
    rl.set_mouse_scale(mouse_scale.x as f32, mouse_scale.y as f32);

    let mut render_texture = rl
        .load_render_texture(&rlt, DIMS.x, DIMS.y)
        .unwrap_or_else(|e| {
            println!("Error creating render texture: {}", e);
            std::process::exit(1);
        });

    let render_command_buffer: RenderCommandBuffer = RenderCommandBuffer::new();
    state.resources.insert(render_command_buffer);

    let rng: StdRng = StdRng::from_entropy();
    state.resources.insert(rng);

    let asteroid_spawn_timer = AsteroidSpawnTimer::new(100);
    state
        .resources
        .insert::<AsteroidSpawnTimer>(asteroid_spawn_timer);

    while state.running && !rl.window_should_close() {
        match state.game_mode {
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

            match state.game_mode {
                GameMode::Title => {
                    title::step(&mut rl, &mut rlt, &mut state);
                }
                GameMode::Playing => {
                    playing::step(&mut rl, &mut rlt, &mut state);
                }
                GameMode::GameOver => {
                    game_over::step(&mut rl, &mut rlt, &mut state);
                }
            }

            audio
                .rl_audio_device
                .update_music_stream(&mut audio.songs[Song::Playing as usize]);
        }

        let mut draw_handle = rl.begin_drawing(&rlt);
        {
            let low_res_draw_handle =
                &mut draw_handle.begin_texture_mode(&rlt, &mut render_texture);
            low_res_draw_handle.clear_background(Color::BLACK);

            match state.game_mode {
                GameMode::Title => {
                    title::draw(&state, low_res_draw_handle);
                }
                GameMode::Playing => {
                    playing::draw(&state, low_res_draw_handle);
                }
                GameMode::GameOver => {
                    game_over::draw(&state, low_res_draw_handle);
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

pub fn center_window(rl: &mut raylib::RaylibHandle, window_dims: UVec2) {
    let screen_dims = UVec2::new(rl.get_screen_width() as u32, rl.get_screen_height() as u32);
    let screen_center = screen_dims / 2;
    let window_center = window_dims / 2;
    let mut offset = window_center - screen_center;
    offset.y += 500;
    rl.set_window_position(offset.x as i32, offset.y as i32);
    rl.set_target_fps(144);
}

pub fn scale_and_blit_render_texture_to_window(
    draw_handle: &mut RaylibDrawHandle,
    render_texture: &mut RenderTexture2D,
    fullscreen: bool,
    window_dims: UVec2,
) {
    let source_rec = Rectangle::new(
        0.0,
        0.0,
        render_texture.texture.width as f32,
        -render_texture.texture.height as f32,
    );
    // dest rec should be the fullscreen resolution if graphics.fullscreen, otherwise window_dims
    let dest_rec = if fullscreen {
        // get the fullscreen resolution
        let screen_width = draw_handle.get_screen_width();
        let screen_height = draw_handle.get_screen_height();
        Rectangle::new(0.0, 0.0, screen_width as f32, screen_height as f32)
    } else {
        Rectangle::new(0.0, 0.0, window_dims.x as f32, window_dims.y as f32)
    };

    let origin = Vector2::new(0.0, 0.0);

    draw_handle.draw_texture_pro(
        render_texture,
        source_rec,
        dest_rec,
        origin,
        0.0,
        Color::WHITE,
    );
}
