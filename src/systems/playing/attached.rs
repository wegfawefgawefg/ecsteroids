use glam::Vec2;
pub use legion::*;
use legion::{system, systems::CommandBuffer, world::SubWorld};

use crate::components::{AttachedTo, CTransform, Physics};

#[system]
#[write_component(CTransform)]
#[read_component(Physics)]
#[read_component(AttachedTo)]
pub fn stick_to_attached(ecs: &mut SubWorld) {
    let mut query = <(Entity, &AttachedTo)>::query();
    let entities_to_update: Vec<(Entity, &AttachedTo)> = query
        .iter(ecs)
        .map(|(entity, attached_to)| (*entity, attached_to))
        .collect();

    let mut new_positions: Vec<(Entity, CTransform, Vec2)> = vec![];
    for (entity, attached_to) in entities_to_update {
        // fetch the attached entity
        if let Ok(at_entry) = ecs.entry_ref(attached_to.entity) {
            if let Ok(at_ctransform) = at_entry.get_component::<CTransform>() {
                new_positions.push((entity, *at_ctransform, attached_to.offset));
            }
        }
    }

    for (entity, new_ctransform, offset) in new_positions {
        if let Ok(mut entry) = ecs.entry_mut(entity) {
            if let Ok(ctransform) = entry.get_component_mut::<CTransform>() {
                let rot_angle = new_ctransform.rot.y.atan2(new_ctransform.rot.x);
                let rotation_matrix = glam::Mat2::from_angle(rot_angle);
                let rotated_offset = rotation_matrix * offset;
                ctransform.pos = new_ctransform.pos + rotated_offset;
                ctransform.rot = new_ctransform.rot.normalize();
            }
        }
    }
}

#[system]
#[read_component(Entity)]
#[read_component(AttachedTo)]
pub fn check_attached_to_null(ecs: &mut SubWorld, cmd: &mut CommandBuffer) {
    for (entity, owned_by) in <(Entity, &AttachedTo)>::query().iter(ecs) {
        if ecs.entry_ref(owned_by.entity).is_err() {
            cmd.remove_component::<AttachedTo>(*entity);
        }
    }
}
