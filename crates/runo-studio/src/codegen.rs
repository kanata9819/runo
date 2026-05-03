use crate::document::{StudioDocument, WidgetKind, WidgetSpec};

pub fn generate_rust_code(document: &StudioDocument) -> String {
    let mut code = String::new();
    code.push_str("use runo::{EventBindings, RunoApplication, Ui, run};\n\n");
    code.push_str("struct GeneratedApp;\n\n");
    code.push_str("impl RunoApplication for GeneratedApp {\n");
    code.push_str("    fn build(&mut self, ui: &mut Ui<'_>) -> EventBindings<()> {\n");
    code.push_str("        ui.vertical(|ui| {\n");

    for widget in document.widgets() {
        code.push_str(&format!(
            "            ui.at({:.1}, {:.1}, |ui| {{\n",
            widget.x, widget.y
        ));
        append_widget(&mut code, widget);
        code.push_str("            });\n");
    }

    code.push_str("        });\n");
    code.push_str("        EventBindings::new()\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");
    code.push_str("fn main() {\n");
    code.push_str("    run(GeneratedApp);\n");
    code.push_str("}\n");
    code
}

fn append_widget(code: &mut String, widget: &WidgetSpec) {
    match widget.kind {
        WidgetKind::Button => append_common_widget(code, "button", widget, true),
        WidgetKind::Label => append_common_widget(code, "label", widget, true),
        WidgetKind::TextBox => append_common_widget(code, "text_box", widget, false),
    }
}

fn append_common_widget(code: &mut String, builder: &str, widget: &WidgetSpec, use_text: bool) {
    code.push_str("                ui.widgets()\n");
    code.push_str(&format!("                    .{}()\n", builder));
    code.push_str(&format!("                    .id(\"{}\")\n", widget.id));
    code.push_str(&format!("                    .width({})\n", widget.width));
    code.push_str(&format!("                    .height({})\n", widget.height));

    if use_text || !widget.text.is_empty() {
        code.push_str(&format!(
            "                    .text(\"{}\")\n",
            escape_rust_string(&widget.text)
        ));
    }

    code.push_str("                    .show();\n");
}

fn escape_rust_string(value: &str) -> String {
    value
        .chars()
        .flat_map(|ch| ch.escape_default())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::{StudioDocument, WidgetKind};

    #[test]
    fn generated_code_contains_widgets_in_document_order() {
        let mut document = StudioDocument::default();
        document.add_widget(WidgetKind::Button);

        let code = generate_rust_code(&document);
        assert!(code.contains("impl RunoApplication for GeneratedApp"));
        assert!(code.contains(".button()"));
        assert!(code.contains(".label()"));
        assert!(code.contains(".text_box()"));
    }

    #[test]
    fn generated_code_escapes_text() {
        let mut document = StudioDocument::default();
        document.set_selected_text("say \"hi\"".to_string());

        let code = generate_rust_code(&document);
        assert!(code.contains("say \\\"hi\\\""));
    }
}
