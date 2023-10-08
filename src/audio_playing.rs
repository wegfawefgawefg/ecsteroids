use glam::{UVec2, Vec2};
use rand::Rng;
use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, RaylibTextureMode, Vector2};

pub type AudioCommandBuffer = Vec<DrawCommand>;

#[derive(Clone)]
pub enum AudioCommand {
    AsteroidExplosion,
    Shoot,
    PlayerExplosion,
    PlayerHit,
}

pub fn execute_audio_command_buffer(
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
    audio_command_buffer: &mut AudioCommandBuffer,
) {
    let mut rng = rand::thread_rng();
    for command in render_command_buffer.iter() {
        match command {}
    }
}
