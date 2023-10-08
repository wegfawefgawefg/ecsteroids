use glam::{UVec2, Vec2};
pub use legion::*;
use raylib::prelude::Color;

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

pub struct Sprite {
    pub sample_pos: UVec2,
}

pub struct Health {
    pub hp: u32,
}

pub struct Physics {
    pub vel: Vec2,
    pub rot_vel: f32,
}

pub struct CaptureInPlayField;
