mod globals;
mod screens;

use bevy::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window { title: "Tennis for Two (text)".into(), ..default() }),
            ..default()
        }))
        .init_state::<globals::AppState>()
        .add_plugins(screens::title::PluginImpl)
        .add_systems(OnEnter(globals::AppState::MainMenu), |mut next: ResMut<NextState<globals::AppState>>| next.set(globals::AppState::Game))
        .add_systems(OnEnter(globals::AppState::Game), |mut next: ResMut<NextState<globals::AppState>>| next.set(globals::AppState::Pause))
        .add_systems(OnEnter(globals::AppState::Pause), |mut next: ResMut<NextState<globals::AppState>>| next.set(globals::AppState::End))
        .add_systems(OnEnter(globals::AppState::End), exit_app)
        .run();
}

fn exit_app(mut exit: MessageWriter<AppExit>) {
    exit.write(AppExit::Success);
}
