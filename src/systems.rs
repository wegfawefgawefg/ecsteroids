use std::thread::spawn;

use glam::{UVec2, Vec2};
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
pub use legion::*;
use rand::{
    rngs::{StdRng, ThreadRng},
    Rng,
};
use raylib::prelude::{Color, Vector2};

use crate::{
    components::{Asteroid, Bullet, CTransform, Gun, InputControlled, LifeSpan, Physics, Player},
    rendering::{DrawCommand, RenderCommandBuffer},
    AsteroidSpawnTimer, DIMS,
};

use crate::playing::PlayingInputs;

// system to make all enemies go to player

// system to damage player if touching player

// system to control InputControlled entities

// render system
/* fetch position and sprite entities, and just blit them with a fixed size with the given position */
#[system]
#[read_component(CTransform)]
#[read_component(Asteroid)]
pub fn entity_render(
    ecs: &SubWorld,
    #[resource] rng: &mut StdRng,
    #[resource] render_command_buffer: &mut RenderCommandBuffer,
) {
    render_command_buffer.clear();

    // schedule asteroid rendering
    <(&CTransform, &Asteroid)>::query()
        .iter(ecs)
        .for_each(|(transform, asteroid)| {
            render_command_buffer.push(DrawCommand::Asteroid {
                pos: transform.pos.as_uvec2(),
                size: asteroid.size,
                dir: transform.rot,
            });
        });

    // schedule bullet rendering
    <&CTransform>::query()
        .filter(component::<Bullet>())
        .iter(ecs)
        .for_each(|transform| {
            render_command_buffer.push(DrawCommand::ColoredSquare {
                pos: transform.pos.as_uvec2(),
                color: Color::new(255, rng.gen_range(10..255), 0, 255),
            });
        });

    // schedule player rendering
    <&CTransform>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .for_each(|transform| {
            render_command_buffer.push(DrawCommand::Ship {
                pos: transform.pos.as_uvec2(),
                dir: transform.rot,
            });
        });
}

const MAX_VEL: f32 = 1.0;

#[system]
#[write_component(CTransform)]
#[write_component(Physics)]
pub fn physics(ecs: &mut SubWorld) {
    let mut query = <(&mut CTransform, &mut Physics)>::query();
    for (ctransform, physics) in query.iter_mut(ecs) {
        if physics.vel.length() > MAX_VEL {
            physics.vel = physics.vel.normalize() * MAX_VEL;
        }
        ctransform.pos += physics.vel;

        let rot_matrix = glam::Mat2::from_angle(physics.rot_vel.to_radians() * 0.1);
        ctransform.rot = rot_matrix * ctransform.rot;

        // loop around the screen
        if ctransform.pos.x < 0.0 {
            ctransform.pos.x += DIMS.x as f32;
        } else if ctransform.pos.x > DIMS.x as f32 {
            ctransform.pos.x -= DIMS.x as f32;
        }
        if ctransform.pos.y < 0.0 {
            ctransform.pos.y += DIMS.y as f32;
        } else if ctransform.pos.y > DIMS.y as f32 {
            ctransform.pos.y -= DIMS.y as f32;
        }
    }
}

const ROTATION_SPEED: f32 = 50.0;
const ACCELERATION: f32 = 0.04;
#[system]
#[read_component(CTransform)]
#[write_component(Physics)]
#[write_component(Gun)]
pub fn control(ecs: &mut SubWorld, #[resource] inputs: &PlayingInputs) {
    let mut query =
        <(&CTransform, &mut Physics, &mut Gun)>::query().filter(component::<InputControlled>());
    for (ctransform, physics, gun) in query.iter_mut(ecs) {
        // if left is true in inputs, rotate left
        if inputs.left {
            physics.rot_vel = -ROTATION_SPEED;
        } else if inputs.right {
            physics.rot_vel = ROTATION_SPEED;
        } else {
            physics.rot_vel = 0.0;
        }

        // if up is true in inputs, accelerate
        if inputs.up {
            physics.vel += ctransform.rot * ACCELERATION;
        }

        // if down is true in inputs, decelerate
        if inputs.down {
            physics.vel -= ctransform.rot * ACCELERATION;
        }

        gun.wants_to_shoot = inputs.shoot;
    }
}

const BULLET_VELOCITY: f32 = 100.0;
#[system]
#[read_component(CTransform)]
#[write_component(Gun)]
pub fn guns(ecs: &mut SubWorld, cmd: &mut CommandBuffer) {
    let mut query = <(&CTransform, &mut Gun)>::query();
    for (ctransform, gun) in query.iter_mut(ecs) {
        if gun.cooldown > 0 {
            gun.cooldown -= 1;
        }

        if gun.cooldown == 0 && gun.wants_to_shoot {
            cmd.push((
                CTransform {
                    pos: ctransform.pos + ctransform.rot * 10.0,
                    rot: ctransform.rot,
                },
                Physics {
                    vel: ctransform.rot * BULLET_VELOCITY,
                    rot_vel: 0.0,
                },
                Bullet,
                LifeSpan { frames_left: 60 },
            ));

            gun.cooldown = gun.fire_delay;
        }
    }
}

#[system]
#[write_component(LifeSpan)]
pub fn step_lifespan(ecs: &mut SubWorld, cmd: &mut CommandBuffer) {
    let mut query = <(Entity, &mut LifeSpan)>::query();
    for (entity, lifespan) in query.iter_mut(ecs) {
        lifespan.frames_left -= 1;
        if lifespan.frames_left == 0 {
            cmd.remove(*entity);
        }
    }
}

#[system]
#[read_component(Asteroid)]
#[write_component(CTransform)]
pub fn spawn_asteroids(
    ecs: &mut SubWorld,
    #[resource] asteroid_spawn_timer: &mut AsteroidSpawnTimer,
    #[resource] rng: &mut StdRng,
    cmd: &mut CommandBuffer,
) {
    asteroid_spawn_timer.countdown -= 1;

    if asteroid_spawn_timer.countdown == 0 {
        asteroid_spawn_timer.countdown = asteroid_spawn_timer.spawn_interval;

        let position = Vec2::new(
            rng.gen_range(0.0..DIMS.x as f32),
            rng.gen_range(0.0..DIMS.y as f32),
        );
        let angle = rng.gen_range(0.0..360.0);
        let rotation = glam::Mat2::from_angle(angle) * Vec2::new(0.0, 1.0);

        cmd.push((
            CTransform {
                pos: position,
                rot: rotation,
            },
            Asteroid {
                size: rng.gen_range(10..30),
            },
            Physics {
                vel: Vec2::new(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5)),
                rot_vel: rng.gen_range(-0.01..0.01),
            },
        ));
    }
}

#[system]
#[read_component(CTransform)]
#[read_component(Bullet)]
#[read_component(Asteroid)]
pub fn collision(ecs: &mut SubWorld, cmd: &mut CommandBuffer, #[resource] rng: &mut StdRng) {
    let mut bullets = <(Entity, &CTransform)>::query().filter(component::<Bullet>());
    let mut asteroids = <(Entity, &CTransform, &Asteroid)>::query();

    // Bullet and asteroid collision
    for (asteroid_entity, asteroid_transform, asteroid) in asteroids.iter(ecs) {
        for (bullet_entity, bullet_transform) in bullets.iter(ecs) {
            let distance = (bullet_transform.pos - asteroid_transform.pos).length();
            let asteroid_radius = asteroid.size as f32 * 0.8;
            if distance <= asteroid_radius {
                cmd.remove(*bullet_entity);
                if asteroid.size < 3 {
                    cmd.remove(*asteroid_entity);
                } else {
                    split_asteroid(asteroid_transform, asteroid, cmd, rng);
                    cmd.remove(*asteroid_entity);
                }
            }
        }
    }

    // const MINIMUM_INTRA_ASTEROID_COLLISION_SIZE: u32 = 10;
    // // Asteroid and asteroid collision
    // let asteroids_vec: Vec<_> = asteroids.iter(ecs).collect();
    // for i in 0..asteroids_vec.len() {
    //     let (asteroid1_entity, asteroid1_transform, asteroid1) = &asteroids_vec[i];
    //     for j in (i + 1)..asteroids_vec.len() {
    //         let (asteroid2_entity, asteroid2_transform, asteroid2) = &asteroids_vec[j];
    //         let distance = (asteroid1_transform.pos - asteroid2_transform.pos).length();
    //         let combined_radius = (asteroid1.size + asteroid2.size) as f32 * 0.8;
    //         if distance <= combined_radius {
    //             if asteroid1.size >= MINIMUM_INTRA_ASTEROID_COLLISION_SIZE {
    //                 split_asteroid(asteroid1_transform, asteroid1, cmd, rng);
    //                 cmd.remove(**asteroid1_entity);
    //             }

    //             if asteroid2.size >= MINIMUM_INTRA_ASTEROID_COLLISION_SIZE {
    //                 split_asteroid(asteroid2_transform, asteroid2, cmd, rng);
    //                 cmd.remove(**asteroid2_entity);
    //             }
    //         }
    //     }
    // }
}

fn split_asteroid(
    transform: &CTransform,
    asteroid: &Asteroid,
    cmd: &mut CommandBuffer,
    rng: &mut StdRng,
) {
    let new_size = asteroid.size / 2;
    for _ in 0..2 {
        let random_velocity = Vec2::new(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5));
        cmd.push((
            CTransform {
                pos: transform.pos,
                rot: transform.rot,
            },
            Asteroid { size: new_size },
            Physics {
                vel: random_velocity,
                rot_vel: rng.gen_range(-50.0..50.0),
            },
        ));
    }
}
