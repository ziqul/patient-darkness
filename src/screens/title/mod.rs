use bevy::prelude::*;

use crate::globals;

//
// Markers
#[derive(Component)]
struct StateMarker;
#[derive(Component)]
struct TitleNodeMarker;
#[derive(Component)]
struct SubtitleNodeMarker;


#[derive(Resource, Debug, Clone)]
struct Config {
    black_hold: f32,
    drop_duration: f32,
    after_drop_pause: f32,
    subtitle_reveal_pause: f32,
    final_hold: f32,
    title_start_y: f32,
    title_end_y: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            black_hold: 1.0,
            drop_duration: 1.0,
            after_drop_pause: 1.0,
            subtitle_reveal_pause: 1.0,
            final_hold: 1.0,
            title_start_y: 520.0,
            title_end_y: 160.0,
        }
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Phases {
    Black,
    DropTitle,
    PauseAfterDrop,
    RevealSubtitle,
    FinalHold,
}


#[derive(Resource, Debug)]
struct Clock {
    phase: Phases,
    t: f32,
}

impl Default for Clock {
    fn default() -> Self {
        Self { phase: Phases::Black, t: 0.0 }
    }
}


pub struct PluginImpl;

impl Plugin for PluginImpl {
    fn build(&self, app: &mut App) {
        app.init_resource::<Config>()
            .init_resource::<Clock>()
            .add_systems(OnEnter(globals::AppState::Title), title_setup)
            .add_systems(Update, title_update.run_if(in_state(globals::AppState::Title)))
            .add_systems(OnExit(globals::AppState::Title), globals::despawn_with::<StateMarker>);
    }
}


fn title_setup(mut commands: Commands, config: Res<Config>, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, StateMarker));

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let sub_font = asset_server.load("fonts/FiraMono-Medium.ttf");

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::BLACK),
            StateMarker,
        ))
        .with_children(|root| {
            root.spawn((
                Text::new("TENNIS FOR TWO"),
                TextFont {
                  font: font.clone(),
                  font_size: 72.0,
                  ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(-config.title_start_y),
                    left: Val::Auto,
                    right: Val::Auto,
                    ..default()
                },
                TitleNodeMarker,
                StateMarker,
            ));

            root.spawn((
                Text::new("a text-only homage"),
                TextFont {
                  font: sub_font.clone(),
                  font_size: 24.0,
                  ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(config.title_end_y + 160.0),
                    left: Val::Auto,
                    right: Val::Auto,
                    ..default()
                },
                Visibility::Hidden,
                SubtitleNodeMarker,
                StateMarker,
            ));
        });
}

// fn example(
//     mut query: Query<(&Health, &mut Transform, Option<&Player>), (With<Player>, Without<Enemy>)>,
// ) {
//     for (health, mut transform, player) in &mut query {
//         // center if hp is zero
//         if health.hp <= 0.0 {
//             transform.translation = Vec3::ZERO;
//         }
//
//         if let Some(player) = player {
//             eprintln!("Player {} has {} HP.", player.name, health.hp);
//         } else {
//             eprintln!("Unknown player has {} HP.", health.hp);
//         }
//     }
// }

fn title_update(
    // Global resources
    time: Res<Time>,
    mut next_state: ResMut<NextState<globals::AppState>>,

    // Local resources
    config: Res<Config>,
    mut clock: ResMut<Clock>,
    mut title_query: Query<&mut Node, With<TitleNodeMarker>>,
    mut subtitle_query: Query<&mut Visibility, With<SubtitleNodeMarker>>,
) {
    clock.t += time.delta_secs();

    match clock.phase {
        Phases::Black => {
            if clock.t >= config.black_hold {
                clock.phase = Phases::DropTitle;
                clock.t = 0.0;
            }
        }
        Phases::DropTitle => {
            let mut title_node = title_query.single_mut().unwrap();
            let p = (clock.t / config.drop_duration).clamp(0.0, 1.0);
            let y = globals::lerp(-config.title_start_y, config.title_end_y, p);
            title_node.top = Val::Px(y);
            if p >= 1.0 {
                clock.phase = Phases::PauseAfterDrop;
                clock.t = 0.0;
            }
        }
        Phases::PauseAfterDrop => {
            if clock.t >= config.after_drop_pause {
                clock.phase = Phases::RevealSubtitle;
                clock.t = 0.0;
            }
        }
        Phases::RevealSubtitle => {
            if let Ok(mut vis) = subtitle_query.single_mut() {
                *vis = Visibility::Visible;
            }
            if clock.t >= config.subtitle_reveal_pause {
                clock.phase = Phases::FinalHold;
                clock.t = 0.0;
            }
        }
        Phases::FinalHold => {
            if clock.t >= config.final_hold {
                next_state.set(globals::AppState::MainMenu);
            }
        }
    }
}
