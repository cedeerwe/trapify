use comfy::*;

use crate::{GameState, HitPoints};

pub struct Player {
    pub hp: HitPoints,
    pub gold: Gold,
}

pub struct Gold {
    pub value: f32,
    pub interest_cooldown: Timer,
    pub interest_size: f32,
    pub max_interest_gainable: f32,
}

impl Gold {
    pub fn interest_to_gain(&self) -> f32 {
        (self.value * self.interest_size).min(self.max_interest_gainable)
    }
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
                self.gold.value += 10.;
            }
            ui.label(format!("Gold: {}", self.gold.value));
        });
        ui.horizontal(|ui| {
            ui.label(format!("Interest: {:.2}%", self.gold.interest_size * 100.));
            ui.label(format!(
                "\t Maximum to gain: {:.2}%",
                self.gold.max_interest_gainable
            ));
        });
        ui.horizontal(|ui| {
            ui.label(format!(
                "Next interest size: {:.2}",
                self.gold.interest_to_gain()
            ));
            ui.add(
                egui::ProgressBar::new(self.gold.interest_cooldown.percent_left()).text(format!(
                    "{:.2}s / {:.2}s",
                    (self.gold.interest_cooldown.duration()
                        - self.gold.interest_cooldown.elapsed())
                    .as_secs_f32(),
                    self.gold.interest_cooldown.duration().as_secs_f32()
                )),
            );
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

    pub fn apply_gold_interest(&mut self) {
        if self.is_paused {
            return;
        }
        self.player.gold.interest_cooldown.tick_secs(self.delta);
        if self.player.gold.interest_cooldown.just_finished() {
            self.player.gold.value += self.player.gold.interest_to_gain()
        }
    }
}
