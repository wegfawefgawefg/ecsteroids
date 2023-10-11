use std::collections::HashMap;

use glam::Vec2;
pub use legion::*;
use legion::{storage::Component, systems::CommandBuffer, world::SubWorld};
use rand::{rngs::StdRng, Rng};

use crate::{
    components::{
        CTransform, Enemy, Gun, InputControlled, LookAt, OwnedBy, Physics, Player, WantsToGoTo,
    },
    playing::PlayingInputs,
    state::GameMode,
};

use super::util::{get_random_pos_in_play_area, is_in_play_area};

const ROTATION_SPEED: f32 = 50.0;
const ACCELERATION: f32 = 0.04;
#[system]
#[read_component(CTransform)]
#[write_component(Physics)]
#[read_component(Enemy)]
#[read_component(Player)]
#[read_component(LookAt)]
#[write_component(WantsToGoTo)]
pub fn enemy_behaviour(ecs: &mut SubWorld, #[resource] rng: &mut StdRng, cmd: &mut CommandBuffer) {
    let players: Vec<_> = <Entity>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .copied()
        .collect();

    // if the enemy has no lookat, pick a random player and look at it
    if !players.is_empty() {
        for (entity, _) in <(Entity, &Enemy)>::query()
            .filter(!component::<LookAt>())
            .iter_mut(ecs)
        {
            let target_player_index = rng.gen_range(0..players.len());
            let target_player = players[target_player_index];

            cmd.add_component(
                *entity,
                LookAt {
                    entity: target_player,
                },
            );
        }
    }

    // if the enemy has no target, get one
    for (entity, _) in <(Entity, &Enemy)>::query()
        .filter(!component::<WantsToGoTo>())
        .iter_mut(ecs)
    {
        let target_position = get_random_pos_in_play_area(rng);
        cmd.add_component(
            *entity,
            WantsToGoTo {
                pos: target_position,
            },
        );
    }

    let unowned_guns: Vec<(Entity, Vec2)> = <(Entity, &CTransform)>::query()
        .filter(component::<Gun>() & !component::<OwnedBy>())
        .iter(ecs)
        .map(|(entity, transform)| (*entity, transform.pos))
        .filter(|(_, pos)| is_in_play_area(*pos))
        .collect();

    // always set your wants to go to to be the nearest unowned gun if there are any
    if !unowned_guns.is_empty() {
        for (entity, enemy_transform, wants_to_go_to) in
            <(Entity, &CTransform, &mut WantsToGoTo)>::query()
                .filter(component::<Enemy>())
                .iter_mut(ecs)
        {
            let enemy_pos = enemy_transform.pos;
            if let Some(closest_gun) =
                unowned_guns
                    .iter()
                    .min_by(|&(_, gun_pos1), &(_, gun_pos2)| {
                        let dist1 = (*gun_pos1 - enemy_pos).length_squared(); // using squared magnitude for performance
                        let dist2 = (*gun_pos2 - enemy_pos).length_squared();
                        dist1
                            .partial_cmp(&dist2)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
            {
                let closest_gun_pos = closest_gun.1;
                wants_to_go_to.pos = closest_gun_pos;
            }
        }
    }

    // if the enemy has a target, vel towards it
    //  TODO: should be its own system
    for (transform, physics, wants_to_go_to) in <(&CTransform, &mut Physics, &WantsToGoTo)>::query()
        .filter(component::<Enemy>())
        .iter_mut(ecs)
    {
        let dir = (wants_to_go_to.pos - transform.pos).normalize();
        // physics.vel *= 0.98;
        // physics.vel += dir * 0.02;
        // physics.vel *= 0.98;
        physics.vel += dir * 0.5;
    }

    // if the enemy is within 5 points of the target, remove the target
    for (entity, transform, wants_to_go_to) in <(Entity, &CTransform, &WantsToGoTo)>::query()
        .filter(component::<Enemy>())
        .iter_mut(ecs)
    {
        let delta = wants_to_go_to.pos - transform.pos;
        if delta.length() < 5.0 {
            cmd.remove_component::<WantsToGoTo>(*entity);
        }
    }

    // if the enemy has a target, make sure that target exists still, if it doesnt, remove the target

    // let mut query = <(&CTransform, &mut Physics)>::query().filter(component::<InputControlled>());
    // for (ctransform, physics) in query.iter_mut(ecs) {
    //     // if left is true in inputs, rotate left
    //     if inputs.left {
    //         physics.rot_vel = -ROTATION_SPEED;
    //     } else if inputs.right {
    //         physics.rot_vel = ROTATION_SPEED;
    //     } else {
    //         physics.rot_vel = 0.0;
    //     }

    //     // if up is true in inputs, accelerate
    //     if inputs.up {
    //         physics.vel += ctransform.rot * ACCELERATION;
    //     }

    //     // if down is true in inputs, decelerate
    //     if inputs.down {
    //         physics.vel -= ctransform.rot * ACCELERATION;
    //     }
    // }

    // let mut query = <&mut Gun>::query().filter(component::<InputControlled>());
    // for gun in query.iter_mut(ecs) {
    //     gun.wants_to_shoot = inputs.shoot;
    // }

    // // if theres no players, go back to title on shoot press
    // let mut players = <(Entity, &CTransform)>::query().filter(component::<Player>());
    // if players.iter(ecs).count() == 0 && inputs.shoot {
    //     *transition_to = Some(GameMode::GameOver);
    // }
}

#[system]
#[allow(clippy::collapsible_match)]
#[read_component(Entity)]
#[read_component(LookAt)]
#[write_component(Physics)]
#[read_component(CTransform)]
pub fn look_at(ecs: &mut SubWorld) {
    let entity_look_at_targets: HashMap<Entity, Option<CTransform>> = <(Entity, &LookAt)>::query()
        .iter(ecs)
        .map(|(entity, look_at)| {
            let transform = ecs
                .entry_ref(look_at.entity)
                .ok()
                .and_then(|entry| entry.get_component::<CTransform>().ok().copied());
            (*entity, transform)
        })
        .collect();

    let mut query = <(Entity, &CTransform, &mut Physics)>::query()
        .filter(component::<Enemy>() & component::<LookAt>());
    for (entity, transform, physics) in query.iter_mut(ecs) {
        if let Some(target_transform_option) = entity_look_at_targets.get(entity) {
            if let Some(target_transform) = target_transform_option {
                // Calculate the direction from the entity to the target
                let direction_to_target = target_transform.pos - transform.pos;

                // Normalize the direction
                let magnitude =
                    (direction_to_target.x.powi(2) + direction_to_target.y.powi(2)).sqrt();
                if magnitude != 0.0 {
                    let desired_rot = Vec2 {
                        x: direction_to_target.x / magnitude,
                        y: direction_to_target.y / magnitude,
                    };

                    // Compute the angle between current rotation and desired rotation
                    // Using the dot product: acos(dot(a, b) / (|a| * |b|)), but both are unit vectors, so it simplifies
                    let dot_product =
                        transform.rot.x * desired_rot.x + transform.rot.y * desired_rot.y;
                    let angle_diff = dot_product.acos() * 100.0;

                    // Determine the direction to rotate (sign of cross product's z-component)
                    let rotation_direction =
                        transform.rot.x * desired_rot.y - transform.rot.y * desired_rot.x;

                    // Set rotational velocity
                    // Here you can also add constraints, for example by multiplying with a constant factor or using min/max
                    physics.rot_vel = if rotation_direction >= 0.0 {
                        angle_diff
                    } else {
                        -angle_diff
                    };
                }
            }
        }
    }
}
