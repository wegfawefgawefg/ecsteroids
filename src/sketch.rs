use std::sync::{Arc, Mutex};

use glam::Vec2;
pub use legion::*;
use rand::rngs::ThreadRng;
use raylib::prelude::*;

use crate::{
    components::{CTransform, InputControlled, Physics, Player},
    rendering::{execute_render_command_buffer, RenderCommandBuffer},
    systems::{
        collision_system, control_system, entity_render_system, physics, physics_system,
        spawn_asteroids_system, spawn_bullets_on_shoot_system, step_lifespan, step_lifespan_system,
    },
    AsteroidSpawnTimer,
};

pub const FRAMES_PER_SECOND: u32 = 60;

pub struct State {
    pub running: bool,
    pub time_since_last_update: f32,

    pub ecs: World,
    pub resources: Resources,
    pub schedule: Schedule,
}

impl State {
    pub fn new() -> Self {
        let mut ecs = World::default();

        // spawn some entities that have Transform and a ccolor
        let mut rng: ThreadRng = rand::thread_rng();

        // add a player
        ecs.push((
            CTransform {
                pos: Vec2::new(100.0, 100.0),
                rot: Vec2::new(0.0, 1.0),
            },
            Physics {
                vel: Vec2::new(1.0, 1.0),
                rot_vel: 30.0,
            },
            InputControlled,
            Player,
        ));

        let mut resources = Resources::default();
        let schedule = Schedule::builder()
            // .add_system(player_input::player_input_system())
            // .add_system(map_render::map_render_system())
            // .add_system(wander_system())
            // .add_system(chase_mouse_system())
            .add_system(control_system())
            .flush()
            .add_system(spawn_bullets_on_shoot_system())
            .add_system(collision_system())
            .flush()
            .add_system(step_lifespan_system())
            .add_system(spawn_asteroids_system())
            .add_system(physics_system())
            .add_system(entity_render_system())
            .build();

        let asteroid_spawn_timer = AsteroidSpawnTimer::new(1000);

        Self {
            running: true,
            time_since_last_update: 0.0,

            ecs,
            resources,
            schedule,
        }
    }
}

pub struct Inputs {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub shoot: bool,
}

pub fn process_events_and_input(rl: &mut RaylibHandle, state: &mut State) {
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ESCAPE) {
        state.running = false;
    }

    let mut inputs = Inputs {
        left: false,
        right: false,
        up: false,
        down: false,
        shoot: false,
    };
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_LEFT) {
        inputs.left = true;
    }
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_RIGHT) {
        inputs.right = true;
    }
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_UP) {
        inputs.up = true;
    }
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_DOWN) {
        inputs.down = true;
    }
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_SPACE) {
        inputs.shoot = true;
    }

    state.resources.insert(inputs);
}

pub fn step(rl: &mut RaylibHandle, rlt: &mut RaylibThread, state: &mut State) {
    let mouse_pos_rl = rl.get_mouse_position();
    let mouse_pos = Vec2::new(mouse_pos_rl.x, mouse_pos_rl.y);
    state.resources.insert(mouse_pos);

    state.schedule.execute(&mut state.ecs, &mut state.resources);
}

pub fn draw(state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    d.draw_text("Stroided!", 12, 12, 12, Color::WHITE);
    let mouse_pos = d.get_mouse_position();
    d.draw_circle(mouse_pos.x as i32, mouse_pos.y as i32, 6.0, Color::GREEN);

    let mut render_command_buffer = state.resources.get_mut::<RenderCommandBuffer>().unwrap();
    execute_render_command_buffer(d, &mut render_command_buffer);

    // let angle = d.get_time() as f32;

    // let center = Vec2::new(d.get_screen_width() as f32, d.get_screen_height() as f32) / 2.0;
    // let offset = center / 4.0;

    // for i in 0..3 {
    //     let rot = glam::Mat2::from_angle(angle + i as f32 * 90.0);
    //     let rect_pos_rotated = rot * offset + center;

    //     let size =
    //         (((d.get_time() as f32 + i as f32 * 1.0) * 2.0).sin() + 1.0) / 2.0 * offset.y + 4.0;
    //     d.draw_rectangle(
    //         rect_pos_rotated.x as i32,
    //         rect_pos_rotated.y as i32,
    //         size as i32,
    //         size as i32,
    //         Color::RED,
    //     );
    // }
}
