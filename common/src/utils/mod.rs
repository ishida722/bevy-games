// 共通のユーティリティ関数をここに定義
// 例: 数学関数、ランダム生成、カメラ制御など

use bevy::prelude::*;
use rand::Rng;

pub fn random_in_range(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

pub fn random_direction() -> Vec3 {
    let mut rng = rand::thread_rng();
    let theta = rng.gen_range(0.0..std::f32::consts::TAU);
    Vec3::new(theta.cos(), 0.0, theta.sin())
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

pub fn setup_2d_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn setup_3d_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 10.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}