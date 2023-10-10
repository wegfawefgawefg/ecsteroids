use legion::Schedule;

use crate::systems::playing::{
    attached::stick_to_attached_system,
    collision::{attach_to_grab_zone, attach_to_grab_zone_system, collision_system},
    physics::{capture_in_play_field_system, physics_system, world_wrap_system},
    rendering::{entity_render_system, render_expiring_messages_system, score_render_system},
    shooting::guns_system,
    spawning_enemies::{spawn_asteroids_system, spawn_guns_system},
    state_changing::game_over_system,
    util::step_lifespan_system,
};

pub fn build_title_schedule() -> Schedule {
    Schedule::builder()
        .add_system(crate::systems::title::input_handling::handle_inputs_system())
        .build()
}

pub fn build_play_schedule() -> Schedule {
    Schedule::builder()
        .add_system(crate::systems::playing::input_handling::handle_inputs_system())
        .flush()
        .add_system(physics_system())
        .add_system(stick_to_attached_system())
        .add_system(guns_system())
        .add_system(collision_system())
        .add_system(attach_to_grab_zone_system())
        .flush()
        .add_system(spawn_asteroids_system())
        .add_system(spawn_guns_system())
        .add_system(world_wrap_system())
        .add_system(capture_in_play_field_system())
        .add_system(step_lifespan_system())
        .flush()
        .add_system(game_over_system())
        .add_system(entity_render_system())
        .add_system(score_render_system())
        .add_system(render_expiring_messages_system())
        .build()
}
