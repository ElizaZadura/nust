use anyhow::Result;
use eframe::{egui, NativeOptions};
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
        self.title = p.file_name().unwrap_or_default().to_string_lossy().to_string();
        self.path = Some(p);
        self.dirty = false;
        Ok(())
    }
    fn save_as(&mut self, p: PathBuf) -> Result<()> {
        fs::write(&p, self.text.as_bytes())?;
        self.title = p.file_name().unwrap_or_default().to_string_lossy().to_string();
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
}

#[derive(PartialEq)]
enum FocusedPane {
    Left,
    Right,
}

impl Default for App {
    fn default() -> Self {
        Self {
            left: Pane { title: "left".into(), ..Default::default() },
            right: Pane { title: "right".into(), ..Default::default() },
            status: "ready".into(),
            manual_path: "output.txt".into(),
            focused_pane: FocusedPane::Left,
            save_as_path: "".into(),
            show_save_as_input: false,
        }
    }
}

impl eframe::App for App {
fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // Shortcuts
    let input = ctx.input(|i| i.clone());
    let ctrl = input.modifiers.matches_logically(egui::Modifiers { ctrl: true, ..Default::default() });
    if ctrl && input.key_pressed(egui::Key::O) {
        self.open_dialog(self.focused_pane == FocusedPane::Left);
    }
    if ctrl && input.key_pressed(egui::Key::S) {
        self.save_focused(false);
    }
    if ctrl && input.modifiers.shift && input.key_pressed(egui::Key::S) {
        self.save_focused(true);
    }

    // Top menu
    egui::TopBottomPanel::top("menu").show(ctx, |ui| {
        ui.horizontal_wrapped(|ui| {
            if ui.button("Open (Ctrl+O)").clicked() { 
                self.open_dialog(self.focused_pane == FocusedPane::Left); 
            }
            if ui.button("Save (Ctrl+S)").clicked() { 
                self.save_focused(false); 
            }
            if ui.button("Save As (Ctrl+Shift+S)").clicked() { 
                self.save_as_focused(); 
            }
            if ui.button("Quick Save").clicked() { 
                self.quick_save_focused(); 
            }
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Save to:");
                ui.text_edit_singleline(&mut self.manual_path);
                if ui.button("Save").clicked() { self.manual_save(); }
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
                let focused = if self.focused_pane == FocusedPane::Left { "Left" } else { "Right" };
                ui.label(format!("Focused: {}", focused));
            });
        });
    });

    // Left / right panes
    egui::SidePanel::left("left").resizable(true).default_width(420.0).show(ctx, |ui| {
        if pane_widget(ui, &mut self.left) {
            self.focused_pane = FocusedPane::Left;
        }
    });
    egui::CentralPanel::default().show(ctx, |ui| {
        if pane_widget(ui, &mut self.right) {
            self.focused_pane = FocusedPane::Right;
        }
    });
}
}

fn pane_widget(ui: &mut egui::Ui, pane: &mut Pane) -> bool {
    let title = if pane.dirty { format!("{} •", pane.title) } else { pane.title.clone() };
    ui.heading(title);
    ui.add_space(6.0);
    let edit = egui::TextEdit::multiline(&mut pane.text)
        .code_editor()
        .desired_rows(30)
        .lock_focus(false)
        .desired_width(f32::INFINITY);
    let resp = ui.add(edit);
    if resp.changed() {
        pane.dirty = true;
    }
    resp.has_focus()
}

impl App {
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
            },
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
                let target = if to_left { &mut self.left } else { &mut self.right };
                self.status = format!("Loading: {}", p.display());
                if let Err(e) = target.load_from(p) {
                    self.status = format!("open error: {e}");
                } else {
                    self.status = "opened".into();
                }
            },
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
        
        // Quick save without file dialog - save to a timestamped file
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let filename = format!("nust_{}_{}.txt", pane_name, timestamp);
        let save_path = std::path::PathBuf::from(filename);
        
        self.status = format!("Quick saving {} pane to {}...", pane_name, save_path.display());
        
        match target.save_as(save_path) {
            Ok(_) => self.status = format!("{} pane quick save successful!", pane_name),
            Err(e) => self.status = format!("Quick save failed: {e}"),
        };
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