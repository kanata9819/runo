# GUI ライブラリ知識ノート（日本語）

GUI ライブラリを作るときに必要になる、実装非依存の基礎知識をまとめています。

## 現在の `runo` 実装メモ

1. 組み込みウィジェット
   `button`, `label`, `text_box`, `combo_box`, `checkbox`, `radio_button`, `slider`, `div`
2. イベントモデル
   `UiEvent`（ハンドル付きイベント）+ `ActionBindings` / `EventBindings` / `EventBindingsBuilder`
3. 活性/非活性 API
   `ui.state().*().set_enabled(...)` / `enabled(...)`
4. ComboBox の動的 items 更新 API
   `ui.state().combo_box().set_items(...)`
5. ハンドル API
   `ButtonHandle` などで `on_click` / `take_click` / `set_enabled` などを利用可能
6. `Option<Handle>` 拡張 API
   `prelude::*` で `Optional*HandleExt`（`on_click` / `on_change` / `take_change`）を利用可能
7. 色プリセット API
   定数 + grouped API（`Gray::gray_50()`, `Semantic::success()` など）

## ドキュメント

1. `architecture.md`
   `runo` の現在実装アーキテクチャ
2. `knowledge-fundamentals.md`
   GUI ライブラリ設計の基礎概念
3. `knowledge-event-input.md`
   イベントループと入力モデル
4. `knowledge-layout-widget.md`
   レイアウトとウィジェット設計
5. `knowledge-rendering-performance.md`
   描画パイプラインと性能
6. `knowledge-testing-release.md`
   テスト戦略と公開前チェック

## ローカル品質チェック

`just ci` で CI と同等のチェック（`fmt --check`, strict `clippy`, workspace tests）を実行できます。
