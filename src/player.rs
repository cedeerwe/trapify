use comfy::*;

use crate::{GameState, HitPoints};

pub struct Player {
    pub hp: HitPoints,
    pub gold: u32,
}

impl Player {
    pub fn debug_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Player");
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("Reset HP").clicked() {
                self.hp.reset();
            }
            ui.label("HP: ");
            ui.add(
                egui::ProgressBar::new(self.hp.remaining_fraction())
                    .text(self.hp.as_text())
                    .fill(RED.into()),
            );
        });
        ui.horizontal(|ui| {
            if ui.button("Top up 10").clicked() {
                self.gold += 10;
            }
            ui.label(format!("Gold: {}", self.gold));
        });
    }
}

impl GameState {
    pub fn check_dead_player(&mut self) {
        if self.player.hp.is_dead() {
            self.is_game_over = true;
            self.is_paused = true;
        }
    }
}
