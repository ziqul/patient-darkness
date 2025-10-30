// --- IMPORTS ---
use bevy::prelude::*;

use crate::globals;


// --- GENERIC STRUCTS ---
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Phases {
    Start,
    RevealTitle,
    PauseBetweenRevealTitleAndRevealSubtitle,
    RevealSubtitle,
    PauseAfterRevealSubtitle,
}


// --- COMPONENTS ---

// ???


// --- MARKERS ---
#[derive(Component)]
struct StateMarker;
#[derive(Component)]
struct TitleNodeMarker;
#[derive(Component)]
struct SubtitleNodeMarker;


// --- RESOURCES ---
#[derive(Resource, Debug, Clone)]
struct Config {
    pause_before_reveal_title: f32,
    reveal_title_duration: f32,
    pause_between_reveal_title_and_reveal_subtitle: f32,
    reveal_subtitle_duration: f32,
    pause_after_reveal_subtitle: f32,
    title_size: f32,
    subtitle_size: f32,
    distance_between_title_and_subtitle: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            pause_before_reveal_title: 1.0,
            reveal_title_duration: 1.0,
            pause_between_reveal_title_and_reveal_subtitle: 1.0,
            reveal_subtitle_duration: 1.0,
            pause_after_reveal_subtitle: 10.0,
            title_size: 96.0,
            subtitle_size: 60.0,
            distance_between_title_and_subtitle: 20.0,
        }
    }
}

#[derive(Resource, Debug)]
struct Clock {
    t: f32,
}

impl Default for Clock {
    fn default() -> Self {
        Self { t: 0.0 }
    }
}

#[derive(Resource, Debug)]
struct Phase {
  ph: Phases
}

impl Default for Phase {
    fn default() -> Self {
        Self { ph: Phases::Start }
    }
}


// --- PLUGIN ---
pub struct PluginImpl;

impl Plugin for PluginImpl {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Config>()
            .init_resource::<Clock>()
            .init_resource::<Phase>()
            .add_systems(OnEnter(globals::AppState::Title), setup)
            .add_systems(Update, roll_title.run_if(in_state(globals::AppState::Title)))
            .add_systems(OnExit(globals::AppState::Title), globals::despawn_with::<StateMarker>);
    }
}


// --- SETUP ---
fn setup(
    mut commands: Commands,
    config: Res<Config>,
    asset_server: Res<AssetServer>,
    window: Query<&Window>,
) {
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
                  font_size: config.title_size,
                  ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    // [top of screens] + [title height (title size)]
                    top: Val::Px(0. - config.title_size),
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
                  font_size: config.subtitle_size,
                  ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(
                      window.single().expect("BOOM!").height() / 2. +
                      config.distance_between_title_and_subtitle / 2.
                    ),
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


// --- SYSTEMS ---

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

fn roll_title(
    // Global resources
    time: Res<Time>,
    window: Query<&Window>,
    mut next_state: ResMut<NextState<globals::AppState>>,

    // Local resources
    config: Res<Config>,
    mut clock: ResMut<Clock>,
    mut phase: ResMut<Phase>,
    mut title_query: Query<&mut Node, With<TitleNodeMarker>>,
    mut subtitle_query: Query<&mut Visibility, With<SubtitleNodeMarker>>,
) {
    clock.t += time.delta_secs();

    match phase.ph {
        Phases::Start => {
            if clock.t >= config.pause_before_reveal_title {
                phase.ph = Phases::RevealTitle;
                clock.t = 0.0;
            }
        }
        Phases::RevealTitle => {
            let mut title_node = title_query.single_mut().unwrap();
            let p = (clock.t / config.reveal_title_duration).clamp(0., 1.);
            let y = globals::lerp(
              0. - config.title_size,
              window.single().expect("BOOM!").height() / 2. - config.title_size - config.distance_between_title_and_subtitle / 2.,
              p
            );
            title_node.top = Val::Px(y);
            if p >= 1.0 {
                phase.ph = Phases::PauseBetweenRevealTitleAndRevealSubtitle;
                clock.t = 0.0;
            }
        }
        Phases::PauseBetweenRevealTitleAndRevealSubtitle => {
            if clock.t >= config.pause_between_reveal_title_and_reveal_subtitle {
                phase.ph = Phases::RevealSubtitle;
                clock.t = 0.0;
                debug!("Move to RevealSubtitle phase");
            }
        }
        Phases::RevealSubtitle => {
            debug!("Revealing subtitle");
            if let Ok(mut vis) = subtitle_query.single_mut() {
                *vis = Visibility::Visible;
            }
            if clock.t >= config.reveal_subtitle_duration {
                phase.ph = Phases::PauseAfterRevealSubtitle;
                clock.t = 0.0;
            }
        }
        Phases::PauseAfterRevealSubtitle => {
            if clock.t >= config.pause_after_reveal_subtitle {
                next_state.set(globals::AppState::MainMenu);
            }
        }
    }
}
