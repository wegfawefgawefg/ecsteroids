use crate::{components::LifeSpan, message_stream::ExpiringMessages};
pub use legion::*;
use legion::{system, systems::CommandBuffer, world::SubWorld, Entity};

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
pub fn step_alerts(#[resource] expiring_messages: &mut ExpiringMessages) {
    for message in expiring_messages.iter_mut() {
        message.lifetime -= 1;
    }
    expiring_messages.retain(|message| message.lifetime > 0);
}
