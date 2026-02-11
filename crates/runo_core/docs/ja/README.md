# GUI ライブラリ知識ノート（日本語）

GUI ライブラリを作るときに必要になる、実装非依存の基礎知識をまとめています。

## 現在の `runo_core` 実装メモ

1. 組み込みウィジェット
   `button`, `label`, `text_box`, `combo_box`
2. イベントモデル
   `UiEvent`（`ButtonClicked`, `TextBoxChanged`, `ComboBoxChanged`）
3. 活性/非活性 API
   `set_*_enabled` / `enabled(...)`

## ドキュメント

1. `architecture.md`
   `runo_core` の現在実装アーキテクチャ
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
