mod app_state;

use app_state::{NewspupPage, Round};
use egui::{CentralPanel, DragValue, Layout, TopBottomPanel};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct NewspupApp {
    num_players: f32,
    page: NewspupPage,
}

impl Default for NewspupApp {
    fn default() -> Self {
        Self {
            num_players: 1.,
            page: NewspupPage::Start,
        }
    }
}

impl NewspupApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for NewspupApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.page != NewspupPage::Start {
            TopBottomPanel::top("header").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("1 | Fri.").clicked() {
                        self.page = NewspupPage::Scores(Round::Fri);
                    }
                    if ui.button("2 | Sat.").clicked() {
                        self.page = NewspupPage::Scores(Round::Sat);
                    }
                    if ui.button("3 | Sun.").clicked() {
                        self.page = NewspupPage::Scores(Round::Sun);
                    }

                    ui.with_layout(Layout::right_to_left(egui::Align::Max), |ui| {
                        if ui.button("New").clicked() {
                            self.page = NewspupPage::Start;
                        }
                        if ui.button("Timer").clicked() {
                            self.page = NewspupPage::Timer;
                        }
                    });
                });
            });
        }

        CentralPanel::default().show(ctx, |ui| {
            match self.page {
                NewspupPage::Start => {
                    ui.heading("Newspup");

                    ui.horizontal(|ui| {
                        ui.label("How many players?");
                        ui.add(
                            DragValue::new(&mut self.num_players)
                                .clamp_range(1.0..=6.0)
                                .speed(0.02)
                                .max_decimals(0),
                        );
                    });

                    for i in 1..=self.num_players as usize {
                        ui.label(format!("Player {i}"));
                    }

                    if ui.button("Start Game").clicked() {
                        self.page = NewspupPage::Scores(Round::Fri);
                    }
                }
                NewspupPage::Scores(round) => {
                    let round = match round {
                        Round::Fri => "Friday",
                        Round::Sat => "Saturday",
                        Round::Sun => "Sunday",
                    };
                    ui.label(format!("Round {round}"));
                    ()
                }
                NewspupPage::Timer => {
                    ui.label("Timer");
                    ()
                }
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
