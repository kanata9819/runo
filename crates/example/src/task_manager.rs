#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use runo::{RunOptions, RunoApplication, Ui, UiEvent, colors};

const TITLE_ID: &str = "task.title";
const SUMMARY_ID: &str = "task.summary";
const PANEL_ID: &str = "task.panel";
const INPUT_ID: &str = "task.input";
const ADD_BUTTON_ID: &str = "task.add";
const CLEAR_DONE_BUTTON_ID: &str = "task.clear_done";

const TASK_CHECK_PREFIX: &str = "task.item.check.";
const TASK_DELETE_PREFIX: &str = "task.item.delete.";
const TASK_ROW_PREFIX: &str = "task.item.row.";

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

    fn parse_task_id(id: &str, prefix: &str) -> Option<u64> {
        id.strip_prefix(prefix)?.parse::<u64>().ok()
    }

    fn summary_text(&self) -> String {
        let done = self.tasks.iter().filter(|task| task.done).count();
        format!("{} tasks ({} done)", self.tasks.len(), done)
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
        ui.vertical(|ui| {
            ui.widgets()
                .label()
                .id(TITLE_ID)
                .text("Task Manager")
                .font_size(26)
                .show();

            ui.widgets()
                .label()
                .id(SUMMARY_ID)
                .text(self.summary_text())
                .font_size(16)
                .text_color(colors::rgb(colors::TEXT_SECONDARY))
                .show();

            ui.widgets()
                .div()
                .id(PANEL_ID)
                .width(860)
                .padding(16)
                .gap(12)
                .background(colors::rgb(colors::PANEL_BG))
                .border(colors::rgb(colors::PANEL_BORDER), 1)
                .radius(12)
                .show(|ui| {
                    ui.horizontal(|ui| {
                        ui.widgets()
                            .text_box()
                            .id(INPUT_ID)
                            .width(520)
                            .height(44)
                            .font_size(18)
                            .placeholder("Add a new task...")
                            .show();

                        ui.widgets()
                            .button()
                            .id(ADD_BUTTON_ID)
                            .width(100)
                            .height(44)
                            .font_size(16)
                            .text("Add")
                            .show();

                        ui.widgets()
                            .button()
                            .id(CLEAR_DONE_BUTTON_ID)
                            .width(120)
                            .height(44)
                            .font_size(16)
                            .text("Clear done")
                            .show();
                    });

                    for task in &self.tasks {
                        let row_id = format!("{}{}", TASK_ROW_PREFIX, task.id);
                        let check_id = format!("{}{}", TASK_CHECK_PREFIX, task.id);
                        let delete_id = format!("{}{}", TASK_DELETE_PREFIX, task.id);

                        ui.widgets()
                            .div()
                            .id(row_id)
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

                                ui.widgets()
                                    .checkbox()
                                    .id(check_id)
                                    .width(620)
                                    .height(36)
                                    .font_size(18)
                                    .text(label)
                                    .checked(task.done)
                                    .show();

                                ui.widgets()
                                    .button()
                                    .id(delete_id)
                                    .width(84)
                                    .height(36)
                                    .font_size(15)
                                    .text("Delete")
                                    .show();
                            });
                    }
                });
        });
    }

    fn update(&mut self, ui: &mut Ui<'_>) {
        for event in ui.events().drain_events() {
            match event {
                UiEvent::TextBoxChanged { id, text } if id == INPUT_ID => {
                    self.draft = text;
                }
                UiEvent::ButtonClicked { id } if id == ADD_BUTTON_ID => {
                    self.add_task();
                    ui.state().text_box().set_text(INPUT_ID, "");
                }
                UiEvent::ButtonClicked { id } if id == CLEAR_DONE_BUTTON_ID => {
                    self.clear_done();
                }
                UiEvent::ButtonClicked { id } => {
                    if let Some(task_id) = Self::parse_task_id(&id, TASK_DELETE_PREFIX) {
                        self.tasks.retain(|task| task.id != task_id);
                    }
                }
                UiEvent::CheckboxChanged { id, checked } => {
                    if let Some(task_id) = Self::parse_task_id(&id, TASK_CHECK_PREFIX)
                        && let Some(task) = self.tasks.iter_mut().find(|task| task.id == task_id)
                    {
                        task.done = checked;
                    }
                }
                _ => {}
            }
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
    });
}
