use bevy::prelude::*;

// ----- App states -----
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
enum AppState {
    #[default]
    Title,
    MainMenu,
    Game,
    Pause,
    End,
}

// ----- Title flow (encapsulated) -----
#[derive(Resource, Debug, Clone)]
struct TitleConfig {
    black_hold: f32,     // seconds screen stays black
    drop_duration: f32,  // seconds for title to fall into place
    after_drop_pause: f32,
    subtitle_reveal_pause: f32,
    final_hold: f32,     // seconds before switching to MainMenu
    title_start_y: f32,  // offscreen start (above)
    title_end_y: f32,    // resting position
}

impl Default for TitleConfig {
    fn default() -> Self {
        Self {
            black_hold: 0.6,
            drop_duration: 0.7,
            after_drop_pause: 0.2,
            subtitle_reveal_pause: 0.25,
            final_hold: 0.6,
            title_start_y: 520.0,
            title_end_y: 160.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum TitlePhase {
    Black,
    DropTitle,
    PauseAfterDrop,
    RevealSubtitle,
    FinalHold,
}

#[derive(Resource, Debug)]
struct TitleClock {
    phase: TitlePhase,
    t: f32, // elapsed time inside current phase (seconds)
}

impl Default for TitleClock {
    fn default() -> Self { Self { phase: TitlePhase::Black, t: 0.0 } }
}

#[derive(Component)]
struct TitleNode;

#[derive(Component)]
struct SubtitleNode;

pub struct TitleFlowPlugin;

impl Plugin for TitleFlowPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TitleConfig>()
            .init_resource::<TitleClock>()
            .add_systems(OnEnter(AppState::Title), title_setup)
            .add_systems(Update, title_update.run_if(in_state(AppState::Title)))
            .add_systems(OnExit(AppState::Title), title_cleanup);
    }
}

fn title_setup(mut commands: Commands, config: Res<TitleConfig>, asset_server: Res<AssetServer>) {
    // UI camera is part of the default 2D/3D camera; we’ll add a general camera.
    commands.spawn(Camera2dBundle::default());

    // Title text (start offscreen)
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let sub_font = asset_server.load("fonts/FiraMono-Medium.ttf");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Start,
                    ..default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            },
        ))
        .with_children(|root| {
            root
                .spawn((
                    TextBundle::from_section(
                        "TENNIS FOR TWO",
                        TextStyle { font: font.clone(), font_size: 72.0, color: Color::WHITE },
                    )
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(-config.title_start_y), // start beyond the top (negative moves up)
                        left: Val::Auto,
                        right: Val::Auto,
                        ..default()
                    }),
                    TitleNode,
                ))
                .with_children(|_| {});

            root.spawn((
                TextBundle::from_section(
                    "a text‑only homage",
                    TextStyle { font: sub_font.clone(), font_size: 24.0, color: Color::WHITE },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(config.title_end_y + 60.0),
                    left: Val::Auto,
                    right: Val::Auto,
                    ..default()
                })
                .with_visibility(Visibility::Hidden),
                SubtitleNode,
            ));
        });
}

fn title_update(
    time: Res<Time>,
    mut clock: ResMut<TitleClock>,
    config: Res<TitleConfig>,
    mut next_state: ResMut<NextState<AppState>>,
    mut q_title: Query<&mut Style, With<TitleNode>>,
    mut q_sub: Query<&mut Visibility, With<SubtitleNode>>,
) {
    clock.t += time.delta_seconds();

    match clock.phase {
        TitlePhase::Black => {
            if clock.t >= config.black_hold {
                clock.phase = TitlePhase::DropTitle;
                clock.t = 0.0;
            }
        }
        TitlePhase::DropTitle => {
            let mut style = q_title.single_mut();
            let p = (clock.t / config.drop_duration).clamp(0.0, 1.0);
            let eased = ease_out_back(p);
            let y = lerp(-config.title_start_y, config.title_end_y, eased);
            style.top = Val::Px(y);
            if p >= 1.0 {
                clock.phase = TitlePhase::PauseAfterDrop;
                clock.t = 0.0;
            }
        }
        TitlePhase::PauseAfterDrop => {
            if clock.t >= config.after_drop_pause {
                clock.phase = TitlePhase::RevealSubtitle;
                clock.t = 0.0;
            }
        }
        TitlePhase::RevealSubtitle => {
            if let Ok(mut vis) = q_sub.get_single_mut() {
                *vis = Visibility::Visible;
            }
            if clock.t >= config.subtitle_reveal_pause {
                clock.phase = TitlePhase::FinalHold;
                clock.t = 0.0;
            }
        }
        TitlePhase::FinalHold => {
            if clock.t >= config.final_hold {
                next_state.set(AppState::MainMenu);
            }
        }
    }
}

fn title_cleanup(mut commands: Commands, q_all: Query<Entity, Or<(With<TitleNode>, With<SubtitleNode>)>>) {
    for e in &q_all { commands.entity(e).despawn_recursive(); }
}

// ----- Helpers -----
fn lerp(a: f32, b: f32, t: f32) -> f32 { a + (b - a) * t }

// A pleasant easing for drops; adjust as desired.
fn ease_out_back(t: f32) -> f32 {
    let c1 = 1.70158;
    let c3 = c1 + 1.0;
    1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
}

// ----- Main app scaffold with stateful screens -----
#[derive(Component)]
struct MenuRoot;

#[derive(Component)]
struct GameRoot;

#[derive(Component)]
struct PauseRoot;

#[derive(Component)]
struct EndRoot;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window { title: "Tennis for Two (text)".into(), ..default() }),
            ..default()
        }))
        .init_state::<AppState>()
        .add_plugins(TitleFlowPlugin)
        .add_systems(OnEnter(AppState::MainMenu), menu_enter)
        .add_systems(OnExit(AppState::MainMenu), menu_exit)
        .add_systems(OnEnter(AppState::Game), game_enter)
        .add_systems(OnExit(AppState::Game), game_exit)
        .add_systems(OnEnter(AppState::Pause), pause_enter)
        .add_systems(OnExit(AppState::Pause), pause_exit)
        .add_systems(OnEnter(AppState::End), end_enter)
        .add_systems(OnExit(AppState::End), end_exit)
        .add_systems(Update, menu_input.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, game_input)
        .run();
}

// ----- Screens (stubs) -----
fn menu_enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    commands
        .spawn((NodeBundle { style: Style { width: Val::Percent(100.0), height: Val::Percent(100.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() }, ..default() }, MenuRoot))
        .with_children(|p| {
            p.spawn(TextBundle::from_section(
                "Press Enter to Start  •  Esc to Quit",
                TextStyle { font, font_size: 24.0, color: Color::WHITE },
            ));
        });
}
fn menu_exit(mut commands: Commands, q: Query<Entity, With<MenuRoot>>) { for e in &q { commands.entity(e).despawn_recursive(); } }

fn menu_input(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<AppState>>) {
    if keys.just_pressed(KeyCode::Enter) { next.set(AppState::Game); }
}

fn game_enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    commands
        .spawn((NodeBundle { style: Style { width: Val::Percent(100.0), height: Val::Percent(100.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() }, ..default() }, GameRoot))
        .with_children(|p| {
            p.spawn(TextBundle::from_section(
                "[Game screen]  P=Pause  End=Finish",
                TextStyle { font, font_size: 24.0, color: Color::WHITE },
            ));
        });
}
fn game_exit(mut commands: Commands, q: Query<Entity, With<GameRoot>>) { for e in &q { commands.entity(e).despawn_recursive(); } }

fn game_input(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<AppState>>, state: Res<State<AppState>>) {
    if state.get() != &AppState::Game { return; }
    if keys.just_pressed(KeyCode::KeyP) { next.set(AppState::Pause); }
    if keys.just_pressed(KeyCode::End) { next.set(AppState::End); }
}

fn pause_enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    commands
        .spawn((NodeBundle { style: Style { width: Val::Percent(100.0), height: Val::Percent(100.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() }, ..default() }, PauseRoot))
        .with_children(|p| {
            p.spawn(TextBundle::from_section(
                "[Paused]  P=Resume",
                TextStyle { font, font_size: 24.0, color: Color::WHITE },
            ));
        });
}
fn pause_exit(mut commands: Commands, q: Query<Entity, With<PauseRoot>>) { for e in &q { commands.entity(e).despawn_recursive(); } }

fn end_enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    commands
        .spawn((NodeBundle { style: Style { width: Val::Percent(100.0), height: Val::Percent(100.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() }, ..default() }, EndRoot))
        .with_children(|p| {
            p.spawn(TextBundle::from_section(
                "[Game Over]  Enter=Main Menu",
                TextStyle { font, font_size: 24.0, color: Color::WHITE },
            ));
        });
}
fn end_exit(mut commands: Commands, q: Query<Entity, With<EndRoot>>) { for e in &q { commands.entity(e).despawn_recursive(); } }

// Optional: resume from Pause with P
fn pause_input(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<AppState>>) {
    if keys.just_pressed(KeyCode::KeyP) { next.set(AppState::Game); }
}
