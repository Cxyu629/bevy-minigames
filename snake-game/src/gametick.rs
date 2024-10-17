use std::time::Duration;

use bevy::prelude::*;

const GAME_TICK: f32 = 0.2;

pub struct GameTickPlugin;

impl Plugin for GameTickPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTicker(Timer::new(
            Duration::from_secs_f32(GAME_TICK),
            TimerMode::Repeating,
        )))
        .add_systems(PreUpdate, update_ticker);
    }
}

fn update_ticker(mut game_ticker: ResMut<GameTicker>, time: Res<Time>) {
    game_ticker.0.tick(time.delta());
}

#[derive(Resource)]
pub struct GameTicker(Timer);

impl GameTicker {
    pub fn finished(&self) -> bool {
        self.0.finished()
    }

    pub fn duration(&self) -> Duration {
        self.0.duration()
    }
}

pub fn game_tick_finished(game_ticker: Res<GameTicker>) -> bool {
    game_ticker.finished()
}
