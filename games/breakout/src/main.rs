use bevy::prelude::*;
use bevy_games_common::prelude::*;

// Constants
const PADDLE_SPEED: f32 = 500.0;
const BALL_SPEED: f32 = 400.0;
const PADDLE_WIDTH: f32 = 120.0;
const PADDLE_HEIGHT: f32 = 20.0;
const BALL_RADIUS: f32 = 10.0;
const BLOCK_WIDTH: f32 = 80.0;
const BLOCK_HEIGHT: f32 = 30.0;
const WALL_THICKNESS: f32 = 10.0;
const BLOCK_ROWS: usize = 5;
const BLOCK_COLUMNS: usize = 8;

// Components
#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Block {
    health: i32,
}

#[derive(Component)]
struct Wall;

#[derive(Component, Clone, Copy)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Collider {
    width: f32,
    height: f32,
}

// Resources
#[derive(Resource)]
struct GameScore {
    score: u32,
}

#[derive(Resource)]
struct GameState {
    is_playing: bool,
    game_over: bool,
}

// Events
#[derive(Event)]
struct BlockDestroyedEvent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GameScore { score: 0 })
        .insert_resource(GameState {
            is_playing: false,
            game_over: false
        })
        .add_event::<BlockDestroyedEvent>()
        .add_systems(Startup, (setup_camera, setup_game, setup_ui))
        .add_systems(Update, (
            paddle_movement_system,
            ball_movement_system,
            ball_collision_system,
            block_collision_system,
            game_control_system,
            update_score_ui,
            check_game_over,
        ))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_game(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
) {
    // Spawn walls
    let wall_color = Color::srgb(0.3, 0.3, 0.3);

    // Top wall
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: wall_color,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 360.0, 0.0),
                scale: Vec3::new(800.0, WALL_THICKNESS, 1.0),
                ..default()
            },
            ..default()
        },
        Wall,
        Collider { width: 800.0, height: WALL_THICKNESS },
    ));

    // Left wall
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: wall_color,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(-400.0, 0.0, 0.0),
                scale: Vec3::new(WALL_THICKNESS, 720.0, 1.0),
                ..default()
            },
            ..default()
        },
        Wall,
        Collider { width: WALL_THICKNESS, height: 720.0 },
    ));

    // Right wall
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: wall_color,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(400.0, 0.0, 0.0),
                scale: Vec3::new(WALL_THICKNESS, 720.0, 1.0),
                ..default()
            },
            ..default()
        },
        Wall,
        Collider { width: WALL_THICKNESS, height: 720.0 },
    ));

    // Spawn paddle
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.3, 0.3, 0.9),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, -300.0, 0.0),
                scale: Vec3::new(PADDLE_WIDTH, PADDLE_HEIGHT, 1.0),
                ..default()
            },
            ..default()
        },
        Paddle,
        Collider { width: PADDLE_WIDTH, height: PADDLE_HEIGHT },
    ));

    // Spawn ball
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.9, 0.9, 0.9),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(BALL_RADIUS * 2.0, BALL_RADIUS * 2.0, 1.0),
                ..default()
            },
            ..default()
        },
        Ball,
        Velocity { x: BALL_SPEED, y: BALL_SPEED },
        Collider { width: BALL_RADIUS * 2.0, height: BALL_RADIUS * 2.0 },
    ));

    // Spawn blocks
    let colors = [
        Color::srgb(0.9, 0.3, 0.3),
        Color::srgb(0.9, 0.6, 0.3),
        Color::srgb(0.9, 0.9, 0.3),
        Color::srgb(0.3, 0.9, 0.3),
        Color::srgb(0.3, 0.3, 0.9),
    ];

    let start_x = -280.0;
    let start_y = 200.0;
    let spacing_x = BLOCK_WIDTH + 5.0;
    let spacing_y = BLOCK_HEIGHT + 5.0;

    for row in 0..BLOCK_ROWS {
        for col in 0..BLOCK_COLUMNS {
            let x = start_x + col as f32 * spacing_x;
            let y = start_y - row as f32 * spacing_y;
            let color = colors[row % colors.len()];

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(x, y, 0.0),
                        scale: Vec3::new(BLOCK_WIDTH, BLOCK_HEIGHT, 1.0),
                        ..default()
                    },
                    ..default()
                },
                Block { health: 1 },
                Collider { width: BLOCK_WIDTH, height: BLOCK_HEIGHT },
            ));
        }
    }

    game_state.is_playing = true;
}

fn setup_ui(mut commands: Commands) {
    // Score text
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        ScoreText,
    ));

    // Game over text (initially hidden)
    commands.spawn((
        TextBundle::from_section(
            "GAME OVER\nPress SPACE to restart",
            TextStyle {
                font_size: 50.0,
                color: Color::srgb(1.0, 0.3, 0.3),
                ..default()
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            display: Display::None,
            margin: UiRect::all(Val::Auto),
            ..default()
        }),
        GameOverText,
    ));
}

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct GameOverText;

fn paddle_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Paddle>>,
    time: Res<Time>,
    game_state: Res<GameState>,
) {
    if !game_state.is_playing {
        return;
    }

    let mut paddle_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
        direction -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
        direction += 1.0;
    }

    let translation = &mut paddle_transform.translation;
    translation.x += direction * PADDLE_SPEED * time.delta_seconds();
    translation.x = translation.x.clamp(-330.0, 330.0);
}

fn ball_movement_system(
    mut query: Query<(&mut Transform, &Velocity), With<Ball>>,
    time: Res<Time>,
    game_state: Res<GameState>,
) {
    if !game_state.is_playing {
        return;
    }

    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn ball_collision_system(
    mut ball_query: Query<(&mut Transform, &mut Velocity, &Collider), With<Ball>>,
    wall_query: Query<(&Transform, &Collider), (With<Wall>, Without<Ball>)>,
    paddle_query: Query<(&Transform, &Collider), (With<Paddle>, Without<Ball>)>,
    game_state: Res<GameState>,
) {
    if !game_state.is_playing {
        return;
    }

    let (mut ball_transform, mut ball_velocity, ball_collider) = ball_query.single_mut();

    // Wall collisions
    for (wall_transform, wall_collider) in wall_query.iter() {
        if check_collision(
            ball_transform.translation,
            ball_collider,
            wall_transform.translation,
            wall_collider,
        ) {
            // Top/bottom wall
            if wall_transform.translation.y.abs() > 300.0 {
                ball_velocity.y = -ball_velocity.y;
            }
            // Side walls
            else {
                ball_velocity.x = -ball_velocity.x;
            }
        }
    }

    // Paddle collision
    for (paddle_transform, paddle_collider) in paddle_query.iter() {
        if check_collision(
            ball_transform.translation,
            ball_collider,
            paddle_transform.translation,
            paddle_collider,
        ) {
            if ball_velocity.y < 0.0 {
                ball_velocity.y = -ball_velocity.y;

                // Add some spin based on where ball hits paddle
                let hit_position = (ball_transform.translation.x - paddle_transform.translation.x)
                    / (PADDLE_WIDTH / 2.0);
                ball_velocity.x = BALL_SPEED * hit_position;
            }
        }
    }
}

fn block_collision_system(
    mut commands: Commands,
    mut ball_query: Query<(&Transform, &mut Velocity, &Collider), With<Ball>>,
    block_query: Query<(Entity, &Transform, &Collider), With<Block>>,
    mut score: ResMut<GameScore>,
    mut block_destroyed: EventWriter<BlockDestroyedEvent>,
    game_state: Res<GameState>,
) {
    if !game_state.is_playing {
        return;
    }

    let (ball_transform, mut ball_velocity, ball_collider) = ball_query.single_mut();

    for (block_entity, block_transform, block_collider) in block_query.iter() {
        if check_collision(
            ball_transform.translation,
            ball_collider,
            block_transform.translation,
            block_collider,
        ) {
            commands.entity(block_entity).despawn();
            score.score += 10;
            block_destroyed.send(BlockDestroyedEvent);

            // Simple bounce logic
            let ball_center = ball_transform.translation;
            let block_center = block_transform.translation;

            let diff_x = ball_center.x - block_center.x;
            let diff_y = ball_center.y - block_center.y;

            if diff_x.abs() > diff_y.abs() {
                ball_velocity.x = -ball_velocity.x;
            } else {
                ball_velocity.y = -ball_velocity.y;
            }

            break; // Only handle one collision per frame
        }
    }
}

fn check_collision(pos1: Vec3, collider1: &Collider, pos2: Vec3, collider2: &Collider) -> bool {
    let half_width1 = collider1.width / 2.0;
    let half_height1 = collider1.height / 2.0;
    let half_width2 = collider2.width / 2.0;
    let half_height2 = collider2.height / 2.0;

    pos1.x - half_width1 < pos2.x + half_width2
        && pos1.x + half_width1 > pos2.x - half_width2
        && pos1.y - half_height1 < pos2.y + half_height2
        && pos1.y + half_height1 > pos2.y - half_height2
}

fn update_score_ui(
    score: Res<GameScore>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    if score.is_changed() {
        for mut text in query.iter_mut() {
            text.sections[1].value = score.score.to_string();
        }
    }
}

fn check_game_over(
    mut game_state: ResMut<GameState>,
    ball_query: Query<&Transform, With<Ball>>,
    block_query: Query<&Block>,
    mut game_over_text: Query<&mut Style, With<GameOverText>>,
) {
    if !game_state.is_playing {
        return;
    }

    // Check if ball fell below paddle
    if let Ok(ball_transform) = ball_query.get_single() {
        if ball_transform.translation.y < -360.0 {
            game_state.is_playing = false;
            game_state.game_over = true;

            // Show game over text
            if let Ok(mut style) = game_over_text.get_single_mut() {
                style.display = Display::Flex;
            }
        }
    }

    // Check if all blocks are destroyed (victory)
    if block_query.is_empty() {
        game_state.is_playing = false;
        game_state.game_over = true;

        // Show victory text
        if let Ok(mut style) = game_over_text.get_single_mut() {
            style.display = Display::Flex;
        }
    }
}

fn game_control_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
    entities: Query<Entity, Or<(With<Ball>, With<Block>, With<Paddle>)>>,
    mut score: ResMut<GameScore>,
    mut game_over_text: Query<&mut Style, With<GameOverText>>,
) {
    if game_state.game_over && keyboard_input.just_pressed(KeyCode::Space) {
        // Despawn all game entities
        for entity in entities.iter() {
            commands.entity(entity).despawn();
        }

        // Reset score
        score.score = 0;

        // Hide game over text
        if let Ok(mut style) = game_over_text.get_single_mut() {
            style.display = Display::None;
        }

        // Reset game state
        game_state.game_over = false;

        // Restart game
        setup_game(commands, game_state);
    }
}