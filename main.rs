mod game;
mod gui;


use crate::gui::MyApp;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Угадай число!",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
