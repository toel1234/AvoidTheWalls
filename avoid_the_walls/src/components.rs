use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Obstacle;

#[derive(Component)]
pub struct PickUp;

#[derive(Component)]
pub struct TrailParticle {
    pub lifetime: Timer,
}

#[derive(Resource)]
pub struct GameState {
    pub score: i32,
    pub is_dead: bool,
    pub is_super: bool,
    pub camera_shake: f32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            score: 0,
            is_dead: false,
            is_super: false,
            camera_shake: 0.0,
        }
    }
}
