use glam::Vec2;
pub use legion::*;
use rand::rngs::ThreadRng;
use raylib::prelude::*;

use crate::systems::{
    capture_in_play_field_system, collision_system, control_system, entity_render_system,
    guns_system, physics, physics_system, spawn_asteroids_system, step_lifespan,
    step_lifespan_system, world_wrap_system,
};

pub const FRAMES_PER_SECOND: u32 = 60;

pub enum GameMode {
    Title,
    Playing,
    GameOver,
}

pub struct State {
    pub running: bool,
    pub time_since_last_update: f32,
    pub game_mode: GameMode,

    pub ecs: World,
    pub resources: Resources,
    pub schedule: Schedule,
}

impl State {
    pub fn new() -> Self {
        let mut ecs = World::default();

        // spawn some entities that have Transform and a ccolor
        // let mut rng: ThreadRng = rand::thread_rng();

        let resources = Resources::default();
        let schedule = Schedule::builder()
            .add_system(control_system())
            .flush()
            .add_system(guns_system())
            .add_system(collision_system())
            .flush()
            .add_system(step_lifespan_system())
            .add_system(spawn_asteroids_system())
            .add_system(physics_system())
            .add_system(world_wrap_system())
            .add_system(capture_in_play_field_system())
            .flush()
            .add_system(entity_render_system())
            .build();

        Self {
            running: true,
            time_since_last_update: 0.0,
            game_mode: GameMode::Title,

            ecs,
            resources,
            schedule,
        }
    }
}
