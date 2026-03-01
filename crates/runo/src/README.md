# Runo Core README

`runo` は、`winit + wgpu + vello` で構成した、保持型（retained mode）寄りの GUI コアです。

## 役割の要約

1. `app/`: 実行ループ、イベント処理、GPU 描画
2. `retained/`: ウィジェット状態の保持・入力更新・描画
3. `ui/`: ユーザー向け API（widgets/state/events/layout/effect）
4. `widget/`: 各ウィジェットのビルダー

## 現在の主要 API

1. ウィジェット生成: `ui.widgets().button()/label()/text_box()/combo_box()/checkbox()/radio_button()/slider()/div()`
2. イベント取得: `ui.events().drain_events()`, `ui.events().next_event()`, `ui.events().drain_actions()`, `ui.events().drain_bound_events()`
3. 状態変更: `ui.state().button().set_text()`, `ui.state().text_box().set_text()`, `ui.state().combo_box().set_selected_index()`, `ui.state().combo_box().set_items()` など
4. 活性/非活性: `ui.state().*().set_enabled()` とビルダーの `enabled(...)`
5. ハンドル操作: `ButtonHandle` などで `set_text()/set_enabled()/on_click()/take_click()` を利用可能
6. `Option<...Handle>` 拡張: `prelude::*` で `Optional*HandleExt` を利用可能
7. 色プリセット: `colors::GRAY_500` / `colors::Gray::gray_500()` / `colors::Semantic::success()`

## ライフサイクル

1. `RunoApplication::build()` で初期 UI 構築
2. `RunoApplication::update()` で毎フレーム状態更新

## フレーム処理の流れ（実装対応）

1. `ui/show/*` がウィジェット定義を `retained/state` に `upsert`
2. `retained/input/*` が入力から `hovered/pressed/focused` を更新し `UiEvent` を生成
3. `retained/paint/*` が保持ノードを描画（本体パス -> オーバーレイパス）
4. `ui/events` がアプリへイベントを提供

## 詳細ドキュメント

- `crates/runo/docs/ja/README.md`
- `crates/runo/docs/en/README.md`
