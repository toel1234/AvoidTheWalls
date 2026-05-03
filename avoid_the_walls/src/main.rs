use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod components;
mod systems;

use components::GameState;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .init_resource::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            player_movement_system, 
            camera_system,
            level_generation_system,
            collision_system,
            ui_system,
            restart_system,
            trail_system,
            particle_update_system
        ))
        .run();
}
