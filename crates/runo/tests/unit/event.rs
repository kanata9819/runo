    use super::UiEvent;
    use crate::widget::slider::SliderHandle;

    #[test]
    fn ui_event_is_cloneable() {
        let event = UiEvent::SliderChanged {
            slider: SliderHandle::new("s".to_string()),
            value: 0.5,
        };
        let cloned = event.clone();
        match cloned {
            UiEvent::SliderChanged { slider, value } => {
                assert_eq!(slider.id(), "s");
                assert!((value - 0.5).abs() < f64::EPSILON);
            }
            _ => panic!("unexpected variant"),
        }
    }
