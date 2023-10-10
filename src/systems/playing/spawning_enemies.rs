use glam::Vec2;
use legion::systems::CommandBuffer;
pub use legion::*;
use rand::{rngs::StdRng, Rng};

use crate::{
    components::{Asteroid, Attachable, AttachedTo, CTransform, CaptureInPlayField, Gun, Physics},
    timer::{AsteroidSpawnTimer, GunSpawnTimer},
    DIMS,
};

#[system]
#[read_component(Asteroid)]
#[write_component(CTransform)]
pub fn spawn_asteroids(
    #[resource] asteroid_spawn_timer: &mut AsteroidSpawnTimer,
    #[resource] rng: &mut StdRng,
    cmd: &mut CommandBuffer,
) {
    asteroid_spawn_timer.step();

    if asteroid_spawn_timer.get_countdown() == 0 {
        asteroid_spawn_timer.reset();

        let size = rng.gen_range(10..30);
        let padded_size = size as f32 * 2.0;
        let position = get_padded_position_outside_play_area(rng, padded_size);
        let target_position = get_random_pos_in_play_area(rng);
        let direction = (target_position - position).normalize();
        let velocity = direction * rng.gen_range(0.5..1.0);

        let angle = rng.gen_range(0.0..360.0);
        let rotation = glam::Mat2::from_angle(angle) * Vec2::new(0.0, 1.0);

        cmd.push((
            CTransform {
                pos: position,
                rot: rotation,
            },
            Asteroid { size },
            Physics {
                vel: velocity,
                rot_vel: rng.gen_range(-0.01..0.01),
            },
            CaptureInPlayField,
        ));
    }
}

// state.ecs.push();

#[system]
#[write_component(CTransform)]
pub fn spawn_guns(
    #[resource] gun_spawn_timer: &mut GunSpawnTimer,
    #[resource] rng: &mut StdRng,
    cmd: &mut CommandBuffer,
) {
    gun_spawn_timer.step();

    if gun_spawn_timer.get_countdown() != 0 {
        return;
    }
    gun_spawn_timer.reset();

    let size = 10;
    let padded_size = size as f32 * 2.0;
    let position = get_padded_position_outside_play_area(rng, padded_size);
    let target_position = get_random_pos_in_play_area(rng);
    let direction = (target_position - position).normalize();
    let velocity = direction * rng.gen_range(0.5..1.0);

    let angle = rng.gen_range(0.0..360.0);
    let rotation = glam::Mat2::from_angle(angle) * Vec2::new(0.0, 1.0);

    cmd.push((
        CTransform {
            pos: position,
            rot: rotation,
        },
        Physics {
            vel: velocity,
            rot_vel: rng.gen_range(-0.01..0.01),
        },
        Gun {
            wants_to_shoot: false,
            fire_delay: rng.gen_range(1..20),
            cooldown: 0,
        },
        // AttachedTo {
        //     entity: player,
        //     offset: random_offset,
        // }, also handle input controlled, and owned by
        Attachable,
    ));
}

pub fn get_position_outside_play_area(rng: &mut StdRng) -> Vec2 {
    get_padded_position_outside_play_area(rng, 0.0)
}

/** gives a random position outside the viewable area, also can account for padded sizes */
pub fn get_padded_position_outside_play_area(rng: &mut StdRng, padded_size: f32) -> Vec2 {
    // position needs to be outside of the screen
    // there are 8 zones, first pick a zone
    let zone = rng.gen_range(0..1);
    let position = match zone {
        0 => Vec2::new(
            // top left
            rng.gen_range(-padded_size * 2.0..-padded_size),
            rng.gen_range(-padded_size * 2.0..-padded_size),
        ),
        1 => Vec2::new(
            // top right
            rng.gen_range(DIMS.x as f32 + padded_size..DIMS.x as f32 + padded_size * 2.0),
            rng.gen_range(-padded_size * 2.0..-padded_size),
        ),
        2 => Vec2::new(
            // bottom right
            rng.gen_range(DIMS.x as f32 + padded_size..DIMS.x as f32 + padded_size * 2.0),
            rng.gen_range(DIMS.y as f32 + padded_size..DIMS.y as f32 + padded_size * 2.0),
        ),
        3 => Vec2::new(
            // bottom left
            rng.gen_range(-padded_size * 2.0..-padded_size),
            rng.gen_range(DIMS.y as f32 + padded_size..DIMS.y as f32 + padded_size * 2.0),
        ),
        4 => Vec2::new(
            // top
            rng.gen_range(0.0..DIMS.x as f32),
            rng.gen_range(-padded_size * 2.0..-padded_size),
        ),
        5 => Vec2::new(
            // bottom
            rng.gen_range(0.0..DIMS.x as f32),
            rng.gen_range(DIMS.y as f32 + padded_size..DIMS.y as f32 + padded_size * 2.0),
        ),
        6 => Vec2::new(
            // left
            rng.gen_range(-padded_size * 2.0..-padded_size),
            rng.gen_range(0.0..DIMS.y as f32),
        ),
        7 => Vec2::new(
            // right
            rng.gen_range(DIMS.x as f32 + padded_size..DIMS.x as f32 + padded_size * 2.0),
            rng.gen_range(0.0..DIMS.y as f32),
        ),
        _ => panic!("Unexpected zone"), // This shouldn't happen with rng.gen_range(0..8)
    };
    position
}

pub fn get_random_pos_in_play_area(rng: &mut StdRng) -> Vec2 {
    Vec2::new(
        rng.gen_range(0.0..DIMS.x as f32),
        rng.gen_range(0.0..DIMS.y as f32),
    )
}
