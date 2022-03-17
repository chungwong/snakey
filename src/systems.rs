use bevy::prelude::*;
use rand::prelude::random;

use crate::component::{
    Direction, Food, LastTailPosition, Position, Size, SnakeBody, SnakeHead,
    SnakeState,
};
use crate::event::{GameOverEvent, GrowthEvent};

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const SNAKE_BODY_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0); // <--

pub fn spawn_snake(mut cmd: Commands, mut snake_state: ResMut<SnakeState>) {
    snake_state.0 = vec![
        cmd.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SnakeHead::default())
        .insert(SnakeBody)
        .insert(Position::default())
        .insert(Size::square(0.8))
        .id(),
        spawn_snake_body(cmd, Position { x: 3, y: 2 }),
    ];
}

pub fn spawn_snake_body(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_BODY_COLOR,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SnakeBody)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}

pub fn snake_movement(
    snake_state: ResMut<SnakeState>,
    mut gameover_writer: EventWriter<GameOverEvent>,
    mut snake_heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
    mut last_tail_position: ResMut<LastTailPosition>,
) {
    if let Ok((head_entity, snake_head)) = snake_heads.get_single_mut() {
        let segment_positions = snake_state
            .0
            .iter()
            .map(|entity| *positions.get_mut(*entity).unwrap())
            .collect::<Vec<Position>>();

        last_tail_position.0 = Some(*segment_positions.last().unwrap());

        let mut head_pos = positions.get_mut(head_entity).unwrap();

        match snake_head.direction {
            Direction::Up => head_pos.y += 1,
            Direction::Down => head_pos.y -= 1,
            Direction::Left => head_pos.x -= 1,
            Direction::Right => head_pos.x += 1,
        };

        if !(0..ARENA_WIDTH).contains(&(head_pos.x as u32))
            || !(0..ARENA_HEIGHT).contains(&(head_pos.y as u32))
        {
            gameover_writer.send(GameOverEvent);
        }

        segment_positions
            .iter()
            .zip(snake_state.0.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });
    }
}

pub fn snake_movement_input(keys: Res<Input<KeyCode>>, mut q: Query<&mut SnakeHead>) {
    // if let Ok(mut snake_head) = q.get_single_mut() {
    if let Some(mut snake_head) = q.iter_mut().next() {
        let dir = if keys.pressed(KeyCode::Up) {
            Direction::Up
        } else if keys.pressed(KeyCode::Down) {
            Direction::Down
        } else if keys.pressed(KeyCode::Left) {
            Direction::Left
        } else if keys.pressed(KeyCode::Right) {
            Direction::Right
        } else {
            snake_head.direction
        };

        if dir != snake_head.direction.opposite() {
            snake_head.direction = dir
        }
    } else {
        println!("not found {:?}", q.get_single_mut());
    }
}

pub fn snake_eating(
    mut cmd: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_position: Query<&Position, With<SnakeHead>>,
) {
    if let Ok(head_pos) = head_position.get_single() {
        for (entity, pos) in food_positions.iter() {
            if pos == head_pos {
                cmd.entity(entity).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

pub fn snake_growth(
    cmd: Commands,
    last_tail_pos: Res<LastTailPosition>,
    mut snake_state: ResMut<SnakeState>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    if growth_reader.iter().next().is_some() {
        snake_state
            .0
            .push(spawn_snake_body(cmd, last_tail_pos.0.unwrap()));
    }
}

pub fn setup_camera(mut cmd: Commands) {
    cmd.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub fn scale_window(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();

    for (size, mut trans) in q.iter_mut() {
        trans.scale = Vec3::new(
            size.width / ARENA_WIDTH as f32 * window.width(),
            size.height / ARENA_HEIGHT as f32 * window.height(),
            1.0,
        );
    }
}

pub fn convert(pos: f32, bound_window: f32, bound_arean: f32) -> f32 {
    let tile_size = bound_window / bound_arean;

    // pos / grid_size
    pos / bound_arean * bound_window - (bound_window / 2.) + (tile_size / 2.)
}

pub fn position(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    let window = windows.get_primary().unwrap();

    for (pos, mut trans) in q.iter_mut() {
        trans.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

pub fn spanw_food(mut cmd: Commands) {
    cmd.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: FOOD_COLOR,
            ..Default::default()
        },
        transform: Transform {
            scale: Vec3::new(10.0, 10.0, 10.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Food)
    .insert(Size::square(0.6))
    .insert(Position {
        x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
        y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
    });
}

pub fn gameover(
    mut cmd: Commands,
    mut gameover_reader: EventReader<GameOverEvent>,
    snake_state: ResMut<SnakeState>,
    food: Query<Entity, With<Food>>,
    bodies: Query<Entity, With<SnakeBody>>,
) {
    if gameover_reader.iter().next().is_some() {
        for entity in food.iter().chain(bodies.iter()) {
            cmd.entity(entity).despawn();
        }
        spawn_snake(cmd, snake_state);
    }
}
