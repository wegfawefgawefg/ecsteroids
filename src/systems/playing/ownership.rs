pub use legion::*;
use legion::{systems::CommandBuffer, world::SubWorld};

use crate::components::OwnedBy;

#[system]
#[read_component(Entity)]
#[read_component(OwnedBy)]
pub fn check_owned_by_null(ecs: &mut SubWorld, cmd: &mut CommandBuffer) {
    for (entity, owned_by) in <(Entity, &OwnedBy)>::query().iter(ecs) {
        if ecs.entry_ref(owned_by.owner).is_err() {
            cmd.remove_component::<OwnedBy>(*entity);
        }
    }
}
