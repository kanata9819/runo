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

1. ウィジェット（Button/Label/TextBox/ComboBox/Checkbox/RadioButton/Slider）を内部状態に保持する
2. 入力でその状態を更新する
3. 保持された状態を描画する

即時モードのように毎フレーム全部を作り直すのではなく、
「保持された UI ツリー」を更新しながら描画します。

## 3. RunoApplication ライフサイクル

```rust
pub trait RunoApplication {
    fn build(&mut self, _ui: &mut Ui<'_>) {}
    fn update(&mut self, _ui: &mut Ui<'_>) {}
    fn options(&self) -> RunOptions {
        RunOptions::default()
    }
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
5. `RetainedState::begin_frame_input()` で `hovered/pressed/focused/open` などを更新
6. `RunoApplication::update()` を実行
7. `RetainedState::render()` で保持されたウィジェットを描画
8. `wgpu` で swapchain に転送して表示

## 5. モジュール構成

### `app/`

アプリ実行の中心。

1. `app/mod.rs`: `RunoApplication` と `run()`
2. `app/runner.rs`: `AppRunner` と初期化状態保持
3. `app/events.rs`: `winit::ApplicationHandler` 実装
4. `app/frame.rs`: フレーム処理（`surface_size` / `compose_frame` / `submit_frame`）
5. `app/gpu.rs`: GPU 描画・surface 転送処理

### `retained/`

保持型 UI の中核。

1. `retained/node.rs`: `ButtonNode` / `LabelNode` / `TextBoxNode` / `ComboBoxNode` / `CheckboxNode` / `RadioButtonNode` / `SliderNode`
2. `retained/state.rs`: ノードの保持・更新 API（upsert）
3. `retained/input/mod.rs`: 入力から hover/click/focus/dropdown などを更新
4. `retained/paint/mod.rs`: 保持ノードの描画

### `ui/`

ユーザー向け API。

1. `widgets()`: `button`, `label`, `text_box`, `combo_box`, `checkbox`, `radio_button`, `slider`, `div`
2. `events()`: `drain_events`, `next_event`
3. `state()`: `set_text`, `set_selected_index`, `set_checked`, `set_selected`, `set_value`, `set_enabled` など
4. `vertical`, `horizontal`
5. `use_effect`

### `widget/`

各ウィジェットのビルダー。

1. `widget/button.rs`: `ButtonBuilder`, `ButtonResponse`
2. `widget/label.rs`: `LabelBuilder`
3. `widget/checkbox.rs`: `CheckboxBuilder`, `CheckboxResponse`
4. `widget/radio_button.rs`: `RadioButtonBuilder`, `RadioButtonResponse`
5. `widget/slider.rs`: `SliderBuilder`, `SliderResponse`
6. `widget/text_box.rs`: `TextBoxBuilder`, `TextBoxResponse`
7. `widget/combo_box.rs`: `ComboBoxBuilder`, `ComboBoxResponse`
8. `widget/text.rs`: テキスト計測と描画ヘルパー

### 補助モジュール

1. `input.rs`: フレーム入力状態
2. `layout/mod.rs`: 簡易レイアウト
3. `hooks/effect.rs`: `use_effect` 実装
4. `font.rs`: デフォルトフォント読み込み

## 6. まず読む順番（おすすめ）

1. `app/mod.rs`
2. `app/events.rs`
3. `app/frame.rs`
4. `ui/mod.rs`
5. `retained/state.rs`
6. `retained/input/mod.rs`
7. `retained/paint/mod.rs`
8. `widget/button.rs`

## 7. 設計の意図

1. 役割を分離し、機能追加時の変更範囲を小さくする
2. 保持データを一元化し、描画と入力判定を安定させる
3. アプリ側コードは「UI 構築」と「状態更新」に集中させる

この構造により、`node -> state -> input -> paint -> widget` の流れで
追加ウィジェットを拡張しやすくしています。
