use bevy::prelude::*;
use components::{Anticipating, Dir, SegmentType, SnakeSegment};
use input::{InputPlugin, InputQueue};
use std::f32;

use crate::{
    food::{Food, FoodTimer},
    game::{self, BoardSize, GameState},
    gametick::game_tick_finished,
    utils::coords_to_translation,
};

pub mod components;
mod input;

const IMAGE_SIZE: f32 = 20.0;
pub const GRID_SIZE: f32 = 16.0;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputPlugin)
            .add_systems(PreStartup, load_assets)
            .add_systems(Startup, (setup_snake, render_snake).chain())
            .add_systems(
                Update,
                (advance_snake, handle_eat, handle_collision, render_snake)
                    .chain()
                    .run_if(in_state(GameState::InGame))
                    .run_if(game_tick_finished),
            );
    }
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let suffix = "_20x20_C";
    let head = asset_server.load(format!("head{}.png", suffix));
    let body = asset_server.load(format!("body{}.png", suffix));
    let tail = asset_server.load(format!("tail{}.png", suffix));

    let suffix = "_20x20_bloated";
    let head_bloated = asset_server.load(format!("head{}.png", suffix));
    let body_bloated = asset_server.load(format!("body{}.png", suffix));
    let tail_bloated = asset_server.load(format!("tail{}.png", suffix));

    let suffix = "_20x20_anticipate";
    let head_anticipate = asset_server.load(format!("head{}.png", suffix));

    commands.insert_resource(SnakeTextures {
        head,
        body,
        tail,
        head_bloated,
        body_bloated,
        tail_bloated,
        head_anticipate,
    });
}

fn setup_snake(mut commands: Commands, board_size: Res<BoardSize>) {
    let center = (board_size.0 / 2.0).floor();
    let snake_segments = vec![
        (SnakeSegment::make_head(Dir::E, center)),
        (SnakeSegment::make_tail(Dir::E, center - Vec2::X)),
    ];

    let mut head_id = None;
    let mut body_ids = vec![];
    let mut tail_id = None;

    for (segment, tp) in snake_segments {
        let id = commands.spawn((segment, tp)).id();

        match tp {
            SegmentType::Head => head_id = Some(id),
            SegmentType::Body => body_ids.push(id),
            SegmentType::Tail => tail_id = Some(id),
        };
    }

    let snake = Snake {
        head_id: head_id.expect("Expected head segment"),
        body_ids,
        tail_id: tail_id.expect("Expected tail segment"),
    };

    commands.insert_resource(snake);
}

fn advance_snake(
    mut commands: Commands,
    mut snake: ResMut<Snake>,
    mut segments: Query<&mut SnakeSegment>,
    mut input_queue: ResMut<InputQueue>,
    board_size: Res<BoardSize>,
) {
    let mut head_segment = segments
        .get_mut(snake.head_id)
        .expect("Expected head segment");

    let mut chosen_direction: Option<Dir> = None;
    while let Some(direction) = input_queue.0.pop_front() {
        if !direction.is_parallel(head_segment.direction.0) {
            chosen_direction = Some(direction);
            break;
        }
    }

    let direction = chosen_direction.unwrap_or(head_segment.direction.1);

    let wrap_coords = |coords: Vec2| coords.rem_euclid(board_size.0);

    let mut front_segment = head_segment.clone();
    front_segment.direction = (front_segment.direction.1, direction);

    head_segment.coords = wrap_coords(head_segment.coords + Into::<Vec2>::into(direction));
    head_segment.direction = (direction, direction);
    head_segment.is_bloated = false;

    front_segment = snake
        .body_ids
        .iter_mut()
        .fold(front_segment, |mut front_segment, body_id| {
            let mut body_segment = segments.get_mut(*body_id).expect("Expected body segment");

            (front_segment, *body_segment) = (*body_segment, front_segment);
            front_segment
        });

    let mut tail_segment = segments
        .get_mut(snake.tail_id)
        .expect("Expected tail segment");

    if tail_segment.is_bloated {
        snake
            .body_ids
            .push(commands.spawn((front_segment, SegmentType::Body)).id());
        tail_segment.is_bloated = false;
    } else {
        *tail_segment = front_segment;
    }
}

fn handle_eat(
    mut commands: Commands,
    snake: Res<Snake>,
    mut segments: Query<&mut SnakeSegment>,
    foods: Query<(Entity, &Food)>,
    mut food_timer: ResMut<FoodTimer>,
) {
    let mut head_segment = segments
        .get_mut(snake.head_id)
        .expect("Expected head segment");
    if let Some((food_entity, _)) = foods
        .iter()
        .find(|(_, food)| food.coords == head_segment.coords)
    {
        commands
            .get_entity(food_entity)
            .expect("Expected food entity")
            .despawn();

        head_segment.is_bloated = true;
        food_timer.0.reset();
    }

    if foods
        .iter()
        .find(|(_, food)| {
            food.coords == head_segment.coords + Into::<Vec2>::into(head_segment.direction.1)
        })
        .is_some()
    {
        commands
            .get_entity(snake.head_id)
            .expect("Expected head entity")
            .insert(Anticipating);
    } else {
        commands
            .get_entity(snake.head_id)
            .expect("Expected head entity")
            .remove::<Anticipating>();
    }
}

fn handle_collision(
    mut next_state: ResMut<NextState<game::GameState>>,
    snake: Res<Snake>,
    segments: Query<(Entity, &SnakeSegment)>,
) {
    let (head_entity, head_segment) = segments.get(snake.head_id).expect("Expected head segment");

    if segments
        .iter()
        .find(|(entity, other)| {
            other.coords == head_segment.coords && entity.index() != head_entity.index()
        })
        .is_some()
    {
        next_state.set(game::GameState::GameOver);
    }
}

fn render_snake(
    mut commands: Commands,
    snake_textures: Res<SnakeTextures>,
    board_size: Res<BoardSize>,
    mut segments: Query<(
        Entity,
        &SnakeSegment,
        &SegmentType,
        Option<&mut Handle<Image>>,
        Option<&Anticipating>,
    )>,
) {
    for (entity, segment, tp, texture, anticipating) in segments.iter_mut() {
        if let Some(mut texture) = texture {
            if *texture != get_texture(tp, segment, &snake_textures, anticipating.is_some()) {
                *texture = get_texture(tp, segment, &snake_textures, anticipating.is_some())
            }
        } else {
            commands
                .get_entity(entity)
                .expect("Expected segment to already exist")
                .insert(SpriteBundle {
                    texture: get_texture(tp, segment, &snake_textures, anticipating.is_some()),
                    ..default()
                });
        }

        commands
            .get_entity(entity)
            .expect("Expected segment to already exist")
            .insert(
                Transform::from_translation(coords_to_translation(
                    board_size.0,
                    Vec2::splat(GRID_SIZE),
                    segment.coords,
                ))
                .with_rotation(segment.direction.0.into()),
            );
    }
}

fn get_texture(
    tp: &SegmentType,
    segment: &SnakeSegment,
    snake_textures: &Res<SnakeTextures>,
    anticipating: bool,
) -> Handle<Image> {
    match (tp, segment.is_bloated, anticipating) {
        (SegmentType::Head, true, _) => snake_textures.head_bloated.clone(),
        (SegmentType::Head, false, true) => snake_textures.head_anticipate.clone(),
        (SegmentType::Head, false, false) => snake_textures.head.clone(),
        (SegmentType::Body, true, _) => snake_textures.body_bloated.clone(),
        (SegmentType::Body, false, _) => snake_textures.body.clone(),
        (SegmentType::Tail, true, _) => snake_textures.tail_bloated.clone(),
        (SegmentType::Tail, false, _) => snake_textures.tail.clone(),
    }
}

#[derive(Resource)]
struct SnakeTextures {
    head: Handle<Image>,
    body: Handle<Image>,
    tail: Handle<Image>,
    head_bloated: Handle<Image>,
    body_bloated: Handle<Image>,
    tail_bloated: Handle<Image>,
    head_anticipate: Handle<Image>,
}

#[derive(Resource)]
pub struct Snake {
    pub head_id: Entity,
    pub body_ids: Vec<Entity>,
    pub tail_id: Entity,
}

impl Snake {
    pub fn len(&self) -> usize {
        self.body_ids.len() + 2
    }
}
