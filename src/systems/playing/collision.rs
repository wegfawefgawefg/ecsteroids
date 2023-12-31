use glam::Vec2;
pub use legion::*;
use legion::{systems::CommandBuffer, world::SubWorld};
use rand::{rngs::StdRng, Rng};

use crate::{
    audio_playing::{AudioCommand, AudioCommandBuffer},
    components::{
        Asteroid, Attachable, AttachedTo, Bullet, CTransform, Enemy, GrabZone, InputControlled,
        OwnedBy, Physics, Player, Score,
    },
};

pub struct ScoreInstance {
    pub owner: Entity,
    pub score: u32,
}

#[system]
#[read_component(CTransform)]
#[read_component(Bullet)]
#[read_component(Asteroid)]
#[write_component(OwnedBy)]
#[read_component(Score)]
#[write_component(Score)]
pub fn collision(
    ecs: &mut SubWorld,
    cmd: &mut CommandBuffer,
    #[resource] audio_command_buffer: &mut AudioCommandBuffer,
    #[resource] rng: &mut StdRng,
) {
    let mut score_instances: Vec<ScoreInstance> = Vec::new();

    let mut bullets = <(Entity, &CTransform)>::query().filter(component::<Bullet>());
    let mut asteroids = <(Entity, &CTransform, &Asteroid)>::query();

    // Bullet and asteroid collision
    for (asteroid_entity, asteroid_transform, asteroid) in asteroids.iter(ecs) {
        for (bullet_entity, bullet_transform) in bullets.iter(ecs) {
            let distance = (bullet_transform.pos - asteroid_transform.pos).length();
            let asteroid_radius = asteroid.size as f32 * 0.8;
            if distance <= asteroid_radius {
                if asteroid.size < 3 {
                    cmd.remove(*asteroid_entity);
                } else {
                    split_asteroid(asteroid_transform, asteroid, cmd, rng);
                    cmd.remove(*asteroid_entity);
                }
                cmd.remove(*bullet_entity);

                if let Ok(entry) = ecs.entry_ref(*bullet_entity) {
                    if let Ok(owned_by) = entry.get_component::<OwnedBy>() {
                        // if theres already a score for this entity, increment it
                        if let Some(score_instance) = score_instances
                            .iter_mut()
                            .find(|score_instance| score_instance.owner == owned_by.owner)
                        {
                            score_instance.score += 1;
                            continue;
                        } else {
                            score_instances.push(ScoreInstance {
                                owner: owned_by.owner,
                                score: 1,
                            });
                        }
                    } else {
                        println!(
                            "Failed to get OwnedBy component for bullet: {:?}",
                            bullet_entity
                        );
                    }
                } else {
                    println!("Failed to get entry for bullet: {:?}", bullet_entity);
                }

                audio_command_buffer.push(AudioCommand::AsteroidExplosion);
            }
        }
    }

    // process the score instances
    for score_instance in score_instances {
        let mut score_query = <&mut Score>::query();
        let score = score_query
            .iter_mut(ecs)
            .find(|score| score.owner == score_instance.owner);
        match score {
            Some(score) => {
                score.score += score_instance.score;
            }
            None => {
                cmd.push((Score {
                    owner: score_instance.owner,
                    score: score_instance.score,
                },));
            }
        }
    }

    let mut dead_ships: Vec<Entity> = Vec::new();

    // asteroid and player collision
    let mut players =
        <(Entity, &CTransform)>::query().filter(component::<Player>() | component::<Enemy>());
    for (player_entity, player_transform) in players.iter(ecs) {
        for (asteroid_entity, asteroid_transform, asteroid) in asteroids.iter(ecs) {
            let distance = (player_transform.pos - asteroid_transform.pos).length();
            let combined_radius = (asteroid.size + 1) as f32 * 0.8;
            if distance <= combined_radius {
                cmd.remove(*asteroid_entity);
                cmd.remove(*player_entity);
                dead_ships.push(*player_entity);
            }
        }
    }

    // remove scores for any dead players
    // let mut score_query = <(Entity, &Score)>::query();
    // for (entity, score) in score_query.iter(ecs) {
    //     if dead_players.contains(&score.owner) {
    //         cmd.remove(*entity);
    //     }
    // }

    //  //  //  //// // ASTEROID TO ASTEROID COLLISION    //  //  //  //
    //  //  //  //// // BLEW UP MY COMPUTER   //  //  //  //  //
    // const MINIMUM_INTRA_ASTEROID_COLLISION_SIZE: u32 = 10;
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

/*
- attachables are floating around
- they stick to something that is a attach_piece
    - upon sticking they become attached to the attach_piece, and if the attach_piece has an owner, adopt the attach_pieces owner

- attach_piece propogation

    - if a attach_piece becomes detached,
        - removes its owned by
        - removes its attached to, (this will need to trigger on attach_pieces on attach_pieces)
        - anything attached to this attach_piece will also need remove owned_by

    - if a attach_piece becomes attached,
        - become owned by attacher
        - become attached_to
        - anything attached to this attach_piece will also need to get owned_by set

*/
#[system]
#[write_component(CTransform)]
#[read_component(Physics)]
#[read_component(GrabZone)]
#[read_component(Attachable)]
pub fn attach_to_grab_zone(ecs: &mut SubWorld, cmd: &mut CommandBuffer) {
    // 1. Fetch all attachable entities
    // let unattached_attachables: Vec<(Entity, CTransform)> =
    //     <(Entity, &CTransform, &Attachable)>::query()
    //         .filter(!component::<AttachedTo>()) // 2. Filter out attached entities
    //         .iter(ecs)
    //         .map(|(entity, transform, _)| (*entity, *transform))
    //         .collect();

    // 3. Fetch all grab zones
    let grab_zone_entities: Vec<(Entity, CTransform, GrabZone)> =
        <(Entity, &CTransform, &GrabZone)>::query()
            .iter(ecs)
            .map(|(entity, transform, grabzone)| (*entity, *transform, *grabzone))
            .collect();

    let mut unattached_attachables =
        <(Entity, &CTransform, &Attachable)>::query().filter(!component::<AttachedTo>());
    for (attachable_entity, attachable_transform, _) in unattached_attachables.iter_mut(ecs) {
        for (grab_entity, grab_transform, grab_zone) in grab_zone_entities.iter() {
            let offset = attachable_transform.pos - grab_transform.pos;
            let distance = offset.length();
            if distance <= grab_zone.radius {
                // This computes the matrix to rotate by -theta if rot represents rotation by theta.
                // let inverse_rotation_matrix = glam::Mat2::from_cols(
                //     grab_transform.rot,
                //     Vec2::new(grab_transform.rot.y, grab_transform.rot.x),
                // );
                let rot_angle = grab_transform.rot.x.atan2(grab_transform.rot.y);
                let rot_matrix = glam::Mat2::from_angle(rot_angle - std::f32::consts::FRAC_PI_2);
                let rot_offset = rot_matrix * offset;

                let attached = AttachedTo {
                    entity: *grab_entity,
                    offset: rot_offset,
                };

                cmd.add_component(*attachable_entity, attached);
                cmd.add_component(*attachable_entity, InputControlled);
                cmd.add_component(
                    *attachable_entity,
                    OwnedBy {
                        owner: *grab_entity,
                    },
                );
                break; // break out of the inner loop, as it's already attached
            }
        }
    }
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
