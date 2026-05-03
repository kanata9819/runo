use runo::{
    ButtonHandle, Color, EventBindings, RunOptions, RunoApplication, TextBoxHandle, Ui, colors,
};

use crate::codegen::generate_rust_code;
use crate::document::{StudioDocument, WidgetKind, WidgetSpec};

const WINDOW_W: u32 = 1320;
const WINDOW_H: u32 = 820;
const HEADER_W: u32 = 1296;
const BODY_W: u32 = 1296;
const BODY_H: u32 = 714;
const PALETTE_BUTTON_X: f64 = 24.0;
const PALETTE_BUTTON_Y: f64 = 136.0;
const PALETTE_BUTTON_W: f64 = 188.0;
const PALETTE_BUTTON_H: f64 = 42.0;
const PALETTE_BUTTON_GAP: f64 = 10.0;
const PREVIEW_W: u32 = 620;
const PREVIEW_H: u32 = 690;
const PREVIEW_X: f64 = (WINDOW_W as f64 - PREVIEW_W as f64) * 0.5;
const PREVIEW_Y: f64 = 86.0;
const PREVIEW_PAD: f64 = 14.0;
const PREVIEW_CONTENT_X: f64 = PREVIEW_X + PREVIEW_PAD;
const PREVIEW_CONTENT_Y: f64 = PREVIEW_Y + 52.0;
const PREVIEW_CONTENT_W: f64 = PREVIEW_W as f64 - PREVIEW_PAD * 2.0;
const PREVIEW_CONTENT_H: f64 = PREVIEW_H as f64 - 66.0;

#[derive(Default)]
pub struct RunoStudioApp {
    document: StudioDocument,
    handles: StudioHandles,
    dragging: Option<DragState>,
}

#[derive(Default)]
struct StudioHandles {
    add_button: Option<ButtonHandle>,
    add_label: Option<ButtonHandle>,
    add_text_box: Option<ButtonHandle>,
    delete_selected: Option<ButtonHandle>,
    move_up: Option<ButtonHandle>,
    move_down: Option<ButtonHandle>,
    property_text: Option<TextBoxHandle>,
    property_width: Option<TextBoxHandle>,
    property_height: Option<TextBoxHandle>,
    selection_rows: Vec<SelectionRowHandle>,
}

struct SelectionRowHandle {
    uid: u64,
    button: ButtonHandle,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum DragState {
    Palette(WidgetKind),
    Placed { uid: u64, grab_x: f64, grab_y: f64 },
}

#[derive(Clone)]
pub enum StudioEvent {
    Add(WidgetKind),
    Select(u64),
    DeleteSelected,
    MoveSelectedUp,
    MoveSelectedDown,
    SelectedTextChanged(String),
    SelectedWidthChanged(String),
    SelectedHeightChanged(String),
}

impl RunoApplication<StudioEvent> for RunoStudioApp {
    fn options(&self) -> RunOptions {
        RunOptions {
            window_title: "Runo Studio".to_string(),
            window_width: WINDOW_W,
            window_height: WINDOW_H,
            window_resizable: false,
            immediate_mode: false,
        }
    }

    fn build(&mut self, ui: &mut Ui<'_>) -> EventBindings<StudioEvent> {
        self.handles = StudioHandles::default();

        ui.vertical(|ui| {
            self.build_header(ui);
            ui.widgets()
                .div()
                .horizontal()
                .width(BODY_W)
                .height(BODY_H)
                .gap(12)
                .padding(12)
                .show(|ui| {
                    self.build_palette(ui);
                    ui.widgets().div().width(94).height(690).show(|_| {});
                    self.build_preview(ui);
                    self.build_properties(ui);
                });
            self.build_drag_overlay(ui);
        });

        self.build_event_bindings()
    }

    fn update(&mut self, ui: &mut Ui<'_>) -> bool {
        let started = self.start_drag_if_needed(ui);
        let moved = self.move_dragged_widget_if_needed(ui);
        let dropped = self.drop_drag_if_needed(ui);
        let palette_dragging = matches!(self.dragging, Some(DragState::Palette(_)));
        started || moved || dropped || palette_dragging
    }

    fn on_event(&mut self, _ui: &mut Ui<'_>, event: StudioEvent) -> bool {
        match event {
            StudioEvent::Add(kind) => self.document.add_widget(kind),
            StudioEvent::Select(uid) => self.document.select(uid),
            StudioEvent::DeleteSelected => self.document.delete_selected(),
            StudioEvent::MoveSelectedUp => self.document.move_selected_up(),
            StudioEvent::MoveSelectedDown => self.document.move_selected_down(),
            StudioEvent::SelectedTextChanged(text) => self.document.set_selected_text(text),
            StudioEvent::SelectedWidthChanged(text) => {
                self.document.set_selected_width_from_text(&text);
            }
            StudioEvent::SelectedHeightChanged(text) => {
                self.document.set_selected_height_from_text(&text);
            }
        }
        true
    }
}

impl RunoStudioApp {
    fn build_header(&mut self, ui: &mut Ui<'_>) {
        ui.widgets()
            .div()
            .horizontal()
            .width(HEADER_W)
            .padding(14)
            .gap(12)
            .background(colors::rgb(colors::PANEL_BG))
            .border(colors::rgb(colors::PANEL_BORDER), 1)
            .radius(10)
            .show(|ui| {
                ui.widgets()
                    .label()
                    .width(240)
                    .height(34)
                    .font_size(26)
                    .text("Runo Studio")
                    .show();
                ui.widgets()
                    .label()
                    .width(760)
                    .height(34)
                    .font_size(16)
                    .text_color(colors::rgb(colors::TEXT_SECONDARY))
                    .text("Drag controls into the centered preview, edit properties, generate Rust")
                    .show();
            });
    }

    fn build_palette(&mut self, ui: &mut Ui<'_>) {
        ui.widgets()
            .div()
            .width(220)
            .height(690)
            .padding(12)
            .gap(10)
            .background(colors::rgb(colors::PANEL_BG))
            .border(colors::rgb(colors::PANEL_BORDER), 1)
            .radius(10)
            .show(|ui| {
                ui.widgets()
                    .label()
                    .height(28)
                    .font_size(20)
                    .text("Palette")
                    .show();
                self.handles.add_button = Some(
                    ui.widgets()
                        .button()
                        .width(188)
                        .height(42)
                        .font_size(15)
                        .text("+ Button")
                        .show(),
                );
                self.handles.add_label = Some(
                    ui.widgets()
                        .button()
                        .width(188)
                        .height(42)
                        .font_size(15)
                        .text("+ Label")
                        .show(),
                );
                self.handles.add_text_box = Some(
                    ui.widgets()
                        .button()
                        .width(188)
                        .height(42)
                        .font_size(15)
                        .text("+ TextBox")
                        .show(),
                );
                ui.widgets()
                    .label()
                    .height(24)
                    .font_size(13)
                    .text_color(colors::rgb(colors::TEXT_SECONDARY))
                    .text("Drag or click to add")
                    .show();
            });
    }

    fn build_preview(&mut self, ui: &mut Ui<'_>) {
        let widgets = self.document.widgets().to_vec();
        let selected_uid = self.document.selected_uid();

        ui.widgets()
            .div()
            .width(PREVIEW_W)
            .height(PREVIEW_H)
            .padding(14)
            .gap(10)
            .background(colors::rgb(colors::PANEL_BG))
            .border(colors::rgb(colors::PANEL_BORDER), 1)
            .radius(10)
            .show(|ui| {
                ui.widgets()
                    .label()
                    .height(24)
                    .font_size(20)
                    .text("Preview")
                    .show();

                if widgets.is_empty() {
                    ui.at(PREVIEW_CONTENT_X + 176.0, PREVIEW_CONTENT_Y + 254.0, |ui| {
                        ui.widgets()
                            .label()
                            .width(260)
                            .height(32)
                            .font_size(20)
                            .text_color(colors::rgb(colors::TEXT_MUTED))
                            .text("Drop controls here")
                            .show();
                    });
                }

                for widget in &widgets {
                    ui.with_stable_key(format!("preview.{}", widget.uid), |ui| {
                        ui.at(
                            PREVIEW_CONTENT_X + widget.x,
                            PREVIEW_CONTENT_Y + widget.y,
                            |ui| {
                                show_preview_widget(ui, widget, selected_uid == Some(widget.uid));
                            },
                        );
                    });
                }
            });
    }

    fn build_properties(&mut self, ui: &mut Ui<'_>) {
        let selected = self.document.selected().cloned();
        let widgets = self.document.widgets().to_vec();
        let selected_uid = self.document.selected_uid();
        let code = generate_rust_code(&self.document);

        ui.widgets()
            .div()
            .width(300)
            .height(690)
            .padding(12)
            .gap(10)
            .background(colors::rgb(colors::PANEL_BG))
            .border(colors::rgb(colors::PANEL_BORDER), 1)
            .radius(10)
            .show(|ui| {
                ui.widgets()
                    .label()
                    .height(28)
                    .font_size(20)
                    .text("Inspector")
                    .show();

                self.build_outline(ui, &widgets, selected_uid);

                if let Some(widget) = selected {
                    self.build_selected_properties(ui, &widget);
                } else {
                    ui.widgets()
                        .label()
                        .height(28)
                        .font_size(15)
                        .text_color(colors::rgb(colors::TEXT_SECONDARY))
                        .text("No widget selected")
                        .show();
                }

                ui.widgets()
                    .label()
                    .height(24)
                    .font_size(18)
                    .text("Generated Rust")
                    .show();
                ui.widgets()
                    .text_box()
                    .width(266)
                    .height(210)
                    .font_size(12)
                    .text(code)
                    .read_only(true)
                    .overflow_x(runo::Overflow::Auto)
                    .overflow_y(runo::Overflow::Auto)
                    .show();
            });
    }

    fn build_outline(
        &mut self,
        ui: &mut Ui<'_>,
        widgets: &[WidgetSpec],
        selected_uid: Option<u64>,
    ) {
        ui.widgets()
            .div()
            .width(266)
            .height(196)
            .padding(10)
            .gap(8)
            .background(colors::rgb(colors::APP_BG))
            .border(colors::rgb(colors::PANEL_BORDER), 1)
            .radius(8)
            .show(|ui| {
                ui.widgets()
                    .label()
                    .height(24)
                    .font_size(18)
                    .text("Outline")
                    .show();

                for widget in widgets {
                    let selected = selected_uid == Some(widget.uid);
                    let label = if selected {
                        format!("> {}", widget.summary())
                    } else {
                        widget.summary()
                    };
                    let button = ui
                        .widgets()
                        .button()
                        .width(236)
                        .height(36)
                        .font_size(13)
                        .text(label)
                        .show();
                    self.handles.selection_rows.push(SelectionRowHandle {
                        uid: widget.uid,
                        button,
                    });
                }
            });
    }

    fn build_selected_properties(&mut self, ui: &mut Ui<'_>, widget: &WidgetSpec) {
        ui.widgets()
            .label()
            .height(24)
            .font_size(15)
            .text_color(colors::rgb(colors::TEXT_SECONDARY))
            .text(format!("{} ({})", widget.id, widget.kind.display_name()))
            .show();

        self.handles.property_text = Some(
            ui.widgets()
                .text_box()
                .id("studio.property.text")
                .width(266)
                .height(40)
                .font_size(15)
                .placeholder("Text")
                .text(widget.text.clone())
                .show(),
        );

        ui.horizontal(|ui| {
            self.handles.property_width = Some(
                ui.widgets()
                    .text_box()
                    .id("studio.property.width")
                    .width(86)
                    .height(40)
                    .font_size(15)
                    .text(widget.width.to_string())
                    .show(),
            );
            self.handles.property_height = Some(
                ui.widgets()
                    .text_box()
                    .id("studio.property.height")
                    .width(86)
                    .height(40)
                    .font_size(15)
                    .text(widget.height.to_string())
                    .show(),
            );
        });

        ui.horizontal(|ui| {
            self.handles.move_up = Some(
                ui.widgets()
                    .button()
                    .width(76)
                    .height(36)
                    .font_size(14)
                    .text("Up")
                    .show(),
            );
            self.handles.move_down = Some(
                ui.widgets()
                    .button()
                    .width(92)
                    .height(36)
                    .font_size(14)
                    .text("Down")
                    .show(),
            );
            self.handles.delete_selected = Some(
                ui.widgets()
                    .button()
                    .width(72)
                    .height(36)
                    .font_size(14)
                    .text("Delete")
                    .show(),
            );
        });
    }

    fn build_drag_overlay(&self, ui: &mut Ui<'_>) {
        if !ui.input().mouse_down() {
            return;
        }

        let Some(DragState::Palette(kind)) = self.dragging else {
            return;
        };

        let (x, y) = ui.input().cursor_pos();
        ui.with_stable_key("studio.drag_overlay", |ui| {
            ui.at(x - 80.0, y - 20.0, |ui| {
                ui.widgets()
                    .div()
                    .width(160)
                    .height(40)
                    .padding(8)
                    .background(Color::from_rgb8(91, 126, 201))
                    .radius(8)
                    .show(|ui| {
                        ui.widgets()
                            .label()
                            .width(144)
                            .height(24)
                            .font_size(15)
                            .text(kind.display_name())
                            .show();
                    });
            });
        });
    }

    fn build_event_bindings(&self) -> EventBindings<StudioEvent> {
        let mut builder = EventBindings::builder()
            .button(
                self.handles.add_button.clone(),
                StudioEvent::Add(WidgetKind::Button),
            )
            .button(
                self.handles.add_label.clone(),
                StudioEvent::Add(WidgetKind::Label),
            )
            .button(
                self.handles.add_text_box.clone(),
                StudioEvent::Add(WidgetKind::TextBox),
            )
            .button(
                self.handles.delete_selected.clone(),
                StudioEvent::DeleteSelected,
            )
            .button(self.handles.move_up.clone(), StudioEvent::MoveSelectedUp)
            .button(
                self.handles.move_down.clone(),
                StudioEvent::MoveSelectedDown,
            )
            .text_box(
                self.handles.property_text.clone(),
                StudioEvent::SelectedTextChanged,
            )
            .text_box(
                self.handles.property_width.clone(),
                StudioEvent::SelectedWidthChanged,
            )
            .text_box(
                self.handles.property_height.clone(),
                StudioEvent::SelectedHeightChanged,
            );

        for row in &self.handles.selection_rows {
            let uid = row.uid;
            builder = builder.button(row.button.clone(), StudioEvent::Select(uid));
        }

        builder.build()
    }

    fn start_drag_if_needed(&mut self, ui: &Ui<'_>) -> bool {
        if !ui.input().mouse_pressed() {
            return false;
        }

        let (x, y) = ui.input().cursor_pos();
        if let Some((local_x, local_y)) = preview_local_pos(x, y)
            && let Some(widget) = self.document.widget_at(local_x, local_y)
        {
            let uid = widget.uid;
            let grab_x = local_x - widget.x;
            let grab_y = local_y - widget.y;
            self.document.select(uid);
            self.dragging = Some(DragState::Placed {
                uid,
                grab_x,
                grab_y,
            });
            return true;
        }

        if let Some(kind) = palette_kind_at(x, y) {
            self.dragging = Some(DragState::Palette(kind));
            return true;
        }

        false
    }

    fn move_dragged_widget_if_needed(&mut self, ui: &Ui<'_>) -> bool {
        if !ui.input().mouse_down() {
            return false;
        }

        let Some(DragState::Placed {
            uid,
            grab_x,
            grab_y,
        }) = self.dragging
        else {
            return false;
        };

        let Some(widget) = self.document.widget(uid) else {
            self.dragging = None;
            return true;
        };

        let (x, y) = ui.input().cursor_pos();
        let (width, height) = (widget.width, widget.height);
        let local_x = x - PREVIEW_CONTENT_X - grab_x;
        let local_y = y - PREVIEW_CONTENT_Y - grab_y;
        let (local_x, local_y) = clamp_widget_position(local_x, local_y, width, height);
        if (widget.x - local_x).abs() < f64::EPSILON && (widget.y - local_y).abs() < f64::EPSILON {
            return false;
        }

        self.document.move_widget_to(uid, local_x, local_y);
        true
    }

    fn drop_drag_if_needed(&mut self, ui: &Ui<'_>) -> bool {
        if !ui.input().mouse_released() {
            return false;
        }

        let Some(dragging) = self.dragging.take() else {
            return false;
        };

        let DragState::Palette(kind) = dragging else {
            return true;
        };

        let (x, y) = ui.input().cursor_pos();
        if !contains(
            x,
            y,
            PREVIEW_CONTENT_X,
            PREVIEW_CONTENT_Y,
            PREVIEW_CONTENT_W,
            PREVIEW_CONTENT_H,
        ) {
            return true;
        }

        let width = default_preview_width(kind);
        let height = default_preview_height(kind);
        let (local_x, local_y) = clamp_widget_position(
            x - PREVIEW_CONTENT_X - 80.0,
            y - PREVIEW_CONTENT_Y - 20.0,
            width,
            height,
        );
        self.document.add_widget_at(kind, local_x, local_y);
        true
    }
}

fn show_preview_widget(ui: &mut Ui<'_>, widget: &WidgetSpec, selected: bool) {
    if selected {
        ui.fill_rect(
            PREVIEW_CONTENT_X + widget.x - 4.0,
            PREVIEW_CONTENT_Y + widget.y - 4.0,
            f64::from(widget.width) + 8.0,
            f64::from(widget.height) + 8.0,
            Color::from_rgb8(91, 126, 201),
        );
    }

    match widget.kind {
        WidgetKind::Button => {
            ui.widgets()
                .button()
                .id(format!("studio.preview.{}", widget.id))
                .width(widget.width)
                .height(widget.height)
                .font_size(15)
                .text(widget.text.clone())
                .show();
        }
        WidgetKind::Label => {
            ui.widgets()
                .label()
                .id(format!("studio.preview.{}", widget.id))
                .width(widget.width)
                .height(widget.height)
                .font_size(16)
                .text(widget.text.clone())
                .show();
        }
        WidgetKind::TextBox => {
            ui.widgets()
                .text_box()
                .id(format!("studio.preview.{}", widget.id))
                .width(widget.width)
                .height(widget.height)
                .font_size(15)
                .placeholder("TextBox")
                .text(widget.text.clone())
                .show();
        }
    }
}

fn palette_kind_at(x: f64, y: f64) -> Option<WidgetKind> {
    [WidgetKind::Button, WidgetKind::Label, WidgetKind::TextBox]
        .into_iter()
        .enumerate()
        .find(|(index, _)| {
            let top = PALETTE_BUTTON_Y + (*index as f64 * (PALETTE_BUTTON_H + PALETTE_BUTTON_GAP));
            contains(
                x,
                y,
                PALETTE_BUTTON_X,
                top,
                PALETTE_BUTTON_W,
                PALETTE_BUTTON_H,
            )
        })
        .map(|(_, kind)| kind)
}

fn preview_local_pos(x: f64, y: f64) -> Option<(f64, f64)> {
    if !contains(
        x,
        y,
        PREVIEW_CONTENT_X,
        PREVIEW_CONTENT_Y,
        PREVIEW_CONTENT_W,
        PREVIEW_CONTENT_H,
    ) {
        return None;
    }

    Some((x - PREVIEW_CONTENT_X, y - PREVIEW_CONTENT_Y))
}

fn clamp_widget_position(x: f64, y: f64, width: u32, height: u32) -> (f64, f64) {
    let max_x = (PREVIEW_CONTENT_W - f64::from(width)).max(0.0);
    let max_y = (PREVIEW_CONTENT_H - f64::from(height)).max(0.0);
    (x.clamp(0.0, max_x), y.clamp(0.0, max_y))
}

fn default_preview_width(kind: WidgetKind) -> u32 {
    match kind {
        WidgetKind::Button => 180,
        WidgetKind::Label => 240,
        WidgetKind::TextBox => 260,
    }
}

fn default_preview_height(kind: WidgetKind) -> u32 {
    match kind {
        WidgetKind::Button => 48,
        WidgetKind::Label => 32,
        WidgetKind::TextBox => 44,
    }
}

fn contains(px: f64, py: f64, x: f64, y: f64, w: f64, h: f64) -> bool {
    px >= x && px <= x + w && py >= y && py <= y + h
}
