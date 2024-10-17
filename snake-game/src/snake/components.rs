use bevy::prelude::*;
use std::f32;

#[derive(Component, Clone, Copy)]
pub struct SnakeSegment {
    pub coords: Vec2,
    pub direction: (Dir, Dir),
    pub is_bloated: bool,
}

impl SnakeSegment {
    pub fn make_head(dir: Dir, coords: Vec2) -> (Self, SegmentType) {
        (
            Self {
                direction: (dir, dir),
                coords,
                is_bloated: false,
            },
            SegmentType::Head,
        )
    }

    pub fn make_tail(dir: Dir, coords: Vec2) -> (Self, SegmentType) {
        (
            Self {
                direction: (dir, dir),
                coords,
                is_bloated: false,
            },
            SegmentType::Tail,
        )
    }

    pub fn make_body(from: Dir, to: Dir, coords: Vec2) -> (Self, SegmentType) {
        (
            Self {
                direction: (from, to),
                coords,
                is_bloated: false,
            },
            SegmentType::Body,
        )
    }
}

#[derive(Component, Clone, Copy)]
pub enum SegmentType {
    Head,
    Body,
    Tail,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    pub fn is_parallel(&self, other: Self) -> bool {
        match self {
            Dir::N | Dir::S => matches!(other, Dir::N | Dir::S),
            Dir::E | Dir::W => matches!(other, Dir::E | Dir::W),
        }
    }
}

impl Into<Quat> for Dir {
    fn into(self) -> Quat {
        Quat::from_rotation_z(match self {
            Dir::N => f32::consts::FRAC_PI_2,
            Dir::E => 0.0,
            Dir::S => 3.0 * f32::consts::FRAC_PI_2,
            Dir::W => f32::consts::PI,
        })
    }
}

impl Into<Vec2> for Dir {
    fn into(self) -> Vec2 {
        match self {
            Dir::N => Vec2::new(0.0, 1.0),
            Dir::E => Vec2::new(1.0, 0.0),
            Dir::S => Vec2::new(0.0, -1.0),
            Dir::W => Vec2::new(-1.0, 0.0),
        }
    }
}

#[derive(Component)]
pub struct Anticipating;