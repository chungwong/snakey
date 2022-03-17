mod component;
mod event;
mod systems;

use bevy::core::FixedTimestep;
use bevy::prelude::*;

use crate::component::{
    LastTailPosition, SnakeAction, SnakeState,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Snakey".to_string(),
            width: 400.0,
            height: 400.0,
            ..Default::default()
        })
        .insert_resource(LastTailPosition::default())
        .insert_resource(SnakeState::default())
        .add_plugins(DefaultPlugins)
        .add_startup_system(systems::setup_camera)
        .add_startup_system(systems::spawn_snake)
        .add_event::<event::GrowthEvent>()
        .add_event::<event::GameOverEvent>()
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(systems::scale_window)
                .with_system(systems::position),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.150))
                .with_system(systems::snake_movement.label(SnakeAction::Movement))
                .with_system(
                    systems::snake_eating
                        .label(SnakeAction::Eating)
                        .after(SnakeAction::Movement),
                )
                .with_system(
                    systems::snake_growth
                        .label(SnakeAction::Growth)
                        .after(SnakeAction::Eating),
                ),
        )
        .add_system(systems::gameover.after(SnakeAction::Movement))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(systems::spanw_food),
        )
        .add_system(
            systems::snake_movement_input
                .label(SnakeAction::Input)
                .before(SnakeAction::Movement),
        )
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
