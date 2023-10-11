pub use legion::*;
use rand::{rngs::StdRng, SeedableRng};

use crate::{
    audio_playing::AudioCommandBuffer,
    message_stream::ExpiringMessages,
    rendering::RenderCommandBuffer,
    schedules,
    timer::{AsteroidSpawnTimer, EnemySpawnTimer, GunSpawnTimer},
};

pub const FRAMES_PER_SECOND: u32 = 60;

#[derive(Clone, Copy)]
pub enum GameMode {
    Title,
    Playing,
    GameOver,
}

pub struct State {
    pub running: bool,
    pub time_since_last_update: f32,

    pub ecs: World,
    pub resources: Resources,
    pub title_schedule: Schedule,
    pub playing_schedule: Schedule,
}

impl State {
    pub fn new() -> Self {
        //////////////////    INIT RESOURCES    //////////////////
        let mut resources = Resources::default();

        let render_command_buffer: RenderCommandBuffer = RenderCommandBuffer::new();
        resources.insert(render_command_buffer);

        let audio_command_buffer: AudioCommandBuffer = AudioCommandBuffer::new();
        resources.insert(audio_command_buffer);

        let expiring_messages = ExpiringMessages::new();
        resources.insert(expiring_messages);

        let rng: StdRng = StdRng::from_entropy();
        resources.insert(rng);

        let asteroid_spawn_timer = AsteroidSpawnTimer::new(500, 0);
        resources.insert::<AsteroidSpawnTimer>(asteroid_spawn_timer);

        let gun_spawn_timer = GunSpawnTimer::new(100, 0);
        resources.insert::<GunSpawnTimer>(gun_spawn_timer);

        let enemy_spawn_timer = EnemySpawnTimer::new(2000, 0);
        resources.insert::<EnemySpawnTimer>(enemy_spawn_timer);

        let game_mode = GameMode::Title;
        resources.insert(game_mode);
        let transition_to: Option<GameMode> = None;
        resources.insert(transition_to);

        Self {
            running: true,
            time_since_last_update: 0.0,

            ecs: World::default(),
            resources,
            title_schedule: schedules::build_title_schedule(),
            playing_schedule: schedules::build_play_schedule(),
        }
    }
}
