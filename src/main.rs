use anyhow::Result;
use eframe::{egui, NativeOptions};
use egui::{Key, Modifiers};
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
}

impl Default for App {
    fn default() -> Self {
        Self {
            left: Pane { title: "left".into(), ..Default::default() },
            right: Pane { title: "right".into(), ..Default::default() },
            status: "ready".into(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Shortcuts
        let input = ctx.input(|i| i.clone());
        let ctrl = input.modifiers.matches(Modifiers { ctrl: true, ..Default::default() });
        if ctrl && input.key_pressed(Key::O) {
            self.open_dialog(true);
        }
        if ctrl && input.key_pressed(Key::S) {
            self.save_current(false);
        }
        if ctrl && input.key_pressed(Key::ShiftS) {
            self.save_current(true);
        }

        // Top menu
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.button("Open (Ctrl+O)").clicked() { self.open_dialog(true); }
                if ui.button("Save (Ctrl+S)").clicked() { self.save_current(false); }
                if ui.button("Save As (Ctrl+Shift+S)").clicked() { self.save_current(true); }
                ui.separator();
                ui.label(&self.status);
            });
        });

        // Left / right panes
        egui::SidePanel::left("left").resizable(true).default_width(420.0).show(ctx, |ui| {
            pane_widget(ui, &mut self.left);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            pane_widget(ui, &mut self.right);
        });
    }
}

fn pane_widget(ui: &mut egui::Ui, pane: &mut Pane) {
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
}

impl App {
    fn save_current(&mut self, force_as: bool) {
        // Simple rule: save the pane that currently has focus, else left
        let left_focused = egui::Context::default(); // placeholder; we’ll pick left by default
        let target = &mut self.left; // keep it simple; you can add focus tracking later
        let mut picker = || {
            rfd::FileDialog::new()
                .set_title("Save As")
                .add_filter("Text/Markdown", &["txt", "md", "log"])
                .save_file()
        };

        let result = if force_as || target.path.is_none() {
            if let Some(p) = picker() {
                target.save_as(p)
            } else {
                return;
            }
        } else {
            target.save()
        };
        self.status = match result {
            Ok(_) => "saved".into(),
            Err(e) => format!("save error: {e}"),
        };
    }

    fn open_dialog(&mut self, to_left: bool) {
        if let Some(p) = rfd::FileDialog::new()
            .set_title("Open")
            .add_filter("Text/Markdown", &["txt", "md", "log"])
            .pick_file()
        {
            let target = if to_left { &mut self.left } else { &mut self.right };
            if let Err(e) = target.load_from(p) {
                self.status = format!("open error: {e}");
            } else {
                self.status = "opened".into();
            }
        }
    }
}

fn main() -> Result<()> {
    let opts = NativeOptions::default();
    eframe::run_native("Eliza Notes (egui)", opts, Box::new(|_| Box::new(App::default())))?;
    Ok(())
}
