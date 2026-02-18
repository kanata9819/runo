# Runo 関数呼び出し相関図（責務別）

このドキュメントは、`crates/runo_core/src` の主要責務ごとの呼び出し関係を、実装ベースで追いやすくするためのメモです。
対象は「主要ルート」です（全関数を網羅する完全コールグラフではありません）。

## 1. アプリ起動とフレーム進行（`app/`）

```mermaid
flowchart TD
    A["app::run()"] --> B["EventLoop::run_app(AppRunner)"]
    B --> C["AppRunner::resumed()"]
    C --> D["init_window_and_gpu()"]
    B --> E["AppRunner::window_event()"]
    E --> F["WindowEvent::RedrawRequested"]
    F --> G["AppRunner::render()"]
    G --> H["compose_frame()"]
    H --> I["build_scene()"]
    H --> J["run_ui_frame()"]
    H --> K["retained.render(scene, font)"]
    G --> L["submit_frame()"]
```

主ファイル:
- `crates/runo_core/src/app/mod.rs`
- `crates/runo_core/src/app/events.rs`
- `crates/runo_core/src/app/frame.rs`
- `crates/runo_core/src/app/runner.rs`

## 2. UI 宣言と RetainedState 同期（`ui/`, `ui/show/`, `retained/state/`）

```mermaid
flowchart LR
    A["user_app.build/update(ui)"] --> B["UiWidgets::*Builder"]
    B --> C["Builder::show()"]
    C --> D["Ui::show_* (ui/show/*.rs)"]
    D --> E["layout_stack.allocate_rect()"]
    D --> F["retained.upsert_*()"]
    F --> G["RetainedState.widgets/order 更新"]
```

代表例:
- `UiWidgets::button()` → `ButtonBuilder::show()` → `Ui::show_button()` → `retained.upsert_button(...)`
- `UiWidgets::div()` → `DivBuilder::show(f)` → `Ui::show_div()`（子レイアウトを push/pop）

主ファイル:
- `crates/runo_core/src/ui/mod.rs`
- `crates/runo_core/src/ui/widgets.rs`
- `crates/runo_core/src/ui/show/*.rs`
- `crates/runo_core/src/retained/state/*.rs`

## 3. 入力イベント反映（`app/events.rs` → `input.rs` → `retained/input/`）

```mermaid
flowchart TD
    A["WindowEvent (winit)"] --> B["InputState::on_* / set_*"]
    B --> C["AppRunner::run_ui_frame()"]
    C --> D["input.snapshot()"]
    D --> E["retained.begin_frame_input(input_frame, font)"]
    E --> F["update_hover_flags()"]
    E --> G["update_*_states()"]
    E --> H["apply_text_box_scroll()"]
    E --> I["apply_text_input()"]
```

主ファイル:
- `crates/runo_core/src/app/events.rs`
- `crates/runo_core/src/input.rs`
- `crates/runo_core/src/retained/input/mod.rs`
- `crates/runo_core/src/retained/input/pointer.rs`
- `crates/runo_core/src/retained/input/text_box.rs`

## 4. 描画パイプライン（`retained/paint/`）

```mermaid
flowchart TD
    A["RetainedState::render(scene, font)"] --> B["order を走査"]
    B --> C["widget別 paint::render()"]
    C --> D["button/checkbox/radio/slider/label/text_box/combo_box"]
    A --> E["2回目走査"]
    E --> F["combo_box::render_dropdown_overlay()"]
```

補助関数:
- `paint/interaction_color.rs::resolve_interaction_color(...)`
  `enabled > pressed > hovered > default` の共通色選択。

主ファイル:
- `crates/runo_core/src/retained/paint/mod.rs`
- `crates/runo_core/src/retained/paint/*.rs`

## 5. テキスト描画・レイアウト・キャッシュ（`widget/text.rs`, `cache/text_layout.rs`）

```mermaid
flowchart LR
    A["paint::*::render()"] --> B["text::layout_text(font, text, size)"]
    B --> C["text_layout::get_or_insert_layout(...)"]
    C -->|cache hit| D["(glyphs, advance) を返す"]
    C -->|cache miss| E["skrifa で glyph/advance 計算"]
    E --> D
    D --> F["text::draw_text_run(scene, glyphs, x, y, size, color)"]
```

主ファイル:
- `crates/runo_core/src/widget/text.rs`
- `crates/runo_core/src/cache/text_layout.rs`

## 6. レイアウト責務（`layout/stack.rs`, `ui/show/div.rs`）

```mermaid
flowchart TD
    A["Ui::vertical/horizontal"] --> B["LayoutStack::push_layout()"]
    B --> C["show_* が allocate_rect()"]
    C --> D["widget rect が確定"]
    A --> E["LayoutStack::pop_layout_and_advance_parent()"]
    F["Ui::show_div()"] --> G["layout_div_children()"]
    G --> H["push_layout_at(content_origin)"]
    H --> I["children 描画/登録"]
    I --> J["pop_layout_consumed()"]
```

主ファイル:
- `crates/runo_core/src/layout/stack.rs`
- `crates/runo_core/src/layout/div.rs`
- `crates/runo_core/src/ui/show/div.rs`

## 7. Effects とイベント取得（`hooks/effect.rs`, `ui/events.rs`）

```mermaid
flowchart LR
    A["run_ui_frame()"] --> B["effects.begin_frame()"]
    B --> C["Ui::use_effect(id, deps, effect)"]
    C --> D["EffectStore::use_effect()"]
    D --> E["deps 変化時 cleanup -> effect 再実行"]
    A --> F["effects.end_frame()"]
    F --> G["未使用エントリ cleanup + remove"]
```

```mermaid
flowchart LR
    A["UiEvents::next_event()/drain_events()"] --> B["retained.pop_event()/drain_events()"]
```

主ファイル:
- `crates/runo_core/src/hooks/effect.rs`
- `crates/runo_core/src/ui/events.rs`
- `crates/runo_core/src/retained/state/core.rs`

---

更新ルール（運用メモ）:
- 新規 widget を追加したら、最低でも「2. UI 宣言」「4. 描画」「3. 入力」の図にノードを追記する。
- 共通化（例: `paint` 共通ヘルパー）を入れたら、該当図に共通ノードを追加する。
