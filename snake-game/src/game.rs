use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{food, gametick, snake};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .insert_resource(BoardSize(Vec2::splat(15.0)))
            .add_systems(Startup, draw_border)
            .add_plugins((
                gametick::GameTickPlugin,
                snake::SnakePlugin,
                food::FoodPlugin,
            ))
            .add_systems(Update, check_win);
    }
}

fn draw_border(
    mut commands: Commands,
    board_size: Res<BoardSize>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let material = ColorMaterial::from_color(Color::hsl(0.0, 0.0, 0.0));
    let mesh = Rectangle::from_size((board_size.0 + 1.0) * snake::GRID_SIZE);
    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(mesh)),
        material: materials.add(material),
        transform: Transform::from_xyz(0.0, 0.0, -1000.0),
        ..default()
    });
}

fn check_win(
    snake: Res<snake::Snake>,
    board_size: Res<BoardSize>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if snake.len() as f32 == board_size.0.element_product() {
        next_state.set(GameState::GameWin);
    }
}

#[derive(Resource)]
pub struct BoardSize(pub Vec2);

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum GameState {
    #[default]
    InGame,
    GameWin,
    GameOver,
}
