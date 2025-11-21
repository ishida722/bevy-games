// 共通のコンポーネントをここに定義
// 例: プレイヤー、敵、アイテムなどゲーム間で共有できるコンポーネント

use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }

    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }
}

#[derive(Component, Debug, Clone)]
pub struct Velocity {
    pub linear: Vec3,
    pub angular: Vec3,
}

#[derive(Component, Debug)]
pub struct Player {
    pub id: usize,
    pub name: String,
}