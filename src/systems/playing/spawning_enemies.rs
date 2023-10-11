use glam::Vec2;
use legion::systems::CommandBuffer;
pub use legion::*;
use rand::{rngs::StdRng, Rng};

use crate::{
    components::{
        Asteroid, Attachable, AttachedTo, CTransform, CaptureInPlayField, Enemy, GrabZone, Gun,
        Physics,
    },
    timer::{AsteroidSpawnTimer, EnemySpawnTimer, GunSpawnTimer},
    DIMS,
};

use super::util::{get_padded_position_outside_play_area, get_random_pos_in_play_area};

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

/*
    let player = state.ecs.push((
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
        GrabZone { radius: 15.0 },
    ));
*/

#[system]
#[read_component(Asteroid)]
#[write_component(CTransform)]
pub fn spawn_enemies(
    #[resource] enemy_spawn_timer: &mut EnemySpawnTimer,
    #[resource] rng: &mut StdRng,
    cmd: &mut CommandBuffer,
) {
    enemy_spawn_timer.step();

    if enemy_spawn_timer.get_countdown() == 0 {
        enemy_spawn_timer.reset();

        let size = 1;
        let padded_size = size as f32 * 2.0;
        let position = get_padded_position_outside_play_area(rng, padded_size);
        let target_position = get_random_pos_in_play_area(rng);
        let direction = (target_position - position).normalize();
        let velocity = direction * rng.gen_range(0.5..1.0);

        let angle = rng.gen_range(0.0..360.0);
        let rotation = glam::Mat2::from_angle(angle) * Vec2::new(0.0, 1.0);

        cmd.push((
            Enemy,
            CTransform {
                pos: position,
                rot: direction,
            },
            Physics {
                vel: velocity,
                rot_vel: 0.0,
            },
            GrabZone { radius: 10.0 },
        ));
    }
}

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

    let size = 5;
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
            fire_delay: rng.gen_range(28..32),
            cooldown: 0,
        },
        // AttachedTo {
        //     entity: player,
        //     offset: random_offset,
        // }, also handle input controlled, and owned by
        Attachable,
    ));
}
