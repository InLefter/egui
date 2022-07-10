#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    name: String,
    value: String,
    age: u32,
    init_flag: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            value: "hello world".to_owned(),
            age: 42,
            init_flag: false,
        }
    }
}

fn code_view_ui(ui: &mut egui::Ui, mut code: &str) {
    let language = "rs";

    ui.add(
        egui::TextEdit::multiline(&mut code)
            .font(egui::TextStyle::Monospace) // for cursor height
            .code_editor()
            .desired_rows(1)
            .lock_focus(true),
    );
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.init_flag {
            // ctx.memory().select_multi_text_edit(true);
            self.init_flag = true;
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                // ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            // ui.add(egui::TextEdit::multiline(&mut self.name).with_rect(false));
            // ui.add(egui::TextEdit::multiline(&mut self.value).with_rect(false));
            ui.add(egui::TextEdit::singleline(&mut self.name).with_rect(false)
                .select_multi(true)
            );
            ui.add(egui::TextEdit::singleline(&mut self.name).with_rect(false)
                .select_multi(true)
            );
            ui.add(egui::TextEdit::singleline(&mut self.value).with_rect(false)
                .select_multi(true)
            );

            //         let code = &r"
    // pub struct CodeExample {
    //     name: String,
    //     age: u32,
    // }
    //
    // impl CodeExample {
    //     fn ui(&mut self, ui: &mut egui::Ui) {
    // "
    //         .trim();
    //         code_view_ui(ui, code);
            if ui.button("Click each year").clicked() {
                self.age += 1;
                // let x = self.age % 2 == 0;
                // ctx.memory().select_multi_text_edit(x);
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}
