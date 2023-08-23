use bevy::prelude::*;

use crate::AppState;

mod board;

/// A simple component to mark all
/// thing that is spawn in the game phase
/// so it can easily be removed on the remove
/// system.
#[derive(Component)]
struct GameComponent;

pub(crate) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), board::setup_board)
            .add_systems(
                Update,
                board::handle_keyboard_input.run_if(in_state(AppState::Game)),
            )
            .add_systems(
                Update,
                board::handle_gamepad_input.run_if(in_state(AppState::Game)),
            )
            .add_systems(
                Update,
                board::detect_end_game.run_if(in_state(AppState::Game)),
            )
            .add_systems(OnExit(AppState::Game), remove::<GameComponent>);
    }
}

fn remove<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
