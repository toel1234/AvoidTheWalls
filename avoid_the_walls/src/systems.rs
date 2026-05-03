use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::*;
use crate::components::Velocity as MyVelocity;
use bevy_rapier2d::prelude::Velocity as RapierVelocity;
use rand::Rng;
use bevy::core_pipeline::bloom::BloomSettings;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct MainCamera;

pub fn player_movement_system(
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut player_query: Query<(&mut Transform, &mut MyVelocity, &mut RapierVelocity), With<Player>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut game_state: ResMut<GameState>,
) {
    if game_state.is_dead { return; }

    let Ok((camera, camera_transform)) = camera_query.get_single() else { return };
    let Ok(window) = window_query.get_single() else { return };

    if let Ok((mut transform, mut my_vel, mut rapier_vel)) = player_query.get_single_mut() {
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            // Smooth horizontal interpolation for "weight"
            transform.translation.x = transform.translation.x + (world_position.x - transform.translation.x) * 0.2;
        }

        let base_yspeed = -500.0;
        let super_multiplier = 3.5;

        if mouse_button_input.pressed(MouseButton::Left) {
            game_state.is_super = true;
            my_vel.0.y = base_yspeed * super_multiplier;
            game_state.camera_shake = 2.0;
        } else {
            game_state.is_super = false;
            my_vel.0.y = base_yspeed;
            game_state.camera_shake = (game_state.camera_shake - 0.1).max(0.0);
        }

        rapier_vel.linvel = Vec2::new(0.0, my_vel.0.y);
    }
}

pub fn trail_system(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    game_state: Res<GameState>,
) {
    if game_state.is_dead { return; }
    if let Ok(transform) = player_query.get_single() {
        let spawn_chance = if game_state.is_super { 1.0 } else { 0.3 };
        let mut rng = rand::thread_rng();
        
        if rng.gen_bool(spawn_chance as f64) {
            let color = if game_state.is_super {
                Color::srgb(0.0, 1.0, 1.0) // Cyan trail
            } else {
                Color::srgb(0.2, 1.0, 0.2) // Green trail
            };

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::splat(20.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(transform.translation),
                    ..default()
                },
                TrailParticle {
                    lifetime: Timer::from_seconds(0.5, TimerMode::Once),
                },
            ));
        }
    }
}

pub fn particle_update_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TrailParticle, &mut Sprite, &mut Transform)>,
    time: Res<Time>,
) {
    for (entity, mut particle, mut sprite, mut transform) in query.iter_mut() {
        particle.lifetime.tick(time.delta());
        if particle.lifetime.finished() {
            commands.entity(entity).despawn_recursive();
        } else {
            let pct = particle.lifetime.fraction_remaining();
            sprite.color.set_alpha(pct);
            transform.scale = Vec3::splat(pct);
        }
    }
}

pub fn camera_system(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    game_state: Res<GameState>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            let mut rng = rand::thread_rng();
            let shake = if game_state.camera_shake > 0.0 {
                Vec3::new(
                    rng.gen_range(-1.0..1.0) * game_state.camera_shake,
                    rng.gen_range(-1.0..1.0) * game_state.camera_shake,
                    0.0
                )
            } else {
                Vec3::ZERO
            };

            let target_y = player_transform.translation.y - 250.0;
            camera_transform.translation.y = camera_transform.translation.y + (target_y - camera_transform.translation.y) * 0.1;
            camera_transform.translation += shake;
        }
    }
}

pub fn level_generation_system(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    mut last_y: Local<f32>,
    game_state: Res<GameState>,
) {
    if game_state.is_dead { return; }

    if let Ok(player_transform) = player_query.get_single() {
        let player_y = player_transform.translation.y;
        
        if player_y < *last_y - 250.0 {
            *last_y -= 250.0;
            let mut rng = rand::thread_rng();
            
            // Neon Obstacles
            for _ in 0..rng.gen_range(1..3) {
                let x_offset = rng.gen_range(-500.0..500.0);
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb(2.0, 0.1, 0.5), // Glowing Magenta
                            custom_size: Some(Vec2::splat(80.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(x_offset, *last_y - 1200.0, 0.0),
                        ..default()
                    },
                    Collider::cuboid(40.0, 40.0),
                    Obstacle,
                ));
            }

            // Neon Pickups
            if rng.gen_bool(0.4) {
                let pickup_x = rng.gen_range(-500.0..500.0);
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb(0.1, 2.0, 2.0), // Glowing Cyan
                            custom_size: Some(Vec2::splat(30.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(pickup_x, *last_y - 900.0, 0.0),
                        ..default()
                    },
                    Collider::ball(15.0),
                    Sensor,
                    PickUp,
                ));
            }
        }
    }
}

pub fn collision_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut game_state: ResMut<GameState>,
    player_query: Query<Entity, With<Player>>,
    obstacle_query: Query<Entity, With<Obstacle>>,
    pickup_query: Query<Entity, With<PickUp>>,
) {
    if game_state.is_dead { return; }
    let Ok(player_entity) = player_query.get_single() else { return };
    
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                let other = if *e1 == player_entity { *e2 } else if *e2 == player_entity { *e1 } else { continue };

                if obstacle_query.get(other).is_ok() {
                    game_state.is_dead = true;
                    game_state.camera_shake = 10.0;
                } else if pickup_query.get(other).is_ok() {
                    game_state.score += if game_state.is_super { 10 } else { 1 };
                    commands.entity(other).despawn_recursive();
                }
            }
            _ => {}
        }
    }
}

pub fn ui_system(
    game_state: Res<GameState>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    for mut text in query.iter_mut() {
        if game_state.is_dead {
            text.sections[0].value = format!("CRITICAL FAILURE\nFINAL SCORE: {}\n[R] TO REBOOT SYSTEM", game_state.score);
            text.sections[0].style.color = Color::srgb(1.0, 0.0, 0.0);
        } else {
            text.sections[0].value = format!("NEURAL LINK: STABLE\nSCORE: {:06}", game_state.score);
            text.sections[0].style.color = if game_state.is_super { Color::srgb(0.0, 1.0, 1.0) } else { Color::WHITE };
        }
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 2026 Bloom & Post-processing
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        BloomSettings::NATURAL,
        MainCamera,
    ));

    setup_game(&mut commands);
    
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font: asset_server.load("Fonts/joystix monospace.ttf"),
                font_size: 32.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        }),
        ScoreText,
    ));
}

fn setup_game(commands: &mut Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.5, 2.0, 0.5), // Neon Green
                custom_size: Some(Vec2::splat(50.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Player,
        MyVelocity(Vec2::ZERO),
        RigidBody::Dynamic,
        RapierVelocity::default(),
        Collider::ball(25.0),
        LockedAxes::ROTATION_LOCKED,
        ActiveEvents::COLLISION_EVENTS,
    ));

    // Start with a dense field
    for i in 0..15 {
        let mut rng = rand::thread_rng();
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(2.0, 0.1, 0.5),
                    custom_size: Some(Vec2::splat(80.0)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    rng.gen_range(-600.0..600.0),
                    -600.0 - (i as f32 * 300.0),
                    0.0
                ),
                ..default()
            },
            Collider::cuboid(40.0, 40.0),
            Obstacle,
        ));
    }
}

pub fn restart_system(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, Or<(With<Player>, With<Obstacle>, With<PickUp>, With<TrailParticle>)>>,
) {
    if game_state.is_dead && keyboard_input.just_pressed(KeyCode::KeyR) {
        *game_state = GameState::default();
        for entity in query.iter() { commands.entity(entity).despawn_recursive(); }
        setup_game(&mut commands);
    }
}
