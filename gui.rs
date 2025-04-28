use eframe::egui;
use crate::game::Game;

#[derive(Default)]
pub struct MyApp {
    guess: String,
    message: String,
    hint_message: String, 
    game: Game,
}

impl MyApp {
    fn new_game(&mut self) {
        self.game = Game::new();
        self.message.clear();
        self.hint_message.clear(); 
    }

    fn give_hint(&mut self) {
        self.game.give_hint();

        let range = self.game.higher_bound - self.game.lower_bound;

        if range <= 5 {
            self.hint_message = "Ты очень близко, осталось чуть-чуть!".to_string();
        } else {
            self.hint_message = format!(
                "Подсказка: число в промежутке: [ {} - {} ]",
                self.game.lower_bound, self.game.higher_bound
            );
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.painter().rect_filled(
                ui.max_rect(),
                0.0,
                egui::Color32::from_rgb(226, 209, 246),
            );

            ui.vertical_centered(|ui| {
                ui.heading("Угадай число в промежутке от 1 до 100!");
                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    ui.label("Введите число:");
                    ui.text_edit_singleline(&mut self.guess);
                });

                ui.add_space(20.0);

                if ui.button("Угадать!").clicked() {
                    if let Ok(num) = self.guess.trim().parse::<u32>() {
                        let result = self.game.check_guess(num);

                        if result == "Ты угадал! умничка<3" {
                            self.hint_message.clear(); 
                        }

                        self.message = result;
                    } else {
                        self.message = "Чёт не то. Введи, пожалуйста, нормальное число.".to_owned();
                    }
                }

                ui.add_space(10.0);

                if ui.button("Дай мне подсказку!").clicked() {
                    self.give_hint();
                }

                ui.add_space(10.0);

                if ui.button("Начать новую игру!").clicked() {
                    self.new_game();
                }

                ui.add_space(20.0);

                ui.colored_label(egui::Color32::from_rgb(0, 150, 0), &self.message);

                if !self.hint_message.is_empty() {
                    ui.colored_label(egui::Color32::from_rgb(0, 0, 255), &self.hint_message);
                }

                ui.separator();
                ui.label("История ходов:");

                for (i, (guess, result)) in self.game.history.iter().enumerate() {
                    let color = if result == "Ты угадала! умничка<3" {
                        egui::Color32::from_rgb(20, 174, 20)
                    } else {
                        egui::Color32::from_rgb(255, 0, 0)
                    };

                    ui.colored_label(color, format!("Ход {}: {} - {}", i + 1, guess, result));
                }
            });
        });
    }
}