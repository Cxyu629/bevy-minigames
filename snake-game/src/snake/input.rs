use std::collections::VecDeque;

use bevy::prelude::*;

use super::components;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputQueue>()
            .add_systems(Update, handle_input);
    }
}

fn handle_input(mut input_queue: ResMut<InputQueue>, keys: Res<ButtonInput<KeyCode>>) {
    for key in keys.get_just_pressed() {
        let dir = match key {
            KeyCode::KeyA => components::Dir::W,
            KeyCode::KeyW => components::Dir::N,
            KeyCode::KeyS => components::Dir::S,
            KeyCode::KeyD => components::Dir::E,
            _ => continue,
        };

        input_queue.0.push_back(dir);
    }
}

#[derive(Resource, Default)]
pub struct InputQueue(pub VecDeque<components::Dir>);
