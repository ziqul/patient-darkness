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
//        .add_systems(Update, draw_coordinates)
        .run();
}

fn exit_app(mut exit: MessageWriter<AppExit>) {
    exit.write(AppExit::Success);
}

fn draw_coordinates(mut gizmos: Gizmos) {
    // Draw axes centered at origin
    gizmos.line(Vec3::ZERO, Vec3::X + 1000.0, Color::srgb(1., 0., 0.)); // X-axis
    gizmos.line(Vec3::ZERO, Vec3::Y * 1000.0, Color::srgb(0., 1., 0.)); // Y-axis
    gizmos.line(Vec3::ZERO, Vec3::Z * 1000.0, Color::srgb(0., 0., 1.)); // Z-axis
}
