pub mod canvas_controller;

slint::slint! {
    import { MainWindow } from "ui/main.slint";
    export { MainWindow }
}

use crate::i18n;

pub fn setup_localization(window: &MainWindow) {
    window.set_app_title(slint::SharedString::from(i18n::get_text("app_title")));
    window.set_canvas_placeholder(slint::SharedString::from(i18n::get_text("canvas_placeholder")));
    window.set_properties(slint::SharedString::from(i18n::get_text("properties")));
    window.set_tool_properties(slint::SharedString::from(i18n::get_text("tool_properties")));
    window.set_brush_tool(slint::SharedString::from(i18n::get_text("brush_tool")));
    window.set_shape_tool(slint::SharedString::from(i18n::get_text("shape_tool")));
    window.set_arrow_tool(slint::SharedString::from(i18n::get_text("arrow_tool")));
    window.set_eraser_tool(slint::SharedString::from(i18n::get_text("eraser_tool")));
    window.set_text_tool(slint::SharedString::from(i18n::get_text("text_tool")));
    window.set_selection_tool(slint::SharedString::from(i18n::get_text("selection_tool")));
}
