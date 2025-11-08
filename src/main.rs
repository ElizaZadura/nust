use anyhow::Result;
use eframe::{
    NativeOptions,
    egui::{self, KeyboardShortcut, Modifiers},
};
use std::{fs, path::PathBuf};

#[derive(Default)]
struct Pane {
    title: String,
    path: Option<PathBuf>,
    text: String,
    dirty: bool,
}

impl Pane {
    fn load_from(&mut self, p: PathBuf) -> Result<()> {
        self.text = fs::read_to_string(&p).unwrap_or_default();
        self.title = p
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        self.path = Some(p);
        self.dirty = false;
        Ok(())
    }
    fn save_as(&mut self, p: PathBuf) -> Result<()> {
        fs::write(&p, self.text.as_bytes())?;
        self.title = p
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        self.path = Some(p);
        self.dirty = false;
        Ok(())
    }
    fn save(&mut self) -> Result<()> {
        if let Some(p) = self.path.clone() {
            fs::write(p, self.text.as_bytes())?;
            self.dirty = false;
            Ok(())
        } else {
            Err(anyhow::anyhow!("no path"))
        }
    }
}

struct App {
    left: Pane,
    right: Pane,
    status: String,
    manual_path: String,
    focused_pane: FocusedPane,
    save_as_path: String,
    show_save_as_input: bool,
    show_split_view: bool,
    actions: Vec<Action>,
    show_command_palette: bool,
    command_palette_query: String,
    command_palette_selected: usize,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum FocusedPane {
    Left,
    Right,
}

#[derive(Clone, Copy)]
struct Action {
    id: &'static str,
    label: &'static str,
    shortcut: Option<KeyboardShortcut>,
    action: AppAction,
}

impl Action {
    const fn new(
        id: &'static str,
        label: &'static str,
        shortcut: Option<KeyboardShortcut>,
        action: AppAction,
    ) -> Self {
        Self {
            id,
            label,
            shortcut,
            action,
        }
    }
}

#[derive(Clone, Copy)]
enum AppAction {
    OpenFile,
    SaveFocused,
    SaveAsFocused,
    QuickSaveFocused,
    CloseFocused,
    ShowSplitView,
    ShowLeftOnly,
    ShowRightOnly,
}

impl Default for App {
    fn default() -> Self {
        Self {
            left: Pane {
                title: "left".into(),
                ..Default::default()
            },
            right: Pane {
                title: "right".into(),
                ..Default::default()
            },
            status: "ready".into(),
            manual_path: "target/quick_saves/output.txt".into(),
            focused_pane: FocusedPane::Left,
            save_as_path: "".into(),
            show_save_as_input: false,
            show_split_view: true,
            actions: Self::registered_actions(),
            show_command_palette: false,
            command_palette_query: String::new(),
            command_palette_selected: 0,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.process_shortcuts(ctx);

        // Top menu
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.button("📋 Command Palette").clicked() {
                    if self.show_command_palette {
                        self.close_command_palette();
                    } else {
                        self.open_command_palette();
                    }
                }
                ui.label("(Ctrl+Shift+P)");
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Save to:");
                    ui.text_edit_singleline(&mut self.manual_path);
                    if ui.button("Save").clicked() {
                        self.manual_save();
                    }
                });
                ui.separator();
                if ui.button("Exit").clicked() {
                    std::process::exit(0);
                }
            });
        });

        // Save As dialog
        if self.show_save_as_input {
            let mut should_save = false;
            let mut should_cancel = false;
            let mut save_path = String::new();

            egui::Window::new("Save As")
                .open(&mut self.show_save_as_input)
                .show(ctx, |ui| {
                    ui.label("Enter filename:");
                    ui.text_edit_singleline(&mut self.save_as_path);
                    ui.horizontal(|ui| {
                        if ui.button("Save").clicked() {
                            if !self.save_as_path.trim().is_empty() {
                                save_path = self.save_as_path.trim().to_string();
                                should_save = true;
                            }
                        }
                        if ui.button("Cancel").clicked() {
                            should_cancel = true;
                        }
                    });
                });

            if should_save {
                self.save_to_path(std::path::PathBuf::from(save_path));
                self.show_save_as_input = false;
                self.save_as_path.clear();
            } else if should_cancel {
                self.show_save_as_input = false;
                self.save_as_path.clear();
            }
        }

        // Status bar
        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.status);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let focused = if self.focused_pane == FocusedPane::Left {
                        "Left"
                    } else {
                        "Right"
                    };
                    ui.label(format!("Focused: {}", focused));
                });
            });
        });

        // Left / right panes
        if self.show_split_view {
            egui::SidePanel::left("left")
                .resizable(true)
                .default_width(420.0)
                .show(ctx, |ui| {
                    if pane_widget(ui, &mut self.left) {
                        self.focused_pane = FocusedPane::Left;
                    }
                });
            egui::CentralPanel::default().show(ctx, |ui| {
                if pane_widget(ui, &mut self.right) {
                    self.focused_pane = FocusedPane::Right;
                }
            });
        } else {
            egui::CentralPanel::default().show(ctx, |ui| match self.focused_pane {
                FocusedPane::Left => {
                    if pane_widget(ui, &mut self.left) {
                        self.focused_pane = FocusedPane::Left;
                    }
                }
                FocusedPane::Right => {
                    if pane_widget(ui, &mut self.right) {
                        self.focused_pane = FocusedPane::Right;
                    }
                }
            });
        }

        if self.show_command_palette {
            self.command_palette_ui(ctx);
        }
    }
}

fn format_shortcut(shortcut: &KeyboardShortcut) -> String {
    let mut parts: Vec<String> = Vec::new();
    if shortcut.modifiers.ctrl {
        parts.push("Ctrl".into());
    }
    if shortcut.modifiers.shift {
        parts.push("Shift".into());
    }
    if shortcut.modifiers.alt {
        parts.push("Alt".into());
    }
    if shortcut.modifiers.mac_cmd {
        parts.push("Cmd".into());
    }
    parts.push(format!("{:?}", shortcut.logical_key));
    parts.join("+")
}

fn pane_widget(ui: &mut egui::Ui, pane: &mut Pane) -> bool {
    let title = if pane.dirty {
        format!("{} •", pane.title)
    } else {
        pane.title.clone()
    };
    ui.heading(title);
    ui.add_space(6.0);
    let mut had_focus = false;
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let edit = egui::TextEdit::multiline(&mut pane.text)
                .code_editor()
                .desired_rows(30)
                .lock_focus(false)
                .desired_width(f32::INFINITY);
            let resp = ui.add(edit);
            if resp.changed() {
                pane.dirty = true;
            }
            had_focus = resp.has_focus();
        });
    had_focus
}

impl App {
    fn registered_actions() -> Vec<Action> {
        let ctrl = Modifiers {
            ctrl: true,
            ..Default::default()
        };
        let ctrl_shift = Modifiers {
            ctrl: true,
            shift: true,
            ..Default::default()
        };
        let ctrl_alt = Modifiers {
            ctrl: true,
            alt: true,
            ..Default::default()
        };
        vec![
            Action::new(
                "open_file",
                "Open File (Focused Pane)",
                Some(KeyboardShortcut::new(ctrl, egui::Key::O)),
                AppAction::OpenFile,
            ),
            Action::new(
                "save_file",
                "Save",
                Some(KeyboardShortcut::new(ctrl, egui::Key::S)),
                AppAction::SaveFocused,
            ),
            Action::new(
                "save_file_as",
                "Save As",
                Some(KeyboardShortcut::new(ctrl_shift, egui::Key::S)),
                AppAction::SaveAsFocused,
            ),
            Action::new(
                "quick_save",
                "Quick Save",
                Some(KeyboardShortcut::new(ctrl_alt, egui::Key::S)),
                AppAction::QuickSaveFocused,
            ),
            Action::new(
                "close_file",
                "Close File (Focused Pane)",
                Some(KeyboardShortcut::new(ctrl, egui::Key::W)),
                AppAction::CloseFocused,
            ),
            Action::new(
                "layout_split",
                "Show Split View",
                Some(KeyboardShortcut::new(ctrl, egui::Key::Num3)),
                AppAction::ShowSplitView,
            ),
            Action::new(
                "layout_left",
                "Show Left Only",
                Some(KeyboardShortcut::new(ctrl, egui::Key::Num1)),
                AppAction::ShowLeftOnly,
            ),
            Action::new(
                "layout_right",
                "Show Right Only",
                Some(KeyboardShortcut::new(ctrl, egui::Key::Num2)),
                AppAction::ShowRightOnly,
            ),
        ]
    }

    fn command_palette_shortcut() -> KeyboardShortcut {
        KeyboardShortcut::new(
            Modifiers {
                ctrl: true,
                shift: true,
                ..Default::default()
            },
            egui::Key::P,
        )
    }

    fn process_shortcuts(&mut self, ctx: &egui::Context) {
        if ctx.input_mut(|i| i.consume_shortcut(&Self::command_palette_shortcut())) {
            if self.show_command_palette {
                self.close_command_palette();
                self.status = "Command palette closed".into();
            } else {
                self.open_command_palette();
            }
        }

        if self.show_command_palette {
            return;
        }

        let actions: Vec<Action> = self.actions.iter().copied().collect();
        for action in actions {
            if let Some(shortcut) = action.shortcut {
                if ctx.input_mut(|i| i.consume_shortcut(&shortcut)) {
                    self.perform_action(action.action);
                }
            }
        }
    }

    fn open_command_palette(&mut self) {
        self.show_command_palette = true;
        self.command_palette_query.clear();
        self.command_palette_selected = 0;
        self.status = "Command palette opened (Ctrl+Shift+P or Esc to close)".into();
    }

    fn close_command_palette(&mut self) {
        self.show_command_palette = false;
        self.command_palette_query.clear();
        self.command_palette_selected = 0;
    }

    fn perform_action(&mut self, action: AppAction) {
        match action {
            AppAction::OpenFile => {
                self.open_dialog(self.focused_pane == FocusedPane::Left);
            }
            AppAction::SaveFocused => {
                self.save_focused(false);
            }
            AppAction::SaveAsFocused => {
                self.save_focused(true);
            }
            AppAction::QuickSaveFocused => {
                self.quick_save_focused();
            }
            AppAction::CloseFocused => {
                self.close_focused();
            }
            AppAction::ShowSplitView => {
                if !self.show_split_view {
                    self.show_split_view = true;
                    self.status = "Split view enabled".into();
                } else {
                    self.status = "Split view already active".into();
                }
            }
            AppAction::ShowLeftOnly => {
                self.show_split_view = false;
                self.focused_pane = FocusedPane::Left;
                self.status = "Single pane mode (showing left)".into();
            }
            AppAction::ShowRightOnly => {
                self.show_split_view = false;
                self.focused_pane = FocusedPane::Right;
                self.status = "Single pane mode (showing right)".into();
            }
        }
    }

    fn command_palette_ui(&mut self, ctx: &egui::Context) {
        use egui::Align2;

        let actions: Vec<Action> = {
            let query = self.command_palette_query.to_lowercase();
            self.actions
                .iter()
                .copied()
                .filter(|action| {
                    query.is_empty()
                        || action.label.to_lowercase().contains(query.as_str())
                        || action.id.contains(query.as_str())
                })
                .collect()
        };

        egui::Window::new("Command Palette")
            .pivot(Align2::CENTER_CENTER)
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("Type to filter commands. Enter runs the first result.");
                let text_response = ui.text_edit_singleline(&mut self.command_palette_query);
                if !text_response.has_focus() {
                    text_response.request_focus();
                }
                if text_response.changed() {
                    self.command_palette_selected = 0;
                }
                ui.separator();

                if actions.is_empty() {
                    ui.label("No matching commands.");
                } else {
                    if self.command_palette_selected >= actions.len() {
                        self.command_palette_selected = actions.len().saturating_sub(1);
                    }

                    let down = ctx.input(|i| i.key_pressed(egui::Key::ArrowDown));
                    let up = ctx.input(|i| i.key_pressed(egui::Key::ArrowUp));
                    if down {
                        self.command_palette_selected =
                            (self.command_palette_selected + 1) % actions.len();
                    } else if up {
                        if self.command_palette_selected == 0 {
                            self.command_palette_selected = actions.len() - 1;
                        } else {
                            self.command_palette_selected -= 1;
                        }
                    }

                    for (idx, action) in actions.iter().enumerate() {
                        let mut label = action.label.to_string();
                        if let Some(shortcut) = action.shortcut {
                            label.push_str(" (");
                            label.push_str(&format_shortcut(&shortcut));
                            label.push(')');
                        }
                        let resp = ui.selectable_label(idx == self.command_palette_selected, label);
                        if resp.clicked() {
                            self.command_palette_selected = idx;
                            self.perform_action(action.action);
                            self.close_command_palette();
                            return;
                        }
                    }

                    if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                        if let Some(action) = actions.get(self.command_palette_selected) {
                            self.perform_action(action.action);
                            self.close_command_palette();
                        }
                    }
                }

                if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                    self.close_command_palette();
                    self.status = "Command palette closed".into();
                }
            });
    }

    fn save_focused(&mut self, force_as: bool) {
        let (pane_name, target) = if self.focused_pane == FocusedPane::Left {
            ("left", &mut self.left)
        } else {
            ("right", &mut self.right)
        };

        if force_as || target.path.is_none() {
            // Try file dialog first, fall back to input if it fails
            self.save_as_focused();
        } else {
            // Direct save to existing path
            self.status = format!("Saving to: {}", target.path.as_ref().unwrap().display());
            match target.save() {
                Ok(_) => self.status = format!("{} pane saved", pane_name),
                Err(e) => self.status = format!("Save error: {e}"),
            }
        }
    }

    fn save_as_focused(&mut self) {
        // Try native file dialog first
        match rfd::FileDialog::new()
            .set_title("Save As")
            .add_filter("Text/Markdown", &["txt", "md", "log"])
            .save_file()
        {
            Some(p) => {
                self.save_to_path(p);
            }
            None => {
                // File dialog failed, show input dialog
                self.show_save_as_input = true;
                self.status = "File dialog not available, using input dialog".into();
            }
        }
    }

    fn save_to_path(&mut self, path: std::path::PathBuf) {
        let (pane_name, target) = if self.focused_pane == FocusedPane::Left {
            ("left", &mut self.left)
        } else {
            ("right", &mut self.right)
        };

        self.status = format!("Saving {} pane to: {}", pane_name, path.display());

        match target.save_as(path) {
            Ok(_) => self.status = format!("{} pane saved", pane_name),
            Err(e) => self.status = format!("Save error: {e}"),
        }
    }

    fn open_dialog(&mut self, to_left: bool) {
        self.status = "Opening file dialog...".into();

        match rfd::FileDialog::new()
            .set_title("Open")
            .add_filter("Text/Markdown", &["txt", "md", "log"])
            .pick_file()
        {
            Some(p) => {
                let target = if to_left {
                    &mut self.left
                } else {
                    &mut self.right
                };
                self.status = format!("Loading: {}", p.display());
                if let Err(e) = target.load_from(p) {
                    self.status = format!("open error: {e}");
                } else {
                    self.status = "opened".into();
                }
            }
            None => {
                self.status = "Open cancelled".into();
            }
        }
    }

    fn quick_save_focused(&mut self) {
        let (pane_name, target) = if self.focused_pane == FocusedPane::Left {
            ("left", &mut self.left)
        } else {
            ("right", &mut self.right)
        };

        let mut quick_save_dir = std::env::current_dir()
            .map(|dir| dir.join("target").join("quick_saves"))
            .unwrap_or_else(|_| std::env::temp_dir().join("nust_quick_saves"));
        if let Err(primary_err) = fs::create_dir_all(&quick_save_dir) {
            let fallback_dir = std::env::temp_dir().join("nust_quick_saves");
            if quick_save_dir != fallback_dir {
                match fs::create_dir_all(&fallback_dir) {
                    Ok(_) => quick_save_dir = fallback_dir,
                    Err(fallback_err) => {
                        self.status = format!(
                            "Quick save failed: {primary_err}; fallback failed: {fallback_err}"
                        );
                        return;
                    }
                }
            } else {
                self.status = format!("Quick save failed: {primary_err}");
                return;
            }
        }

        // Quick save without file dialog - save to a timestamped file
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let filename = format!("nust_{}_{}.txt", pane_name, timestamp);
        let save_path = quick_save_dir.join(filename);

        self.status = format!(
            "Quick saving {} pane to {}...",
            pane_name,
            save_path.display()
        );

        match target.save_as(save_path.clone()) {
            Ok(_) => {
                self.status = format!(
                    "{} pane quick save successful: {}",
                    pane_name,
                    save_path.display()
                )
            }
            Err(e) => self.status = format!("Quick save failed: {e}"),
        };
    }

    fn close_focused(&mut self) {
        let (pane_name, target, default_title) = if self.focused_pane == FocusedPane::Left {
            ("left", &mut self.left, "left")
        } else {
            ("right", &mut self.right, "right")
        };

        target.text.clear();
        target.path = None;
        target.dirty = false;
        target.title = default_title.into();

        self.status = format!("{pane_name} pane cleared");
    }

    fn manual_save(&mut self) {
        if self.manual_path.trim().is_empty() {
            self.status = "Please enter a filename".into();
            return;
        }

        let (pane_name, target) = if self.focused_pane == FocusedPane::Left {
            ("left", &mut self.left)
        } else {
            ("right", &mut self.right)
        };

        let save_path = std::path::PathBuf::from(self.manual_path.trim());
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = save_path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                self.status = format!("Failed to create directory: {e}").into();
                return;
            }
        }
        
        self.status = format!("Saving {} pane to {}...", pane_name, save_path.display());

        match target.save_as(save_path) {
            Ok(_) => self.status = "Manual save successful!".into(),
            Err(e) => self.status = format!("Manual save failed: {e}").into(),
        }
    }
}

fn main() -> Result<()> {
    let opts = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_resizable(true)
            .with_inner_size([1000.0, 700.0]),
        ..Default::default()
    };
    eframe::run_native("Nust", opts, Box::new(|_| Box::new(App::default())))
        .map_err(|e| anyhow::anyhow!("eframe error: {}", e))?;
    Ok(())
}
