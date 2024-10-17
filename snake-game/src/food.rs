use bevy::{prelude::*, utils::Duration};
use rand::seq::IteratorRandom;

use crate::{
    game::{self, GameState},
    gametick::{self, game_tick_finished}, snake,
    utils::coords_to_translation,
};

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FoodTimer>()
            .add_systems(PreStartup, load_assets)
            .add_systems(
                PostUpdate,
                (update_timer, update_food)
                    .chain()
                    .run_if(in_state(GameState::InGame))
                    .run_if(game_tick_finished),
            )
            ;
    }
}

const MAX_FOOD: usize = 3;
const FOOD_GENERATION_INTERVAL: f32 = 1.0;

#[derive(Resource)]
struct FoodTexture {
    food: Handle<Image>,
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let food = asset_server.load("food.png");

    commands.insert_resource(FoodTexture { food });
}

fn update_timer(mut food_timer: ResMut<FoodTimer>, game_ticker: Res<gametick::GameTicker>) {
    food_timer.0.tick(game_ticker.duration());
}

fn update_food(
    mut commands: Commands,
    foods: Query<&Food>,
    mut food_timer: ResMut<FoodTimer>,
    snake_segments: Query<&snake::components::SnakeSegment>,
    board_size: Res<game::BoardSize>,
    food_texture: Res<FoodTexture>,
) {
    if foods.iter().len() < MAX_FOOD && food_timer.0.finished() || foods.iter().len() == 0 {
        let board = (0..(board_size.0.x as usize)).flat_map(|x| {
            (0..(board_size.0.y as usize)).map(move |y| Vec2::new(x as f32, y as f32))
        });

        let free_spaces = board.filter(|coords| {
            snake_segments
                .iter()
                .all(|segment| segment.coords != *coords)
                && foods.iter().all(|food| food.coords != *coords)
        });

        if let Some(chosen) = free_spaces.choose(&mut rand::thread_rng()) {
            commands.spawn((
                Food { coords: chosen },
                SpriteBundle {
                    texture: food_texture.food.clone(),
                    transform: Transform::from_translation(coords_to_translation(
                        board_size.0,
                        Vec2::splat(16.0),
                        chosen,
                    )),
                    ..default()
                },
            ));

            food_timer.0.reset();
        }
    }
}

#[derive(Clone, Copy, Component)]
pub struct Food {
    pub coords: Vec2,
}

#[derive(Resource)]
pub struct FoodTimer(pub Timer);

impl FromWorld for FoodTimer {
    fn from_world(_world: &mut World) -> Self {
        FoodTimer(Timer::new(
            Duration::from_secs_f32(FOOD_GENERATION_INTERVAL),
            TimerMode::Once,
        ))
    }
}
