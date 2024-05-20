use comfy::*;

pub enum Trap {
    Simple {
        cooldown: Timer,
        damage: f32,
    },
    DamageOverTime {
        cooldown: Timer,
        duration_secs: f32,
        damage_per_second: f32,
    },
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TrapBuilder {
    Simple {
        cooldown_secs: f32,
        damage: f32,
        gold_cost: u32,
    },
    DamageOverTime {
        cooldown_secs: f32,
        duration_secs: f32,
        damage_per_second: f32,
        gold_cost: u32,
    },
}

impl TrapBuilder {
    pub fn default_simple() -> Self {
        Self::Simple {
            cooldown_secs: 1.,
            damage: 2.,
            gold_cost: 10,
        }
    }

    pub fn default_damage_over_time() -> Self {
        Self::DamageOverTime {
            cooldown_secs: 1.,
            duration_secs: 2.,
            damage_per_second: 2.,
            gold_cost: 20,
        }
    }
}

impl Default for TrapBuilder {
    fn default() -> Self {
        Self::default_simple()
    }
}

impl TrapBuilder {
    pub fn name(&self) -> &'static str {
        match self {
            TrapBuilder::Simple { .. } => "Simple",
            TrapBuilder::DamageOverTime { .. } => "DOT",
        }
    }

    pub fn into_trap(&self) -> Trap {
        match self {
            TrapBuilder::Simple {
                cooldown_secs,
                damage,
                gold_cost: _,
            } => Trap::Simple {
                cooldown: Timer::from_seconds(*cooldown_secs, true),
                damage: *damage,
            },
            TrapBuilder::DamageOverTime {
                cooldown_secs,
                duration_secs,
                damage_per_second,
                gold_cost: _,
            } => Trap::DamageOverTime {
                cooldown: Timer::from_seconds(*cooldown_secs, true),
                duration_secs: *duration_secs,
                damage_per_second: *damage_per_second,
            },
        }
    }

    pub fn as_ui(&mut self, ui: &mut egui::Ui, player_gold: &mut u32) -> Option<Trap> {
        egui::ComboBox::from_label("Choose a trap")
            .selected_text(self.name())
            .show_ui(ui, |ui| {
                let default_simple = match self {
                    TrapBuilder::Simple { .. } => *self,
                    _ => Self::default_simple(),
                };
                let default_dot = match self {
                    TrapBuilder::DamageOverTime { .. } => *self,
                    _ => Self::default_damage_over_time(),
                };
                ui.selectable_value(self, default_simple, default_simple.name());
                ui.selectable_value(self, default_dot, default_dot.name());
            });
        match self {
            TrapBuilder::Simple {
                cooldown_secs,
                damage,
                gold_cost,
            } => {
                ui.label(&format!("Damage: {}", damage));
                ui.label(&format!("Cooldown (s): {}", cooldown_secs));
                ui.label(&format!("Gold Cost: {}", gold_cost));
            }
            TrapBuilder::DamageOverTime {
                cooldown_secs,
                duration_secs,
                damage_per_second,
                gold_cost,
            } => {
                ui.label(&format!("Damage per second: {}", damage_per_second));
                ui.label(&format!("Cooldown (s): {}", cooldown_secs));
                ui.label(&format!("Duration (s): {}", duration_secs));
                ui.label(&format!("Gold Cost: {}", gold_cost));
            }
        }
        if ui.button("Build").clicked() {
            let gold_cost = match self {
                TrapBuilder::Simple { gold_cost, .. }
                | TrapBuilder::DamageOverTime { gold_cost, .. } => *gold_cost,
            };
            if *player_gold >= gold_cost {
                *player_gold -= gold_cost;
                return Some(self.into_trap());
            } else {
                // TODO: Better reporting
                println!("Not enough gold to build!");
                return None;
            }
        }
        None
    }
}

pub enum TrapTile {
    Built(Trap),
    ToBeBuild(TrapBuilder),
}

impl Default for TrapTile {
    fn default() -> Self {
        Self::ToBeBuild(TrapBuilder::default())
    }
}
