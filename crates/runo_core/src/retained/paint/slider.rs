use vello::Scene;
use vello::kurbo::{Affine, Circle, RoundedRect, Stroke};
use vello::peniko::{Color, Fill, FontData};

use crate::retained::node::SliderNode;
use crate::widget::text::{draw_text_run, layout_text};

pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, slider: &SliderNode) {
    let track_h = 6.0;
    let pad_x = 12.0;
    let track_x0 = slider.rect.x0 + pad_x;
    let track_x1 = slider.rect.x1 - pad_x;
    let track_y = slider.rect.y0 + slider.rect.height() * 0.62;
    let track_rect = RoundedRect::new(
        track_x0,
        track_y - track_h * 0.5,
        track_x1,
        track_y + track_h * 0.5,
        3.0,
    );

    let ratio = value_ratio(slider.value, slider.min, slider.max);
    let thumb_x = track_x0 + (track_x1 - track_x0) * ratio;

    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        if slider.enabled {
            Color::from_rgb8(56, 63, 74)
        } else {
            Color::from_rgb8(48, 52, 58)
        },
        None,
        &track_rect,
    );

    let active_rect = RoundedRect::new(
        track_x0,
        track_y - track_h * 0.5,
        thumb_x,
        track_y + track_h * 0.5,
        3.0,
    );
    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        if !slider.enabled {
            Color::from_rgb8(78, 82, 90)
        } else if slider.pressed {
            Color::from_rgb8(37, 132, 214)
        } else if slider.hovered {
            Color::from_rgb8(62, 154, 234)
        } else {
            Color::from_rgb8(50, 144, 229)
        },
        None,
        &active_rect,
    );

    let thumb = Circle::new((thumb_x, track_y), 8.0);
    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        if slider.enabled {
            Color::from_rgb8(240, 246, 255)
        } else {
            Color::from_rgb8(163, 169, 177)
        },
        None,
        &thumb,
    );
    scene.stroke(
        &Stroke::new(1.0),
        Affine::IDENTITY,
        Color::from_rgb8(78, 89, 104),
        None,
        &thumb,
    );

    let Some(font) = font else {
        return;
    };
    if let Some(text) = slider.text.as_deref()
        && let Some((glyphs, _)) = layout_text(font, text, slider.font_size)
    {
        let baseline_y = slider.rect.y0 + slider.font_size as f64;
        draw_text_run(
            scene,
            font,
            glyphs,
            slider.rect.x0 + pad_x,
            baseline_y,
            slider.font_size,
            if slider.enabled {
                slider.text_color
            } else {
                Color::from_rgb8(146, 152, 160)
            },
        );
    }

    let value_text = format!("{:.2}", slider.value);
    if let Some((glyphs, w)) = layout_text(font, &value_text, slider.font_size) {
        let baseline_y = slider.rect.y0 + slider.font_size as f64;
        draw_text_run(
            scene,
            font,
            glyphs,
            slider.rect.x1 - pad_x - w as f64,
            baseline_y,
            slider.font_size,
            if slider.enabled {
                slider.text_color
            } else {
                Color::from_rgb8(146, 152, 160)
            },
        );
    }
}

fn value_ratio(value: f64, min: f64, max: f64) -> f64 {
    let span = (max - min).abs();
    if span <= f64::EPSILON {
        return 0.0;
    }
    ((value - min) / (max - min)).clamp(0.0, 1.0)
}
