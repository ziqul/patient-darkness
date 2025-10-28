use bevy::prelude::*;


// --- GLOBAL STRUCTS ---

// ----- App states -----
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Title,
    MainMenu,
    Game,
    Pause,
    End,
}


// --- HELPER FUNCTIONS ---

// "Limear interpolation"
pub fn lerp(a: f32, b: f32, t: f32) -> f32 { a + (b - a) * t }

pub fn despawn_with<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}
