use super::*;
use vello::kurbo::Rect;

#[test]
fn centered_computes_expected_baseline() {
    let rect = Rect::new(0.0, 10.0, 100.0, 50.0);
    let baseline = centered(rect, 20.0);
    assert_eq!(baseline, 37.0);
}

#[test]
fn top_aligned_computes_expected_baseline() {
    let rect = Rect::new(0.0, 10.0, 100.0, 50.0);
    let baseline = top_aligned(rect, 20.0);
    assert_eq!(baseline, 30.0);
}
