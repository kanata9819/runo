
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use super::*;

#[test]
fn load_font_from_nonexistent_path_returns_none() {
    let path = "/tmp/runo_font_does_not_exist.ttf";
    assert!(load_font_from_path(path).is_none());
}

#[test]
fn load_font_from_existing_file_returns_some_font_data() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("runo_font_test_{unique}.bin"));
    let mut file = std::fs::File::create(&path).expect("create temp file");
    file.write_all(&[1_u8, 2, 3, 4]).expect("write temp bytes");

    let loaded = load_font_from_path(path.to_str().expect("path utf-8"));
    assert!(loaded.is_some());

    let _ = std::fs::remove_file(path);
}
