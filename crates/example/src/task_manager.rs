#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use runo::{
    ButtonHandle, CheckboxHandle, RunOptions, RunoApplication, TextBoxHandle, Ui, UiEvent, colors,
};

const SUMMARY_STATE_ID: &str = "state.task.summary";

#[derive(Clone)]
struct Task {
    id: u64,
    title: String,
    done: bool,
}

struct TaskApp {
    tasks: Vec<Task>,
    draft: String,
    next_id: u64,
    input: Option<TextBoxHandle>,
    add_button: Option<ButtonHandle>,
    clear_done_button: Option<ButtonHandle>,
    task_rows: Vec<TaskRowHandles>,
}

struct TaskRowHandles {
    task_id: u64,
    checkbox: CheckboxHandle,
    delete_button: ButtonHandle,
}

enum ButtonAction {
    Add,
    ClearDone,
    DeleteTask(u64),
}

impl TaskApp {
    fn add_task(&mut self) {
        let title = self.draft.trim();
        if title.is_empty() {
            return;
        }

        self.tasks.push(Task {
            id: self.next_id,
            title: title.to_string(),
            done: false,
        });
        self.next_id += 1;
        self.draft.clear();
    }

    fn clear_done(&mut self) {
        self.tasks.retain(|task| !task.done);
    }

    fn summary_text(&self) -> String {
        let done = self.tasks.iter().filter(|task| task.done).count();
        format!("{} tasks ({} done)", self.tasks.len(), done)
    }

    fn resolve_button_action(&self, button: &ButtonHandle) -> Option<ButtonAction> {
        if self.add_button.as_ref() == Some(button) {
            return Some(ButtonAction::Add);
        }
        if self.clear_done_button.as_ref() == Some(button) {
            return Some(ButtonAction::ClearDone);
        }
        self.task_rows
            .iter()
            .find(|row| row.delete_button == *button)
            .map(|row| ButtonAction::DeleteTask(row.task_id))
    }
}

impl RunoApplication for TaskApp {
    fn options(&self) -> RunOptions {
        RunOptions {
            window_title: "runo task manager example".to_string(),
            window_width: 1100,
            window_height: 760,
            window_resizable: false,
        }
    }

    fn build(&mut self, ui: &mut Ui<'_>) {
        let (summary, _) = ui.use_state(SUMMARY_STATE_ID, || self.summary_text());

        ui.vertical(|ui| {
            ui.widgets()
                .label()
                .text("Task Manager")
                .font_size(26)
                .show();

            ui.widgets()
                .label()
                .text(summary)
                .font_size(16)
                .text_color(colors::rgb(colors::TEXT_SECONDARY))
                .show();

            ui.widgets()
                .div()
                .width(860)
                .padding(16)
                .gap(12)
                .background(colors::rgb(colors::PANEL_BG))
                .border(colors::rgb(colors::PANEL_BORDER), 1)
                .radius(12)
                .show(|ui| {
                    ui.horizontal(|ui| {
                        self.input = Some(
                            ui.widgets()
                                .text_box()
                                .width(520)
                                .height(44)
                                .font_size(18)
                                .placeholder("Add a new task...")
                                .show(),
                        );

                        self.add_button = Some(
                            ui.widgets()
                                .button()
                                .width(100)
                                .height(44)
                                .font_size(16)
                                .text("Add")
                                .show(),
                        );

                        self.clear_done_button = Some(
                            ui.widgets()
                                .button()
                                .width(120)
                                .height(44)
                                .font_size(16)
                                .text("Clear done")
                                .show(),
                        );
                    });

                    let mut task_rows = Vec::new();
                    for task in &self.tasks {
                        ui.widgets()
                            .div()
                            .horizontal()
                            .width(728)
                            .padding(10)
                            .gap(8)
                            .background(colors::rgb(if task.done {
                                colors::PANEL_BG_ACTIVE
                            } else {
                                colors::APP_BG
                            }))
                            .border(colors::rgb(colors::PANEL_BORDER), 1)
                            .radius(10)
                            .show(|ui| {
                                let label = if task.done {
                                    format!("[done] {}", task.title)
                                } else {
                                    task.title.clone()
                                };

                                let checkbox = ui
                                    .widgets()
                                    .checkbox()
                                    .width(620)
                                    .height(36)
                                    .font_size(18)
                                    .text(label)
                                    .checked(task.done)
                                    .show();

                                let delete_button = ui
                                    .widgets()
                                    .button()
                                    .width(84)
                                    .height(36)
                                    .font_size(15)
                                    .text("Delete")
                                    .show();

                                task_rows.push(TaskRowHandles {
                                    task_id: task.id,
                                    checkbox,
                                    delete_button,
                                });
                            });
                    }
                    self.task_rows = task_rows;
                });
        });
    }

    fn update(&mut self, ui: &mut Ui<'_>) {
        let mut clear_input = false;
        for event in ui.events().drain_events() {
            match event {
                UiEvent::ButtonClicked { button } => match self.resolve_button_action(&button) {
                    Some(ButtonAction::Add) => {
                        self.add_task();
                        clear_input = true;
                    }
                    Some(ButtonAction::ClearDone) => {
                        self.clear_done();
                    }
                    Some(ButtonAction::DeleteTask(task_id)) => {
                        self.tasks.retain(|task| task.id != task_id);
                    }
                    None => {}
                },
                UiEvent::TextBoxChanged { text_box, text } => {
                    if self.input.as_ref() == Some(&text_box) {
                        self.draft = text;
                    }
                }
                UiEvent::CheckboxChanged { checkbox, checked } => {
                    if let Some(task_id) = self
                        .task_rows
                        .iter()
                        .find(|row| row.checkbox == checkbox)
                        .map(|row| row.task_id)
                        && let Some(task) = self.tasks.iter_mut().find(|task| task.id == task_id)
                    {
                        task.done = checked;
                    }
                }
                UiEvent::RadioButtonChanged { .. }
                | UiEvent::SliderChanged { .. }
                | UiEvent::ComboBoxChanged { .. } => {}
            }
        }

        if clear_input && let Some(input) = &self.input {
            input.set_text(ui, "");
        }

        let total = self.tasks.len();
        let done = self.tasks.iter().filter(|task| task.done).count();

        // use_effect is for side effects, not for deriving UI text values.
        ui.use_effect("task_stats_logger", (total, done), move || {
            println!("[effect] stats changed: total={}, done={}", total, done);
            Some(Box::new(move || {
                println!("[cleanup] previous stats: total={}, done={}", total, done);
            }))
        });

        let draft_is_empty = self.draft.trim().is_empty();
        ui.use_effect("draft_status_logger", draft_is_empty, move || {
            println!(
                "[effect] draft status changed: {}",
                if draft_is_empty { "empty" } else { "typing" }
            );
            None
        });

        let (_, set_summary) = ui.use_state(SUMMARY_STATE_ID, String::new);
        set_summary.set(ui, self.summary_text());
    }
}

fn main() {
    runo::run(TaskApp {
        tasks: vec![
            Task {
                id: 0,
                title: "Read runo docs".to_string(),
                done: true,
            },
            Task {
                id: 1,
                title: "Build a task app example".to_string(),
                done: false,
            },
        ],
        draft: String::new(),
        next_id: 2,
        input: None,
        add_button: None,
        clear_done_button: None,
        task_rows: Vec::new(),
    });
}
