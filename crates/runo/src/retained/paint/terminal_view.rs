use vello::Scene;
use vello::kurbo::{Affine, Rect, RoundedRect};
use vello::peniko::{Color, Fill, FontData};

use crate::retained::node::TerminalViewNode;
use crate::theme::color;
use crate::widget::terminal_view::TerminalCell;
use crate::widget::text;

const CORNER_RADIUS: f64 = 8.0;
const BORDER_STROKE_WIDTH: f64 = 1.0;
const INNER_PADDING: f64 = 12.0;
const LINE_HEIGHT_RATIO: f64 = 1.35;

struct TerminalMetrics {
    inner_left: f64,
    inner_top: f64,
    inner_right: f64,
    inner_bottom: f64,
    first_baseline: f64,
    cell_width: f64,
    line_height: f64,
}

pub(super) fn render(
    scene: &mut Scene,
    font: Option<&FontData>,
    terminal_view: &mut TerminalViewNode,
) {
    draw_background_and_border(scene, terminal_view);

    let Some(font) = font else {
        return;
    };

    let metrics = TerminalMetrics {
        line_height: f64::from(terminal_view.font_size) * LINE_HEIGHT_RATIO,
        cell_width: f64::from(text::estimate_text_width("W", terminal_view.font_size)),
        inner_left: terminal_view.rect.x0 + INNER_PADDING,
        inner_top: terminal_view.rect.y0 + INNER_PADDING,
        inner_right: terminal_view.rect.x1 - INNER_PADDING,
        inner_bottom: terminal_view.rect.y1 - INNER_PADDING,
        first_baseline: terminal_view.rect.y0 + INNER_PADDING + f64::from(terminal_view.font_size),
    };

    for (row_index, line) in terminal_view.buffer.lines.iter().enumerate() {
        let baseline_y: f64 = metrics.first_baseline + row_index as f64 * metrics.line_height
            - terminal_view.scroll_y;
        if baseline_y + metrics.line_height < metrics.inner_top
            || baseline_y - f64::from(terminal_view.font_size) > metrics.inner_bottom
        {
            continue;
        }

        for (col_index, cell) in line.iter().enumerate() {
            draw_cell(
                scene,
                font,
                terminal_view,
                &metrics,
                cell,
                metrics.inner_left + col_index as f64 * metrics.cell_width,
                baseline_y,
            );
        }
    }

    draw_cursor(scene, terminal_view, &metrics);
}

fn draw_background_and_border(scene: &mut Scene, terminal_view: &TerminalViewNode) {
    let bg: RoundedRect = RoundedRect::from_rect(terminal_view.rect, CORNER_RADIUS);

    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        if terminal_view.enabled {
            terminal_view.bg_color
        } else {
            color::Neutral::tone_45_49_55()
        },
        None,
        &bg,
    );

    let border_color: Color = if !terminal_view.enabled {
        color::Neutral::tone_86_92_101()
    } else {
        terminal_view.border_color
    };

    if !terminal_view.disable_border {
        scene.stroke(
            &vello::kurbo::Stroke::new(BORDER_STROKE_WIDTH),
            Affine::IDENTITY,
            border_color,
            None,
            &bg,
        );
    }
}

fn draw_cell(
    scene: &mut Scene,
    font: &FontData,
    terminal_view: &TerminalViewNode,
    metrics: &TerminalMetrics,
    cell: &TerminalCell,
    x: f64,
    baseline_y: f64,
) {
    if cell.ch == ' ' || x + 1.0 < metrics.inner_left || x > metrics.inner_right {
        return;
    }

    let color: Color = if cell.faint {
        Color::from_rgba8(236, 241, 247, 120)
    } else {
        terminal_view.text_color
    };

    let Some((glyphs, _)) = text::layout_text(font, &cell.ch.to_string(), terminal_view.font_size)
    else {
        return;
    };

    text::draw_text_run(
        scene,
        font,
        glyphs,
        x,
        baseline_y,
        terminal_view.font_size,
        color,
    );
}

fn draw_cursor(scene: &mut Scene, terminal_view: &TerminalViewNode, metrics: &TerminalMetrics) {
    let x0: f64 = metrics.inner_left + terminal_view.buffer.cursor_col as f64 * metrics.cell_width;
    let y0: f64 = metrics.inner_top + terminal_view.buffer.cursor_row as f64 * metrics.line_height
        - terminal_view.scroll_y;
    let rect: Rect = Rect::new(
        x0,
        y0,
        x0 + metrics.cell_width.max(2.0),
        y0 + metrics.line_height,
    );

    if rect.x1 < metrics.inner_left
        || rect.x0 > metrics.inner_right
        || rect.y1 < metrics.inner_top
        || rect.y0 > metrics.inner_bottom
    {
        return;
    }

    scene.stroke(
        &vello::kurbo::Stroke::new(1.0),
        Affine::IDENTITY,
        Color::from_rgba8(255, 255, 255, 180),
        None,
        &rect,
    );
}
