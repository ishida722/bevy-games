use bevy::prelude::*;
use bevy_games_common::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .init_resource::<GameSettings>()
        .init_resource::<Score>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            rotate_cube,
            apply_velocity,
            health_system,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // カメラ
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // ライト
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // 回転するキューブ
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.5, 0.7, 1.0),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        RotatingCube,
        Health::new(100.0),
        Velocity {
            linear: Vec3::ZERO,
            angular: Vec3::new(0.0, 1.0, 0.0),
        },
    ));

    // 床
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            ..default()
        }),
        ..default()
    });

    // テキスト表示
    commands.spawn(
        TextBundle::from_section(
            "Hello Bevy!",
            TextStyle {
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
}

#[derive(Component)]
struct RotatingCube;

fn rotate_cube(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<RotatingCube>>,
) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() * 0.5);
    }
}