// defaults.rs
// 开发者只需要使用 t!(cx, "key") 宏，开发者在写代码时使用 t! 宏和对应的键名
// 如果这个键在 defaults.rs 中不存在，会在日志中警告
// 开发者需要在 defaults.rs 中添加对应的默认英文文本，修改默认文本只需要改 defaults.rs 一个地方
// 当加载语言包时，如果找到对应翻译就使用翻译，否则使用默认文本
// 可以通过工具自动导出所有需要翻译的文本
use std::collections::HashMap;
use once_cell::sync::Lazy;

// 全局静态默认文本映射
static DEFAULT_TEXTS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut texts = HashMap::new();

    texts.insert("i18n.menu.zed", "Zed");
    texts.insert("i18n.menu.zed.about_zed", "About Zed…");
    texts.insert("i18n.menu.zed.check_for_updates", "Check for Updates");
    texts.insert("i18n.menu.zed.settings.open_settings", "Open Settings");
    texts.insert("i18n.menu.zed.settings.open_key_bindings", "Open Key Bindings");
    texts.insert("i18n.menu.zed.settings.open_default_settings", "Open Default Settings");
    texts.insert("i18n.menu.zed.settings.open_default_key_bindings", "Open Default Key Bindings");
    texts.insert("i18n.menu.zed.settings.open_project_settings", "Open Project Settings");
    texts.insert("i18n.menu.zed.settings.select_theme", "Select Theme...");
    texts.insert("i18n.menu.zed.extensions", "Extensions");
    texts.insert("i18n.menu.zed.install_cli", "Install CLI");
    texts.insert("i18n.menu.zed.hide_zed", "Hide Zed");
    texts.insert("i18n.menu.zed.hide_others", "Hide Others");
    texts.insert("i18n.menu.zed.show_all", "Show All");
    texts.insert("i18n.menu.zed.quit", "Quit");
    texts.insert("i18n.menu.file", "File");
    texts.insert("i18n.menu.file.new", "New");
    texts.insert("i18n.menu.file.new_window", "New Window");
    texts.insert("i18n.menu.file.open_file", "Open File...");
    texts.insert("i18n.menu.file.macos", "macos");
    texts.insert("i18n.menu.file.open_recent", "Open Recent...");
    texts.insert("i18n.menu.file.open_remote", "Open Remote...");
    texts.insert("i18n.menu.file.add_folder_to_project", "Add Folder to Project…");
    texts.insert("i18n.menu.file.save", "Save");
    texts.insert("i18n.menu.file.save_as", "Save As…");
    texts.insert("i18n.menu.file.save_all", "Save All");
    texts.insert("i18n.menu.file.close_editor", "Close Editor");
    texts.insert("i18n.menu.file.close_window", "Close Window");
    texts.insert("i18n.menu.edit", "Edit");
    texts.insert("i18n.menu.edit.undo", "Undo");
    texts.insert("i18n.menu.edit.redo", "Redo");
    texts.insert("i18n.menu.edit.cut", "Cut");
    texts.insert("i18n.menu.edit.copy", "Copy");
    texts.insert("i18n.menu.edit.copy_and_trim", "Copy and Trim");
    texts.insert("i18n.menu.edit.paste", "Paste");
    texts.insert("i18n.menu.edit.find", "Find");
    texts.insert("i18n.menu.edit.find_in_project", "Find In Project");
    texts.insert("i18n.menu.edit.toggle_line_comment", "Toggle Line Comment");
    texts.insert("i18n.menu.selection", "Selection");
    texts.insert("i18n.menu.selection.select_all", "Select All");
    texts.insert("i18n.menu.selection.expand_selection", "Expand Selection");
    texts.insert("i18n.menu.selection.shrink_selection", "Shrink Selection");
    texts.insert("i18n.menu.selection.add_cursor_above", "Add Cursor Above");
    texts.insert("i18n.menu.selection.add_cursor_below", "Add Cursor Below");
    texts.insert("i18n.menu.selection.select_next_occurrence", "Select Next Occurrence");
    texts.insert("i18n.menu.selection.move_line_up", "Move Line Up");
    texts.insert("i18n.menu.selection.move_line_down", "Move Line Down");
    texts.insert("i18n.menu.selection.duplicate_selection", "Duplicate Selection");
    texts.insert("i18n.menu.view", "View");
    texts.insert("i18n.menu.view.zoom_in", "Zoom In");
    texts.insert("i18n.menu.view.zoom_out", "Zoom Out");
    texts.insert("i18n.menu.view.reset_zoom", "Reset Zoom");
    texts.insert("i18n.menu.view.toggle_left_dock", "Toggle Left Dock");
    texts.insert("i18n.menu.view.toggle_right_dock", "Toggle Right Dock");
    texts.insert("i18n.menu.view.toggle_bottom_dock", "Toggle Bottom Dock");
    texts.insert("i18n.menu.view.close_all_docks", "Close All Docks");
    texts.insert("i18n.menu.view.editor_layout.split_up", "Split Up");
    texts.insert("i18n.menu.view.editor_layout.split_down", "Split Down");
    texts.insert("i18n.menu.view.editor_layout.split_left", "Split Left");
    texts.insert("i18n.menu.view.editor_layout.split_right", "Split Right");
    texts.insert("i18n.menu.view.project_panel", "Project Panel");
    texts.insert("i18n.menu.view.outline_panel", "Outline Panel");
    texts.insert("i18n.menu.view.collab_panel", "Collab Panel");
    texts.insert("i18n.menu.view.terminal_panel", "Terminal Panel");
    texts.insert("i18n.menu.view.diagnostics", "Diagnostics");
    texts.insert("i18n.menu.go", "Go");
    texts.insert("i18n.menu.go.back", "Back");
    texts.insert("i18n.menu.go.forward", "Forward");
    texts.insert("i18n.menu.go.command_palette", "Command Palette...");
    texts.insert("i18n.menu.go.go_to_file", "Go to File...");
    texts.insert("i18n.menu.go.go_to_symbol_in_project", "Go to Symbol in Project");
    texts.insert("i18n.menu.go.go_to_symbol_in_editor", "Go to Symbol in Editor...");
    texts.insert("i18n.menu.go.go_to_line/column", "Go to Line/Column...");
    texts.insert("i18n.menu.go.go_to_definition", "Go to Definition");
    texts.insert("i18n.menu.go.go_to_declaration", "Go to Declaration");
    texts.insert("i18n.menu.go.go_to_type_definition", "Go to Type Definition");
    texts.insert("i18n.menu.go.find_all_references", "Find All References");
    texts.insert("i18n.menu.go.next_problem", "Next Problem");
    texts.insert("i18n.menu.go.previous_problem", "Previous Problem");
    texts.insert("i18n.menu.window", "Window");
    texts.insert("i18n.menu.window.minimize", "Minimize");
    texts.insert("i18n.menu.window.zoom", "Zoom");
    texts.insert("i18n.menu.help", "Help");
    texts.insert("i18n.menu.help.view_telemetry", "View Telemetry");
    texts.insert("i18n.menu.help.view_dependency_licenses", "View Dependency Licenses");
    texts.insert("i18n.menu.help.show_welcome", "Show Welcome");
    texts.insert("i18n.menu.help.give_feedback", "Give Feedback...");
    texts.insert("i18n.menu.help.documentation", "Documentation");
    texts.insert("i18n.menu.help.zed_twitter", "Zed Twitter");
    texts.insert("i18n.menu.help.join_the_team", "Join the Team");
    texts.insert("i18n.menu.other.settings", "Settings");
    texts.insert("i18n.menu.other.services", "Services");
    texts.insert("i18n.menu.other.open_folder", "Open Folder...");
    texts.insert("i18n.menu.other.open", "Open…");
    texts.insert("i18n.menu.other.editor_layout", "Editor Layout");

    texts
});

/// 获取默认文本
pub fn get_default_text(key: &str) -> Option<&'static str> {
    DEFAULT_TEXTS.get(key).copied()
}

/// 获取所有默认文本键
pub fn get_all_default_text_keys() -> impl Iterator<Item = &'static str> {
    DEFAULT_TEXTS.keys().copied()
}
