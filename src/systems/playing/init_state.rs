use glam::Vec2;
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{
    components::{
        AttachedTo, CTransform, GrabZone, Gun, InputControlled, OwnedBy, Physics, Player,
    },
    state::State,
};

pub fn init(state: &mut State) {
    state.ecs.clear();

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

    state.ecs.push((
        CTransform {
            pos: Vec2::new(100.0, 100.0),
            rot: Vec2::new(0.0, 1.0),
        },
        Gun {
            wants_to_shoot: false,
            fire_delay: 10,
            cooldown: 0,
        },
        AttachedTo {
            entity: player,
            offset: Vec2::new(10.0, 0.0),
        },
        InputControlled,
        OwnedBy { owner: player },
    ));

    // spawn a bunch of guns attached to the player at random offsets
    // for _ in 0..10 {
    //     let mut rng: StdRng = StdRng::from_entropy();
    //     let random_offset = Vec2::new(rng.gen_range(-20.0..20.0), rng.gen_range(-20.0..20.0));
    //     state.ecs.push((
    //         CTransform {
    //             pos: Vec2::new(100.0, 100.0),
    //             rot: glam::Mat2::from_angle(rng.gen_range(0.0..360.0)) * Vec2::new(0.0, 1.0),
    //         },
    //         Gun {
    //             wants_to_shoot: false,
    //             fire_delay: rng.gen_range(1..20),
    //             cooldown: 0,
    //         },
    //         AttachedTo {
    //             entity: player,
    //             offset: random_offset,
    //         },
    //         InputControlled,
    //         OwnedBy { owner: player },
    //     ));
    // }
}
