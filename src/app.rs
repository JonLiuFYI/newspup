mod app_state;
mod page_menu;
mod page_scores;
mod page_start;
mod page_timer;

use app_state::{NewspupPage, Round};
use egui::{
    CentralPanel, FontData, FontDefinitions, FontFamily, FontId, Layout, TextStyle, TopBottomPanel,
};

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
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "inter".to_owned(),
            FontData::from_static(include_bytes!("../assets/fonts/Inter-Regular.ttf")),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "inter".to_owned());
        cc.egui_ctx.set_fonts(fonts);

        let mut style = (*cc.egui_ctx.style()).clone();
        style.text_styles = [
            (
                TextStyle::Heading,
                FontId::new(25.0, FontFamily::Proportional),
            ),
            (
                TextStyle::Name("Heading2".into()),
                FontId::new(20.0, FontFamily::Proportional),
            ),
            (
                TextStyle::Name("Context".into()),
                FontId::new(20.0, FontFamily::Proportional),
            ),
            (TextStyle::Body, FontId::new(16.0, FontFamily::Proportional)),
            (
                TextStyle::Monospace,
                FontId::new(14.0, FontFamily::Proportional),
            ),
            (
                TextStyle::Button,
                FontId::new(16.0, FontFamily::Proportional),
            ),
            (
                TextStyle::Small,
                FontId::new(10.0, FontFamily::Proportional),
            ),
        ]
        .into();
        cc.egui_ctx.set_style(style);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    /// Choose which page to show based on `NewspupPage`
    fn page_router(&mut self, ui: &mut egui::Ui) {
        match self.page {
            NewspupPage::Start => self.page_start(ui),
            NewspupPage::Scores(round) => self.page_scores(round, ui),
            NewspupPage::Timer => self.page_timer(ui),
            NewspupPage::Menu => self.page_menu(ui),
        }
    }
}

impl eframe::App for NewspupApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // header bar
        if self.page != NewspupPage::Start {
            TopBottomPanel::top("header").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.page, NewspupPage::Scores(Round::Fri), "Fri.");
                    ui.selectable_value(&mut self.page, NewspupPage::Scores(Round::Sat), "Sat.");
                    ui.selectable_value(&mut self.page, NewspupPage::Scores(Round::Sun), "Sun.");

                    if cfg!(debug_assertions) {
                        ui.label(
                            egui::RichText::new("⚠dbg")
                                .small()
                                .color(ui.visuals().warn_fg_color),
                        );
                    }

                    ui.with_layout(Layout::right_to_left(egui::Align::Max), |ui| {
                        ui.selectable_value(&mut self.page, NewspupPage::Menu, "☰");
                        ui.selectable_value(&mut self.page, NewspupPage::Timer, "⏰ Timer");
                    });
                });
            });
        }

        // main panel
        CentralPanel::default().show(ctx, |ui| {
            self.page_router(ui);
        });
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
