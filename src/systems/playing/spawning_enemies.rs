use glam::Vec2;
use legion::systems::CommandBuffer;
pub use legion::*;
use rand::{rngs::StdRng, Rng};

use crate::{
    components::{Asteroid, CTransform, CaptureInPlayField, Physics},
    AsteroidSpawnTimer, DIMS,
};

#[system]
#[read_component(Asteroid)]
#[write_component(CTransform)]
pub fn spawn_asteroids(
    #[resource] asteroid_spawn_timer: &mut AsteroidSpawnTimer,
    #[resource] rng: &mut StdRng,
    cmd: &mut CommandBuffer,
) {
    asteroid_spawn_timer.countdown -= 1;

    if asteroid_spawn_timer.countdown == 0 {
        asteroid_spawn_timer.countdown = asteroid_spawn_timer.spawn_interval;

        let size = rng.gen_range(10..30);
        let padded_size = size as f32 * 2.0;

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

        let target_position = Vec2::new(
            rng.gen_range(0.0..DIMS.x as f32),
            rng.gen_range(0.0..DIMS.y as f32),
        );
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
