use legion::world::SubWorld;
pub use legion::*;

use crate::{
    components::{CTransform, Gun, InputControlled, Physics, Player},
    playing::PlayingInputs,
    state::GameMode,
};

const ROTATION_SPEED: f32 = 50.0;
const ACCELERATION: f32 = 0.04;
#[system]
#[read_component(CTransform)]
#[write_component(Physics)]
#[write_component(Gun)]
#[read_component(Player)]
pub fn handle_inputs(
    ecs: &mut SubWorld,
    #[resource] inputs: &PlayingInputs,
    #[resource] transition_to: &mut Option<GameMode>,
) {
    let mut query = <(&CTransform, &mut Physics)>::query().filter(component::<InputControlled>());
    for (ctransform, physics) in query.iter_mut(ecs) {
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
    }

    let mut query = <&mut Gun>::query().filter(component::<InputControlled>());
    for gun in query.iter_mut(ecs) {
        gun.wants_to_shoot = inputs.shoot;
    }

    // if theres no players, go back to title on shoot press
    let mut players = <(Entity, &CTransform)>::query().filter(component::<Player>());
    if players.iter(ecs).count() == 0 && inputs.shoot {
        *transition_to = Some(GameMode::GameOver);
    }
}
