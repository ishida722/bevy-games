// 共通のリソースをここに定義
// 例: ゲーム設定、スコア、ゲーム状態など

use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct GameSettings {
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub music_volume: f32,
    pub difficulty: Difficulty,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            sfx_volume: 1.0,
            music_volume: 0.7,
            difficulty: Difficulty::Normal,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

#[derive(Resource, Debug, Default)]
pub struct Score {
    pub current: u32,
    pub high_score: u32,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    Paused,
    GameOver,
}