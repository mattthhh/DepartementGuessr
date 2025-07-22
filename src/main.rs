use csv::Reader;
use rand::seq::SliceRandom;
use serde::Deserialize;
use eframe::egui;

#[derive(Deserialize, Clone)]
struct Department {
    #[serde(rename = "code_departement")]
    code: String,
    #[serde(rename = "nom_departement")]
    name: String,
}

struct Game {
    data: Vec<Department>,
    filtered: Vec<Department>,
    start_code: String,
    end_code: String,
    answer: String,
    current: Option<Department>,
    message: String,
}

impl Game {
    fn new(data: Vec<Department>) -> Self {
        let mut game = Self {
            data: data.clone(),
            filtered: data,
            start_code: String::new(),
            end_code: String::new(),
            answer: String::new(),
            current: None,
            message: String::new(),
        };
        game.next_question();
        game
    }

    fn set_range(&mut self) {
        if self.start_code.is_empty() || self.end_code.is_empty() {
            self.filtered = self.data.clone();
        } else {
            let codes: Vec<&String> = self.data.iter().map(|d| &d.code).collect();
            if let (Some(start_idx), Some(end_idx)) = (
                codes.iter().position(|c| *c == &self.start_code),
                codes.iter().position(|c| *c == &self.end_code),
            ) {
                let (s, e) = if start_idx <= end_idx {
                    (start_idx, end_idx)
                } else {
                    (end_idx, start_idx)
                };
                self.filtered = self.data[s..=e].to_vec();
            } else {
                self.message = "Invalid codes".to_string();
                return;
            }
        }
        self.next_question();
    }

    fn next_question(&mut self) {
        let mut rng = rand::thread_rng();
        if let Some(dep) = self.filtered.choose(&mut rng) {
            self.current = Some(dep.clone());
            self.answer.clear();
            self.message.clear();
        }
    }

    fn check_answer(&mut self) {
        if let Some(ref current) = self.current {
            if self.answer.trim().eq_ignore_ascii_case(&current.name) {
                self.message = "Correct!".into();
            } else {
                self.message = format!("Incorrect. {} = {}", current.code, current.name);
            }
        }
        self.next_question();
    }
}

impl eframe::App for Game {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Start code:");
                ui.text_edit_singleline(&mut self.start_code);
            });
            ui.horizontal(|ui| {
                ui.label("End code:");
                ui.text_edit_singleline(&mut self.end_code);
            });
            if ui.button("Set Range").clicked() {
                self.set_range();
            }

            if let Some(ref current) = self.current {
                ui.heading(&current.code);
            }

            ui.horizontal(|ui| {
                ui.label("Department name:");
                let response = ui.text_edit_singleline(&mut self.answer);
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.check_answer();
                }
            });
            if ui.button("Submit").clicked() {
                self.check_answer();
            }
            if !self.message.is_empty() {
                ui.label(&self.message);
            }
        });
    }
}

fn load_data(path: &str) -> Vec<Department> {
    let mut rdr = Reader::from_path(path).expect("Cannot open CSV");
    rdr.deserialize().map(|r| r.expect("bad row")).collect()
}

fn main() -> eframe::Result<()> {
    let data = load_data("departements.csv");
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Departement Guessr",
        options,
        Box::new(|_cc| Box::new(Game::new(data))),
    )
}
