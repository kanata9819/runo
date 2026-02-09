# Runo Core README

この README は、GUI フレームワーク初心者向けに、`runo_core` の仕組みをざっくり理解するための説明です。

## 1. まず GUI フレームワークって何をしているの？

GUI フレームワークは、だいたい次の仕事をしています。

1. ウィンドウを作る  
2. マウス・キーボード入力を受ける  
3. 「何を表示するか」を管理する  
4. 画面に描画する  
5. これを毎フレーム繰り返す

`runo_core` では、以下のライブラリを使っています。

1. `winit`  
   ウィンドウ作成とイベント受け取り（マウス移動、クリック、リサイズなど）
2. `wgpu`  
   GPU に描画命令を送る低レベル API
3. `vello`  
   ベクター描画（四角、角丸、テキストなど）を扱いやすくする層

## 2. 保持型（Retained Mode）とは？

GUI には大きく 2 パターンあります。

1. 即時モード（Immediate Mode）  
   毎フレーム「ボタンを描く」「ラベルを描く」を全部書き直す
2. 保持モード（Retained Mode）  
   ボタンやラベルを内部に保持して、状態変化に応じて更新して描画する

`Runo` は保持モード寄りにしています。  
具体的には、`RetainedState` が「どんなウィジェットがあるか」を持ち続けます。

## 3. Application の `build` と `update`

`Application` トレイト:

```rust
pub trait Application {
    fn build(&mut self, _ui: &mut Ui<'_>) {}
    fn update(&mut self, _ui: &mut Ui<'_>) {}
}
```

役割は次の通りです。

1. `build`  
   初期 UI を作る場所です（基本 1 回）。  
   例: `button_id("main.toggle").show();`
2. `update`  
   毎フレーム呼ばれて、入力に応じて状態更新します。  
   例: `if ui.button_clicked("main.toggle") { ... }`

## 4. 1フレームの流れ（ざっくり）

1. `winit` がイベントを運ぶ  
2. `InputState` が入力状態を更新する  
3. `render()` が呼ばれる  
4. 背景を描く  
5. `RetainedState` が入力を処理して各ボタンの `hovered/pressed/clicked` を更新  
6. `Application::update()` でアプリ状態を更新  
7. `RetainedState` に保持されたウィジェットを描画  
8. `wgpu` で画面へ転送・表示

## 5. モジュールごとの役割

### `app/`

アプリ実行の「司令塔」です。

1. `app/mod.rs`  
   `Application` トレイトと `run()` の入口
2. `app/runner.rs`  
   `AppRunner` 本体（window / renderer / scene / input など保持）
3. `app/events.rs`  
   `winit` のイベント処理（クリック、カーソル移動、リサイズ）
4. `app/frame.rs`  
   1フレームの処理手順（UI更新と描画準備）
5. `app/gpu.rs`  
   GPU 描画実行（`render_to_texture`、surface への転送）

### `retained/`

保持型 UI の中核です。

1. `retained/state.rs`  
   保持データ本体。ボタン/ラベルの登録・更新（upsert）を担当
2. `retained/node.rs`  
   保持されるウィジェット構造体（`ButtonNode`, `LabelNode`）
3. `retained/input.rs`  
   入力からボタン状態を更新（hovered/pressed/clicked）
4. `retained/paint.rs`  
   保持されたウィジェットを `Scene` に描画

### `ui.rs`

ユーザーが触る API 層です。  
`button_id(...)`, `label(...)`, `vertical(...)`, `horizontal(...)`, `use_effect(...)` などを提供します。

### `widget/`

個別ウィジェットのビルダー群です。

1. `widget/button.rs`  
   `ButtonBuilder` と `ButtonResponse`
2. `widget/label.rs`  
   `LabelBuilder`
3. `widget/text.rs`  
   テキストのレイアウト・描画ヘルパー

### `layout/`

縦並び・横並びなどの簡易レイアウトロジックです。

### `hooks/`

`use_effect` の実装です。依存値が変わった時だけ effect を再実行します。

### `input.rs`

現在フレームのマウス入力状態（位置、押下、離上）を管理します。

### `font.rs`

デフォルトフォント読み込みです。

## 6. どう読むと理解しやすいか（おすすめ順）

1. `app/mod.rs`  
2. `app/events.rs`  
3. `app/frame.rs`  
4. `ui.rs`  
5. `retained/state.rs`  
6. `retained/input.rs`  
7. `retained/paint.rs`  
8. `widget/button.rs`

## 7. 今の設計のポイント

1. UI 要素は `RetainedState` に保持される  
2. 描画は `RetainedState::render()` が一元管理する  
3. アプリ側は `build` で構築、`update` で状態変更に集中できる

この構成にしておくと、今後 `TextField` や `Checkbox` を追加するときも、  
`node/state/input/paint/widget` の各責務に沿って拡張しやすくなります。
