# Runo Core README

`runo_core` は、`winit + wgpu + vello` で構成した、保持型（retained mode）寄りの GUI コアです。

## 役割の要約

1. `app/`: 実行ループ、イベント処理、GPU 描画
2. `retained/`: ウィジェット状態の保持・入力更新・描画
3. `ui/`: ユーザー向け API（widgets/state/events/layout/effect）
4. `widget/`: 各ウィジェットのビルダー

## 現在の主要 API

1. ウィジェット生成: `ui.widgets().button()/label()/text_box()/combo_box()/checkbox()/radio_button()/slider()/div()`
2. イベント取得: `ui.events().drain_events()`, `ui.events().next_event()`
3. 状態変更: `ui.state().button().set_text()`, `ui.state().text_box().set_text()`, `ui.state().combo_box().set_selected_index()` など
4. 活性/非活性: `ui.state().*().set_enabled()` とビルダーの `enabled(...)`

## ライフサイクル

1. `RunoApplication::build()` で初期 UI 構築
2. `RunoApplication::update()` で毎フレーム状態更新

## 詳細ドキュメント

- `crates/runo_core/docs/README.md`
