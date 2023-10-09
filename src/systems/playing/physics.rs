pub use legion::*;
use legion::{systems::CommandBuffer, world::SubWorld};

use crate::{
    components::{CTransform, CaptureInPlayField, Physics, VelocityUncapped},
    DIMS,
};

const MAX_VEL: f32 = 2.0;

#[system]
#[write_component(CTransform)]
#[write_component(Physics)]
pub fn physics(ecs: &mut SubWorld) {
    let query = <&mut Physics>::query();
    for physics in query.filter(!component::<VelocityUncapped>()).iter_mut(ecs) {
        if physics.vel.length() > MAX_VEL {
            physics.vel = physics.vel.normalize() * MAX_VEL;
        }
    }
    let mut step_query = <(&mut CTransform, &mut Physics)>::query();
    for (ctransform, physics) in step_query.iter_mut(ecs) {
        if physics.vel.length() > MAX_VEL {
            physics.vel = physics.vel.normalize() * MAX_VEL;
        }
        ctransform.pos += physics.vel;

        let rot_matrix = glam::Mat2::from_angle(physics.rot_vel.to_radians() * 0.1);
        ctransform.rot = rot_matrix * ctransform.rot;
    }
}

#[system]
#[write_component(CTransform)]
pub fn world_wrap(ecs: &mut SubWorld) {
    let mut query = <&mut CTransform>::query().filter(!component::<CaptureInPlayField>());
    for ctransform in query.iter_mut(ecs) {
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

#[system]
#[write_component(CTransform)]
#[write_component(Physics)]
pub fn capture_in_play_field(ecs: &mut SubWorld, cmd: &mut CommandBuffer) {
    let mut query = <(Entity, &mut CTransform)>::query().filter(component::<CaptureInPlayField>());
    for (entity, ctransform) in query.iter_mut(ecs) {
        let is_in_play_field = ctransform.pos.x > 0.0
            && ctransform.pos.x < DIMS.x as f32
            && ctransform.pos.y > 0.0
            && (ctransform.pos.y < DIMS.y as f32);
        if is_in_play_field {
            cmd.remove_component::<CaptureInPlayField>(*entity);
        }
    }
}
