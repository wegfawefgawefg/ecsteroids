use glam::{UVec2, Vec2};
pub use legion::*;
use raylib::prelude::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CTransform {
    pub pos: Vec2,
    pub rot: Vec2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bullet;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LifeSpan {
    pub frames_left: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Asteroid {
    pub size: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputControlled;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sprite {
    pub sample_pos: UVec2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub hp: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Physics {
    pub vel: Vec2,
    pub rot_vel: f32,
}
