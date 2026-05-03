#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use runo::run;
use runo_studio::RunoStudioApp;

fn main() {
    run(RunoStudioApp::default());
}
