# Runo Core README

`runo_core` は、`winit + wgpu + vello` で構成した、保持型（retained mode）寄りの GUI コアです。

## 役割の要約

1. `app/`: 実行ループ、イベント処理、GPU 描画
2. `retained/`: ウィジェット状態の保持・入力更新・描画
3. `ui.rs`: ユーザー向け API（button/label/layout/effect）
4. `widget/`: 各ウィジェットのビルダー

## ライフサイクル

1. `Application::build()` で初期 UI 構築
2. `Application::update()` で毎フレーム状態更新

## 詳細ドキュメント

- `crates/runo_core/docs/README.md`
