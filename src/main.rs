#![warn(clippy::cargo_common_metadata)]

use anyhow::Result;
#[cfg(feature = "diagnostic")]
use bevy::diagnostic::*;
use bevy::{log::LogPlugin, prelude::*, window::WindowTheme};
#[cfg(feature = "inspector")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use game::GamePlugin;
use menu::MenuPlugin;
use resources::MazeConfig;

mod game;
mod menu;
mod resources;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum AppState {
    #[default]
    Menu,
    Game,
}

fn main() -> Result<()> {
    // Tips from https://bevy-cheatbook.github.io/features/log.html
    // this code is compiled only if debug assertions are enabled (debug mode)
    #[cfg(debug_assertions)]
    let log_plugin = LogPlugin {
        level: bevy::log::Level::DEBUG,
        filter: "info,wgpu_core=warn,wgpu_hal=warn,labyrinth=debug".into(),
    };

    #[cfg(not(debug_assertions))]
    let log_plugin = LogPlugin {
        level: bevy::log::Level::DEBUG,
        filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
    };

    // Use WinitPlugin ??
    let mut app = App::new();
    let app = app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Maze".into(),
                    window_theme: Some(WindowTheme::Dark),
                    ..default()
                }),
                ..default()
            })
            .set(log_plugin),
        RapierPhysicsPlugin::<NoUserData>::default(),
    ));

    #[cfg(feature = "inspector")]
    app.add_plugins(WorldInspectorPlugin::default());

    #[cfg(feature = "diagnostic")]
    app.add_plugins((
        LogDiagnosticsPlugin::default(),
        EntityCountDiagnosticsPlugin::default(),
        FrameTimeDiagnosticsPlugin::default(),
        SystemInformationDiagnosticsPlugin::default(),
    ));

    #[cfg(feature = "debug")]
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.add_state::<AppState>()
        .add_plugins(())
        .insert_resource(MazeConfig::default())
        .add_plugins((MenuPlugin, GamePlugin))
        .run();

    Ok(())
}
