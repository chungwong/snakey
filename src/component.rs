use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct SnakeHead {
    pub direction: Direction,
}

#[derive(Component)]
pub struct SnakeBody;

#[derive(Default)]
pub struct SnakeState(pub Vec<Entity>);

#[derive(Clone, Component, Copy, Debug, Eq, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 3, y: 3 }
    }
}

#[derive(Component, Debug)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Clone, Component, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Up
    }
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemLabel)]
pub enum SnakeAction {
    Eating,
    Growth,
    Input,
    Movement,
}

#[derive(Default)]
pub struct LastTailPosition(pub Option<Position>);

#[derive(Component)]
pub struct Food;
