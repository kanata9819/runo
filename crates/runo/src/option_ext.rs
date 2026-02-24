use crate::ui::UiEvents;
use crate::widget::button::ButtonHandle;
use crate::widget::checkbox::CheckboxHandle;
use crate::widget::combo_box::ComboBoxHandle;
use crate::widget::radio_button::RadioButtonHandle;
use crate::widget::slider::SliderHandle;
use crate::widget::text_box::TextBoxHandle;

pub trait OptionalButtonHandleExt {
    fn on_click(&self, events: &mut UiEvents<'_, '_>, f: impl FnOnce());
    fn on_click_with_ui(&self, events: &mut UiEvents<'_, '_>, f: impl FnOnce(&mut crate::Ui<'_>));
}

impl OptionalButtonHandleExt for Option<ButtonHandle> {
    fn on_click(&self, events: &mut UiEvents<'_, '_>, f: impl FnOnce()) {
        if let Some(handle) = self {
            handle.on_click(events, f);
        }
    }

    fn on_click_with_ui(&self, events: &mut UiEvents<'_, '_>, f: impl FnOnce(&mut crate::Ui<'_>)) {
        if let Some(handle) = self {
            handle.on_click_with_ui(events, f);
        }
    }
}

pub trait OptionalTextBoxHandleExt {
    fn on_change(&self, events: &mut UiEvents<'_, '_>, f: impl FnOnce(String));
    fn on_change_with_ui(
        &self,
        events: &mut UiEvents<'_, '_>,
        f: impl FnOnce(&mut crate::Ui<'_>, String),
    );
}

impl OptionalTextBoxHandleExt for Option<TextBoxHandle> {
    fn on_change(&self, events: &mut UiEvents<'_, '_>, f: impl FnOnce(String)) {
        if let Some(handle) = self {
            handle.on_change(events, f);
        }
    }

    fn on_change_with_ui(
        &self,
        events: &mut UiEvents<'_, '_>,
        f: impl FnOnce(&mut crate::Ui<'_>, String),
    ) {
        if let Some(handle) = self {
            handle.on_change_with_ui(events, f);
        }
    }
}

pub trait OptionalCheckboxHandleExt {
    fn on_change(&self, events: &mut UiEvents<'_, '_>, f: impl FnOnce(bool));
    fn on_change_with_ui(
        &self,
        events: &mut UiEvents<'_, '_>,
        f: impl FnOnce(&mut crate::Ui<'_>, bool),
    );
}

impl OptionalCheckboxHandleExt for Option<CheckboxHandle> {
    fn on_change(&self, events: &mut UiEvents<'_, '_>, f: impl FnOnce(bool)) {
        if let Some(handle) = self {
            handle.on_change(events, f);
        }
    }

    fn on_change_with_ui(
        &self,
        events: &mut UiEvents<'_, '_>,
        f: impl FnOnce(&mut crate::Ui<'_>, bool),
    ) {
        if let Some(handle) = self {
            handle.on_change_with_ui(events, f);
        }
    }
}

pub trait OptionalSliderHandleExt {
    fn on_change(&self, events: &mut UiEvents<'_, '_>, f: impl FnOnce(f64));
    fn on_change_with_ui(
        &self,
        events: &mut UiEvents<'_, '_>,
        f: impl FnOnce(&mut crate::Ui<'_>, f64),
    );
}

impl OptionalSliderHandleExt for Option<SliderHandle> {
    fn on_change(&self, events: &mut UiEvents<'_, '_>, f: impl FnOnce(f64)) {
        if let Some(handle) = self {
            handle.on_change(events, f);
        }
    }

    fn on_change_with_ui(
        &self,
        events: &mut UiEvents<'_, '_>,
        f: impl FnOnce(&mut crate::Ui<'_>, f64),
    ) {
        if let Some(handle) = self {
            handle.on_change_with_ui(events, f);
        }
    }
}

pub trait OptionalRadioButtonHandleExt {
    fn on_change(&self, events: &mut UiEvents<'_, '_>, f: impl FnOnce(bool));
    fn on_change_with_ui(
        &self,
        events: &mut UiEvents<'_, '_>,
        f: impl FnOnce(&mut crate::Ui<'_>, bool),
    );
}

impl OptionalRadioButtonHandleExt for Option<RadioButtonHandle> {
    fn on_change(&self, events: &mut UiEvents<'_, '_>, f: impl FnOnce(bool)) {
        if let Some(handle) = self {
            handle.on_change(events, f);
        }
    }

    fn on_change_with_ui(
        &self,
        events: &mut UiEvents<'_, '_>,
        f: impl FnOnce(&mut crate::Ui<'_>, bool),
    ) {
        if let Some(handle) = self {
            handle.on_change_with_ui(events, f);
        }
    }
}

pub trait OptionalComboBoxHandleExt {
    fn on_change(&self, events: &mut UiEvents<'_, '_>, f: impl FnOnce(usize, String));
    fn on_change_with_ui(
        &self,
        events: &mut UiEvents<'_, '_>,
        f: impl FnOnce(&mut crate::Ui<'_>, usize, String),
    );
}

impl OptionalComboBoxHandleExt for Option<ComboBoxHandle> {
    fn on_change(&self, events: &mut UiEvents<'_, '_>, f: impl FnOnce(usize, String)) {
        if let Some(handle) = self {
            handle.on_change(events, f);
        }
    }

    fn on_change_with_ui(
        &self,
        events: &mut UiEvents<'_, '_>,
        f: impl FnOnce(&mut crate::Ui<'_>, usize, String),
    ) {
        if let Some(handle) = self {
            handle.on_change_with_ui(events, f);
        }
    }
}
