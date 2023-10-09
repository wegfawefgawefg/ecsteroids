pub use legion::*;
use legion::{systems::CommandBuffer, world::SubWorld};

use crate::components::{Bullet, CTransform, Gun, LifeSpan, OwnedBy, Physics, VelocityUncapped};

const BULLET_VELOCITY: f32 = 100.0;
#[system]
#[read_component(CTransform)]
#[write_component(Gun)]
#[read_component(OwnedBy)]
pub fn guns(ecs: &mut SubWorld, cmd: &mut CommandBuffer) {
    let mut query = <(&CTransform, &mut Gun, &OwnedBy)>::query();
    for (ctransform, gun, owned_by) in query.iter_mut(ecs) {
        if gun.cooldown > 0 {
            gun.cooldown -= 1;
        }

        if gun.cooldown == 0 && gun.wants_to_shoot {
            cmd.push((
                CTransform {
                    pos: ctransform.pos + ctransform.rot * 2.0,
                    rot: ctransform.rot,
                },
                Physics {
                    vel: ctransform.rot * BULLET_VELOCITY,
                    rot_vel: 0.0,
                },
                Bullet,
                OwnedBy {
                    owner: owned_by.owner,
                },
                LifeSpan { frames_left: 60 },
                VelocityUncapped,
            ));

            gun.cooldown = gun.fire_delay;
        }
    }
}
