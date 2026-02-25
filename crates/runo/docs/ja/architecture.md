# Runo Core Architecture

このドキュメントは、GUI フレームワーク初心者向けに、`runo` の内部構造を説明します。

## 1. GUI フレームワークの基本

GUI フレームワークは主に次を行います。

1. ウィンドウを作る
2. 入力イベント（マウス/キーボード）を受ける
3. 画面に何を出すかを管理する
4. GPU へ描画命令を送る
5. これをフレームごとに繰り返す

`runo` では次を利用しています。

1. `winit`: ウィンドウとイベントループ
2. `wgpu`: GPU への描画実行
3. `vello`: ベクター図形・テキスト描画

## 2. Runo の UI モデル（保持型）

`Runo` は保持型（Retained Mode）寄りの設計です。

1. ウィジェット（Button/Label/TextBox/ComboBox/Checkbox/RadioButton/Slider/Div）を内部状態に保持する
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

アプリ実行とGPUライフサイクル管理の中心です。イベントループの入口からフレーム送出までを扱います。

1. `app/mod.rs`: `RunoApplication` と `run()` の公開エントリ
2. `app/runner.rs`: `AppRunner`（window/surface/renderer/input/effects/retained）を保持
3. `app/events.rs`: `winit::ApplicationHandler` 実装。OSイベントを入力状態へ反映
4. `app/frame.rs`: フレーム処理を `surface_size` / `compose_frame` / `submit_frame` に分割
5. `app/gpu.rs`: surface取得・レンダリング・present処理とGPUエラー分類

### `retained/`

保持型UIの中核で、描画対象ノードと相互作用状態の単一ソースです。

1. `retained/node.rs`: 各ウィジェットノードのデータ構造（rect、hovered、pressed、selectedなど）
2. `retained/state/`: ノードの生成・更新・参照API（upsert/state mutation）
   `mod.rs` / `core.rs` / `button.rs` / `checkbox.rs` / `radio_button.rs` / `slider.rs` / `text_box.rs` / `combo_box.rs`
3. `retained/input/mod.rs`: 入力処理の統合入口
4. `retained/input/pointer.rs`: hover/click/drag/dropdownなどポインタ由来の状態遷移
5. `retained/input/text_box.rs`: テキスト編集、キャレット移動、スクロールバー/ホイール処理
6. `retained/paint/mod.rs`: ノード描画の統合入口
7. `retained/paint/*.rs`: ウィジェット別の描画実装（button/checkbox/combo_box/label/radio_button/slider/text_box）

### `ui/`

アプリ利用者が直接触る高水準API層です。

1. `ui/mod.rs`: `Ui` 本体。`widgets/state/events` の窓口を提供
2. `ui/widgets.rs`: `ui.widgets()` の各ビルダー入口
3. `ui/state.rs`: `ui.state()` の状態更新API（例: `combo_box().set_items(...)`）
4. `ui/events.rs`: `ui.events()` で `drain_events` / `next_event`
   `drain_actions` / `drain_bound_events` と、handle ベースの `on_*` / `*_changed` 系も提供
5. `ui/show/*.rs`: 各ウィジェットを保持状態へ反映する中継層
6. `ui/colors.rs`: 色定数 + grouped API（`Gray::gray_50()`, `Blue::blue_500()`, `Semantic::success()`）
7. `ui/mod.rs` の補助: `vertical`, `horizontal`, `use_effect`

### `widget/`

ウィジェットビルダーとレスポンス型を提供する層です。

1. `widget/button.rs`: `ButtonBuilder`, `ButtonResponse`
2. `widget/label.rs`: `LabelBuilder`
3. `widget/checkbox.rs`: `CheckboxBuilder`, `CheckboxResponse`
4. `widget/radio_button.rs`: `RadioButtonBuilder`, `RadioButtonResponse`
5. `widget/slider.rs`: `SliderBuilder`, `SliderResponse`
6. `widget/text_box.rs`: `TextBoxBuilder`, `TextBoxResponse`, `Overflow`
7. `widget/combo_box.rs`: `ComboBoxBuilder`, `ComboBoxResponse`
8. `widget/text.rs`: 文字幅推定、glyphレイアウト、テキスト描画ヘルパー

### 補助モジュール

1. `input.rs`: フレーム入力スナップショット（pressed/released/text/scrollなど）
2. `layout/mod.rs` / `layout/stack.rs` / `layout/div.rs`: レイアウト配置とコンテナ計算
3. `hooks/effect.rs`: `use_effect` の依存追跡と cleanup 管理
4. `cache/mod.rs` / `cache/text_layout.rs`: テキストレイアウトのキャッシュ
5. `event.rs`: `UiEvent` 定義
6. `font.rs`: デフォルトフォント探索・読み込み

## 6. まず読む順番（おすすめ）

1. `app/mod.rs`
2. `app/events.rs`
3. `app/frame.rs`
4. `ui/mod.rs`
5. `retained/state/mod.rs`
6. `retained/input/mod.rs`
7. `retained/paint/mod.rs`
8. `widget/button.rs`

## 7. 設計の意図

1. 役割を分離し、機能追加時の変更範囲を小さくする
2. 保持データを一元化し、描画と入力判定を安定させる
3. アプリ側コードは「UI 構築」と「状態更新」に集中させる

この構造により、`node -> state -> input -> paint -> widget` の流れで
追加ウィジェットを拡張しやすくしています。
