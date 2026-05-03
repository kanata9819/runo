#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WidgetKind {
    Button,
    Label,
    TextBox,
}

impl WidgetKind {
    pub fn display_name(self) -> &'static str {
        match self {
            Self::Button => "Button",
            Self::Label => "Label",
            Self::TextBox => "TextBox",
        }
    }

    pub fn default_text(self) -> &'static str {
        match self {
            Self::Button => "Button",
            Self::Label => "Label",
            Self::TextBox => "",
        }
    }

    fn id_prefix(self) -> &'static str {
        match self {
            Self::Button => "button",
            Self::Label => "label",
            Self::TextBox => "text_box",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WidgetSpec {
    pub uid: u64,
    pub id: String,
    pub kind: WidgetKind,
    pub text: String,
    pub width: u32,
    pub height: u32,
    pub x: f64,
    pub y: f64,
}

impl WidgetSpec {
    pub fn summary(&self) -> String {
        format!(
            "{}  {}  {}x{}",
            self.id,
            self.kind.display_name(),
            self.width,
            self.height
        )
    }

    fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.x
            && x <= self.x + f64::from(self.width)
            && y >= self.y
            && y <= self.y + f64::from(self.height)
    }
}

#[derive(Clone, Debug)]
pub struct StudioDocument {
    widgets: Vec<WidgetSpec>,
    selected_uid: Option<u64>,
    next_uid: u64,
}

impl Default for StudioDocument {
    fn default() -> Self {
        let mut document = Self {
            widgets: Vec::new(),
            selected_uid: None,
            next_uid: 1,
        };
        document.add_widget(WidgetKind::Button);
        document.add_widget(WidgetKind::Label);
        document.add_widget(WidgetKind::TextBox);
        document.select_first();
        document
    }
}

impl StudioDocument {
    pub fn widgets(&self) -> &[WidgetSpec] {
        &self.widgets
    }

    pub fn selected_uid(&self) -> Option<u64> {
        self.selected_uid
    }

    pub fn selected(&self) -> Option<&WidgetSpec> {
        let uid = self.selected_uid?;
        self.widgets.iter().find(|widget| widget.uid == uid)
    }

    pub fn widget(&self, uid: u64) -> Option<&WidgetSpec> {
        self.widgets.iter().find(|widget| widget.uid == uid)
    }

    pub fn widget_at(&self, x: f64, y: f64) -> Option<&WidgetSpec> {
        self.widgets
            .iter()
            .rev()
            .find(|widget| widget.contains(x, y))
    }

    pub fn add_widget(&mut self, kind: WidgetKind) {
        let offset = self.widgets.len() as f64 * 18.0;
        self.add_widget_at(kind, 24.0 + offset, 64.0 + offset);
    }

    pub fn add_widget_at(&mut self, kind: WidgetKind, x: f64, y: f64) {
        let uid = self.next_uid;
        self.next_uid += 1;

        self.widgets.push(WidgetSpec {
            uid,
            id: format!("{}_{}", kind.id_prefix(), uid),
            kind,
            text: kind.default_text().to_string(),
            width: default_width(kind),
            height: default_height(kind),
            x,
            y,
        });
        self.selected_uid = Some(uid);
    }

    pub fn select(&mut self, uid: u64) {
        if self.widgets.iter().any(|widget| widget.uid == uid) {
            self.selected_uid = Some(uid);
        }
    }

    pub fn move_widget_to(&mut self, uid: u64, x: f64, y: f64) {
        if let Some(widget) = self.widgets.iter_mut().find(|widget| widget.uid == uid) {
            widget.x = x;
            widget.y = y;
        }
    }

    pub fn delete_selected(&mut self) {
        let Some(selected_uid) = self.selected_uid else {
            return;
        };
        let Some(index) = self.index_of(selected_uid) else {
            self.selected_uid = None;
            return;
        };

        self.widgets.remove(index);
        self.selected_uid = self
            .widgets
            .get(index)
            .or_else(|| index.checked_sub(1).and_then(|prev| self.widgets.get(prev)))
            .map(|widget| widget.uid);
    }

    pub fn move_selected_up(&mut self) {
        let Some(selected_uid) = self.selected_uid else {
            return;
        };
        let Some(index) = self.index_of(selected_uid) else {
            return;
        };
        if index > 0 {
            self.widgets.swap(index, index - 1);
        }
    }

    pub fn move_selected_down(&mut self) {
        let Some(selected_uid) = self.selected_uid else {
            return;
        };
        let Some(index) = self.index_of(selected_uid) else {
            return;
        };
        if index + 1 < self.widgets.len() {
            self.widgets.swap(index, index + 1);
        }
    }

    pub fn set_selected_text(&mut self, text: String) {
        if let Some(widget) = self.selected_mut() {
            widget.text = text;
        }
    }

    pub fn set_selected_width_from_text(&mut self, text: &str) {
        if let Some(width) = parse_dimension(text)
            && let Some(widget) = self.selected_mut()
        {
            widget.width = width;
        }
    }

    pub fn set_selected_height_from_text(&mut self, text: &str) {
        if let Some(height) = parse_dimension(text)
            && let Some(widget) = self.selected_mut()
        {
            widget.height = height;
        }
    }

    fn select_first(&mut self) {
        self.selected_uid = self.widgets.first().map(|widget| widget.uid);
    }

    fn selected_mut(&mut self) -> Option<&mut WidgetSpec> {
        let uid = self.selected_uid?;
        self.widgets.iter_mut().find(|widget| widget.uid == uid)
    }

    fn index_of(&self, uid: u64) -> Option<usize> {
        self.widgets.iter().position(|widget| widget.uid == uid)
    }
}

fn default_width(kind: WidgetKind) -> u32 {
    match kind {
        WidgetKind::Button => 180,
        WidgetKind::Label => 240,
        WidgetKind::TextBox => 260,
    }
}

fn default_height(kind: WidgetKind) -> u32 {
    match kind {
        WidgetKind::Button => 48,
        WidgetKind::Label => 32,
        WidgetKind::TextBox => 44,
    }
}

fn parse_dimension(text: &str) -> Option<u32> {
    let value = text.trim().parse::<u32>().ok()?;
    Some(value.clamp(24, 1000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_widget_updates_position_without_changing_selection() {
        let mut document = StudioDocument::default();
        let uid = document.selected_uid().unwrap();

        document.move_widget_to(uid, 123.0, 234.0);

        let widget = document.widget(uid).unwrap();
        assert_eq!((widget.x, widget.y), (123.0, 234.0));
        assert_eq!(document.selected_uid(), Some(uid));
    }

    #[test]
    fn widget_at_prefers_topmost_widget() {
        let mut document = StudioDocument {
            widgets: Vec::new(),
            selected_uid: None,
            next_uid: 1,
        };
        document.add_widget_at(WidgetKind::Button, 10.0, 10.0);
        document.add_widget_at(WidgetKind::Label, 20.0, 20.0);

        let widget = document.widget_at(24.0, 24.0).unwrap();

        assert_eq!(widget.kind, WidgetKind::Label);
    }
}
