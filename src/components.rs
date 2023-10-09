use glam::Vec2;
pub use legion::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CTransform {
    pub pos: Vec2,
    pub rot: Vec2,
}

pub struct Player;

pub struct Gun {
    pub wants_to_shoot: bool,
    pub fire_delay: u32,
    pub cooldown: u32,
}

pub struct Bullet;

pub struct LifeSpan {
    pub frames_left: u32,
}

pub struct Asteroid {
    pub size: u32,
}

pub struct InputControlled;

pub struct _Health {
    pub hp: u32,
}

pub struct VelocityUncapped;

pub struct Physics {
    pub vel: Vec2,
    pub rot_vel: f32,
}

pub struct CaptureInPlayField;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Score {
    pub owner: Entity,
    pub score: u32,
}

pub struct OwnedBy {
    pub owner: Entity,
}

pub struct AttachedTo {
    pub entity: Entity,
    pub offset: Vec2,
}
