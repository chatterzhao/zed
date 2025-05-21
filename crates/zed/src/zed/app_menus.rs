use collab_ui::collab_panel;
use gpui::{Menu, MenuItem, OsAction};
use terminal_view::terminal_panel;

pub fn app_menus() -> Vec<Menu> {
    use zed_actions::Quit;

    vec![
        Menu {
            name: t!(cx, "i18n.menu.zed").into(),
            items: vec![
                MenuItem::action(t!(cx, "i18n.menu.zed.about_zed"), zed_actions::About),
                MenuItem::action(t!(cx, "i18n.menu.zed.check_for_updates"), auto_update::Check),
                MenuItem::separator(),
                MenuItem::submenu(Menu {
                    name: t!(cx, "i18n.menu.other.settings").into(),
                    items: vec![
                        MenuItem::action(t!(cx, "i18n.menu.zed.settings.open_settings"), super::OpenSettings),
                        MenuItem::action(t!(cx, "i18n.menu.zed.settings.open_key_bindings"), zed_actions::OpenKeymap),
                        MenuItem::action(t!(cx, "i18n.menu.zed.settings.open_default_settings"), super::OpenDefaultSettings),
                        MenuItem::action(
                            t!(cx, "i18n.menu.zed.settings.open_default_key_bindings"),
                            zed_actions::OpenDefaultKeymap,
                        ),
                        MenuItem::action(t!(cx, "i18n.menu.zed.settings.open_project_settings"), super::OpenProjectSettings),
                        MenuItem::action(
                            t!(cx, "i18n.menu.zed.settings.select_theme"),
                            zed_actions::theme_selector::Toggle::default(),
                        ),
                    ],
                }),
                MenuItem::separator(),
                MenuItem::submenu(Menu {
                    name: t!(cx, "i18n.menu.other.services").into(),
                    items: vec![],
                }),
                MenuItem::separator(),
                MenuItem::action(t!(cx, "i18n.menu.zed.extensions"), zed_actions::Extensions::default()),
                MenuItem::action(t!(cx, "i18n.menu.zed.install_cli"), install_cli::Install),
                MenuItem::separator(),
                #[cfg(target_os = "macos")]
                MenuItem::action(t!(cx, "i18n.menu.zed.hide_zed"), super::Hide),
                #[cfg(target_os = "macos")]
                MenuItem::action(t!(cx, "i18n.menu.zed.hide_others"), super::HideOthers),
                #[cfg(target_os = "macos")]
                MenuItem::action(t!(cx, "i18n.menu.zed.show_all"), super::ShowAll),
                MenuItem::action(t!(cx, "i18n.menu.zed.quit"), Quit),
            ],
        },
        Menu {
            name: t!(cx, "i18n.menu.file").into(),
            items: vec![
                MenuItem::action(t!(cx, "i18n.menu.file.new"), workspace::NewFile),
                MenuItem::action(t!(cx, "i18n.menu.file.new_window"), workspace::NewWindow),
                MenuItem::separator(),
                #[cfg(not(target_os = "macos"))]
                MenuItem::action(t!(cx, "i18n.menu.file.open_file"), workspace::OpenFiles),
                MenuItem::action(
                    if cfg!(not(target_os = "macos")) {
                        t!(cx, "i18n.menu.other.open_folder")
                    } else {
                        t!(cx, "i18n.menu.other.open")
                    },
                    workspace::Open,
                ),
                MenuItem::action(
                    t!(cx, "i18n.menu.file.open_recent"),
                    zed_actions::OpenRecent {
                        create_new_window: true,
                    },
                ),
                MenuItem::action(t!(cx, "i18n.menu.file.open_remote"), zed_actions::OpenRemote),
                MenuItem::separator(),
                MenuItem::action(t!(cx, "i18n.menu.file.add_folder_to_project"), workspace::AddFolderToProject),
                MenuItem::separator(),
                MenuItem::action(t!(cx, "i18n.menu.file.save"), workspace::Save { save_intent: None }),
                MenuItem::action(t!(cx, "i18n.menu.file.save_as"), workspace::SaveAs),
                MenuItem::action(t!(cx, "i18n.menu.file.save_all"), workspace::SaveAll { save_intent: None }),
                MenuItem::separator(),
                MenuItem::action(
                    t!(cx, "i18n.menu.file.close_editor"),
                    workspace::CloseActiveItem {
                        save_intent: None,
                        close_pinned: true,
                    },
                ),
                MenuItem::action(t!(cx, "i18n.menu.file.close_window"), workspace::CloseWindow),
            ],
        },
        Menu {
            name: t!(cx, "i18n.menu.edit").into(),
            items: vec![
                MenuItem::os_action(t!(cx, "i18n.menu.edit.undo"), editor::actions::Undo, OsAction::Undo),
                MenuItem::os_action(t!(cx, "i18n.menu.edit.redo"), editor::actions::Redo, OsAction::Redo),
                MenuItem::separator(),
                MenuItem::os_action(t!(cx, "i18n.menu.edit.cut"), editor::actions::Cut, OsAction::Cut),
                MenuItem::os_action(t!(cx, "i18n.menu.edit.copy"), editor::actions::Copy, OsAction::Copy),
                MenuItem::action(t!(cx, "i18n.menu.edit.copy_and_trim"), editor::actions::CopyAndTrim),
                MenuItem::os_action(t!(cx, "i18n.menu.edit.paste"), editor::actions::Paste, OsAction::Paste),
                MenuItem::separator(),
                MenuItem::action(t!(cx, "i18n.menu.edit.find"), search::buffer_search::Deploy::find()),
                MenuItem::action(t!(cx, "i18n.menu.edit.find_in_project"), workspace::DeploySearch::find()),
                MenuItem::separator(),
                MenuItem::action(
                    t!(cx, "i18n.menu.edit.toggle_line_comment"),
                    editor::actions::ToggleComments::default(),
                ),
            ],
        },
        Menu {
            name: t!(cx, "i18n.menu.selection").into(),
            items: vec![
                MenuItem::os_action(
                    t!(cx, "i18n.menu.selection.select_all"),
                    editor::actions::SelectAll,
                    OsAction::SelectAll,
                ),
                MenuItem::action(t!(cx, "i18n.menu.selection.expand_selection"), editor::actions::SelectLargerSyntaxNode),
                MenuItem::action(t!(cx, "i18n.menu.selection.shrink_selection"), editor::actions::SelectSmallerSyntaxNode),
                MenuItem::separator(),
                MenuItem::action(t!(cx, "i18n.menu.selection.add_cursor_above"), editor::actions::AddSelectionAbove),
                MenuItem::action(t!(cx, "i18n.menu.selection.add_cursor_below"), editor::actions::AddSelectionBelow),
                MenuItem::action(
                    t!(cx, "i18n.menu.selection.select_next_occurrence"),
                    editor::actions::SelectNext {
                        replace_newest: false,
                    },
                ),
                MenuItem::separator(),
                MenuItem::action(t!(cx, "i18n.menu.selection.move_line_up"), editor::actions::MoveLineUp),
                MenuItem::action(t!(cx, "i18n.menu.selection.move_line_down"), editor::actions::MoveLineDown),
                MenuItem::action(t!(cx, "i18n.menu.selection.duplicate_selection"), editor::actions::DuplicateLineDown),
            ],
        },
        Menu {
            name: t!(cx, "i18n.menu.view").into(),
            items: vec![
                MenuItem::action(
                    t!(cx, "i18n.menu.view.zoom_in"),
                    zed_actions::IncreaseBufferFontSize { persist: true },
                ),
                MenuItem::action(
                    t!(cx, "i18n.menu.view.zoom_out"),
                    zed_actions::DecreaseBufferFontSize { persist: true },
                ),
                MenuItem::action(
                    t!(cx, "i18n.menu.view.reset_zoom"),
                    zed_actions::ResetBufferFontSize { persist: true },
                ),
                MenuItem::separator(),
                MenuItem::action(t!(cx, "i18n.menu.view.toggle_left_dock"), workspace::ToggleLeftDock),
                MenuItem::action(t!(cx, "i18n.menu.view.toggle_right_dock"), workspace::ToggleRightDock),
                MenuItem::action(t!(cx, "i18n.menu.view.toggle_bottom_dock"), workspace::ToggleBottomDock),
                MenuItem::action(t!(cx, "i18n.menu.view.close_all_docks"), workspace::CloseAllDocks),
                MenuItem::submenu(Menu {
                    name: t!(cx, "i18n.menu.other.editor_layout").into(),
                    items: vec![
                        MenuItem::action(t!(cx, "i18n.menu.view.editor_layout.split_up"), workspace::SplitUp),
                        MenuItem::action(t!(cx, "i18n.menu.view.editor_layout.split_down"), workspace::SplitDown),
                        MenuItem::action(t!(cx, "i18n.menu.view.editor_layout.split_left"), workspace::SplitLeft),
                        MenuItem::action(t!(cx, "i18n.menu.view.editor_layout.split_right"), workspace::SplitRight),
                    ],
                }),
                MenuItem::separator(),
                MenuItem::action(t!(cx, "i18n.menu.view.project_panel"), project_panel::ToggleFocus),
                MenuItem::action(t!(cx, "i18n.menu.view.outline_panel"), outline_panel::ToggleFocus),
                MenuItem::action(t!(cx, "i18n.menu.view.collab_panel"), collab_panel::ToggleFocus),
                MenuItem::action(t!(cx, "i18n.menu.view.terminal_panel"), terminal_panel::ToggleFocus),
                MenuItem::separator(),
                MenuItem::action(t!(cx, "i18n.menu.view.diagnostics"), diagnostics::Deploy),
                MenuItem::separator(),
            ],
        },
        Menu {
            name: t!(cx, "i18n.menu.go").into(),
            items: vec![
                MenuItem::action(t!(cx, "i18n.menu.go.back"), workspace::GoBack),
                MenuItem::action(t!(cx, "i18n.menu.go.forward"), workspace::GoForward),
                MenuItem::separator(),
                MenuItem::action(t!(cx, "i18n.menu.go.command_palette"), zed_actions::command_palette::Toggle),
                MenuItem::separator(),
                MenuItem::action(t!(cx, "i18n.menu.go.go_to_file"), workspace::ToggleFileFinder::default()),
                // MenuItem::action(t!(cx, "i18n.menu.go.go_to_symbol_in_project"), project_symbols::Toggle),
                MenuItem::action(
                    t!(cx, "i18n.menu.go.go_to_symbol_in_editor"),
                    zed_actions::outline::ToggleOutline,
                ),
                MenuItem::action(t!(cx, "i18n.menu.go.go_to_line/column"), editor::actions::ToggleGoToLine),
                MenuItem::separator(),
                MenuItem::action(t!(cx, "i18n.menu.go.go_to_definition"), editor::actions::GoToDefinition),
                MenuItem::action(t!(cx, "i18n.menu.go.go_to_declaration"), editor::actions::GoToDeclaration),
                MenuItem::action(t!(cx, "i18n.menu.go.go_to_type_definition"), editor::actions::GoToTypeDefinition),
                MenuItem::action(t!(cx, "i18n.menu.go.find_all_references"), editor::actions::FindAllReferences),
                MenuItem::separator(),
                MenuItem::action(t!(cx, "i18n.menu.go.next_problem"), editor::actions::GoToDiagnostic),
                MenuItem::action(t!(cx, "i18n.menu.go.previous_problem"), editor::actions::GoToPreviousDiagnostic),
            ],
        },
        Menu {
            name: t!(cx, "i18n.menu.window").into(),
            items: vec![
                MenuItem::action(t!(cx, "i18n.menu.window.minimize"), super::Minimize),
                MenuItem::action(t!(cx, "i18n.menu.window.zoom"), super::Zoom),
                MenuItem::separator(),
            ],
        },
        Menu {
            name: t!(cx, "i18n.menu.help").into(),
            items: vec![
                MenuItem::action(t!(cx, "i18n.menu.help.view_telemetry"), zed_actions::OpenTelemetryLog),
                MenuItem::action(t!(cx, "i18n.menu.help.view_dependency_licenses"), zed_actions::OpenLicenses),
                MenuItem::action(t!(cx, "i18n.menu.help.show_welcome"), workspace::Welcome),
                MenuItem::action(t!(cx, "i18n.menu.help.give_feedback"), zed_actions::feedback::GiveFeedback),
                MenuItem::separator(),
                MenuItem::action(
                    t!(cx, "i18n.menu.help.documentation"),
                    super::OpenBrowser {
                        url: "https://zed.dev/docs".into(),
                    },
                ),
                MenuItem::action(
                    t!(cx, "i18n.menu.help.zed_twitter"),
                    super::OpenBrowser {
                        url: "https://twitter.com/zeddotdev".into(),
                    },
                ),
                MenuItem::action(
                    t!(cx, "i18n.menu.help.join_the_team"),
                    super::OpenBrowser {
                        url: "https://zed.dev/jobs".into(),
                    },
                ),
            ],
        },
    ]
}
