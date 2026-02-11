# Runo Core Architecture

このドキュメントは、GUI フレームワーク初心者向けに、`runo_core` の内部構造を説明します。

## 1. GUI フレームワークの基本

GUI フレームワークは主に次を行います。

1. ウィンドウを作る
2. 入力イベント（マウス/キーボード）を受ける
3. 画面に何を出すかを管理する
4. GPU へ描画命令を送る
5. これをフレームごとに繰り返す

`runo_core` では次を利用しています。

1. `winit`: ウィンドウとイベントループ
2. `wgpu`: GPU への描画実行
3. `vello`: ベクター図形・テキスト描画

## 2. Runo の UI モデル（保持型）

`Runo` は保持型（Retained Mode）寄りの設計です。

1. ウィジェット（Button/Label）を内部状態に保持する
2. 入力でその状態を更新する
3. 保持された状態を描画する

即時モードのように毎フレーム全部を作り直すのではなく、
「保持された UI ツリー」を更新しながら描画します。

## 3. Application ライフサイクル

```rust
pub trait Application {
    fn build(&mut self, _ui: &mut Ui<'_>) {}
    fn update(&mut self, _ui: &mut Ui<'_>) {}
}
```

1. `build`
   初期 UI を構築するフェーズ（通常は最初の 1 回）
2. `update`
   毎フレーム呼ばれ、入力に応じて状態を更新するフェーズ

## 4. 1 フレームの処理フロー

1. `winit` がイベントを配信
2. `InputState` が入力状態を更新
3. `render()` 開始
4. 背景を描画
5. `RetainedState::begin_frame_input()` で `hovered/pressed/clicked` を更新
6. `Application::update()` を実行
7. `RetainedState::render()` で保持されたウィジェットを描画
8. `wgpu` で swapchain に転送して表示

## 5. モジュール構成

### `app/`

アプリ実行の中心。

1. `app/mod.rs`: `Application` と `run()`
2. `app/runner.rs`: `AppRunner` と初期化状態保持
3. `app/events.rs`: `winit::ApplicationHandler` 実装
4. `app/frame.rs`: フレーム処理（UI 更新と描画準備）
5. `app/gpu.rs`: GPU 描画・surface 転送処理

### `retained/`

保持型 UI の中核。

1. `retained/node.rs`: `ButtonNode` / `LabelNode` などノード定義
2. `retained/state.rs`: ノードの保持・更新 API（upsert）
3. `retained/input.rs`: 入力からボタン状態を更新
4. `retained/paint.rs`: 保持ノードの描画

### `ui.rs`

ユーザー向け API。

1. `button`, `label`, `text_box`, `vertical`, `horizontal`
2. `button_clicked`, `button_state`, `set_button_text`
3. `use_effect`

### `widget/`

各ウィジェットのビルダー。

1. `widget/button.rs`: `ButtonBuilder`, `ButtonResponse`
2. `widget/label.rs`: `LabelBuilder`
3. `widget/text.rs`: テキスト計測と描画ヘルパー

### 補助モジュール

1. `input.rs`: フレーム入力状態
2. `layout/mod.rs`: 簡易レイアウト
3. `hooks/effect.rs`: `use_effect` 実装
4. `font.rs`: デフォルトフォント読み込み

## 6. まず読む順番（おすすめ）

1. `app/mod.rs`
2. `app/events.rs`
3. `app/frame.rs`
4. `ui.rs`
5. `retained/state.rs`
6. `retained/input.rs`
7. `retained/paint.rs`
8. `widget/button.rs`

## 7. 設計の意図

1. 役割を分離し、機能追加時の変更範囲を小さくする
2. 保持データを一元化し、描画と入力判定を安定させる
3. アプリ側コードは「UI 構築」と「状態更新」に集中させる

この構造により、今後 `TextField` / `Checkbox` / `Slider` を追加する場合も、
`node -> state -> input -> paint -> widget` の流れで拡張できます。
