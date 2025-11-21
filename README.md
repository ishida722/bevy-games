# Bevy Games プロジェクト

Bevy（Rustのゲームエンジン）を使用して小さなゲームを学習しながら作成するプロジェクトです。

## プロジェクト構造

```
bevy-games/
├── Cargo.toml           # ワークスペース設定
├── common/              # 共通ライブラリ
│   ├── Cargo.toml
│   └── src/
│       ├── components/  # 共通コンポーネント
│       ├── systems/     # 共通システム
│       ├── resources/   # 共通リソース
│       └── utils/       # ユーティリティ関数
├── games/               # 個別のゲーム
│   ├── hello-bevy/      # サンプルゲーム
│   └── ...              # 他のゲーム
└── README.md
```

## セットアップ

### 必要なツール

- Rust (1.79.0以降)
- Cargo

### ビルドと実行

```bash
# ワークスペース全体のビルド
cargo build

# 特定のゲームを実行
cargo run -p hello-bevy

# リリースモードでビルド
cargo build --release
```

## 新しいゲームの追加

1. `games/`ディレクトリに新しいフォルダを作成

```bash
mkdir games/my-game
```

2. Cargo.tomlを作成

```toml
[package]
name = "my-game"
version = "0.1.0"
edition.workspace = true

[dependencies]
bevy = { workspace = true }
bevy-games-common = { path = "../../common" }
```

3. src/main.rsにゲームロジックを実装

## 共通ライブラリの使用

共通ライブラリ（`bevy-games-common`）には、複数のゲームで使用できる以下のモジュールが含まれています：

- **components**: Health、Velocity、Playerなどの共通コンポーネント
- **systems**: 移動、衝突判定、体力管理などの共通システム
- **resources**: ゲーム設定、スコア、ゲーム状態などのリソース
- **utils**: 数学関数、ランダム生成、カメラ設定などのユーティリティ

## 開発ガイドライン

1. 各ゲームは独立したプロジェクトとして管理
2. 共通機能は`common`ライブラリに実装
3. ゲーム固有の機能は各ゲームプロジェクト内に実装
4. Bevyのバージョンはワークスペースレベルで管理

## ライセンス

MIT OR Apache-2.0