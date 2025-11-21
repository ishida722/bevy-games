// 共通のシステムをここに定義
// 例: 移動、衝突判定、入力処理などのシステム

use bevy::prelude::*;
use crate::components::*;

pub fn apply_velocity(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform)>,
) {
    for (velocity, mut transform) in &mut query {
        transform.translation += velocity.linear * time.delta_seconds();
        transform.rotation *= Quat::from_euler(
            EulerRot::XYZ,
            velocity.angular.x * time.delta_seconds(),
            velocity.angular.y * time.delta_seconds(),
            velocity.angular.z * time.delta_seconds(),
        );
    }
}

pub fn health_system(
    mut commands: Commands,
    query: Query<(Entity, &Health)>,
) {
    for (entity, health) in &query {
        if health.is_dead() {
            commands.entity(entity).despawn();
        }
    }
}