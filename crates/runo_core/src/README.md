# Runo Core README

`runo_core` は、`winit + wgpu + vello` で構成した、保持型（retained mode）寄りの GUI コアです。

## 役割の要約

1. `app/`: 実行ループ、イベント処理、GPU 描画
2. `retained/`: ウィジェット状態の保持・入力更新・描画
3. `ui.rs`: ユーザー向け API（button/label/text_box/combo_box/div/layout/effect）
4. `widget/`: 各ウィジェットのビルダー

## 現在の主要 API

1. ウィジェット生成: `button()`, `label()`, `text_box()`, `combo_box()`, `div()`
2. イベント取得: `drain_events()`, `next_event()`
3. 状態変更: `set_button_text()`, `set_text_box_text()`, `set_combo_box_selected_index()`
4. 活性/非活性: `set_button_enabled()`, `set_text_box_enabled()`, `set_combo_box_enabled()`, `set_label_enabled()`

## ライフサイクル

1. `Application::build()` で初期 UI 構築
2. `Application::update()` で毎フレーム状態更新

## 詳細ドキュメント

- `crates/runo_core/docs/README.md`
