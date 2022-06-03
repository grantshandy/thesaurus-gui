use std::{sync::Arc, thread};

use eframe::{
    egui::{self, ScrollArea, Visuals},
    App, CreationContext, Frame
};

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Thesaurus",
        native_options,
        Box::new(|cc| Box::new(Thesaurus::new(cc))),
    );
}

static mut RES: Option<String> = None;
static mut LOADING: bool = false;

#[derive(Clone)]
struct Thesaurus {
    query: String,
}

impl Thesaurus {
    fn new(cc: &CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals::dark());
        Self::default()
    }

    fn search(&mut self) {
        let query = Arc::new(self.query.clone());

        thread::spawn(move || unsafe {
            LOADING = true;
            RES = None;
            if query.as_str() != "" {
                RES = Some(match thesaurus::synonym(query.as_str()) {
                    Some(res) => {
                        let mut f = String::new();

                        for word in res {
                            f.push_str(&format!(" - {word}\n"));
                        }

                        f
                    }
                    None => "None found".to_string(),
                });
            }
            LOADING = false;
        });
    }
}

impl Default for Thesaurus {
    fn default() -> Self {
        Self {
            query: String::new(),
        }
    }
}

impl App for Thesaurus {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Thesaurus");
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.query);
                if ui.button("Search").clicked() {
                    self.search();
                }
            });
            unsafe {
                if LOADING {
                    ui.label("Loading...");
                }
                if let Some(res) = &RES {
                    ui.separator();
                    ScrollArea::new([false, true]).auto_shrink([false, false]).show(ui, |ui| {
                        ui.label(res);
                    });
                }
            }
        });
    }
}
