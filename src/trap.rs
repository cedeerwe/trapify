use comfy::*;

use crate::*;

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
    Slow {
        cooldown: Timer,
        duration_secs: f32,
        slow_effect: f32,
        area: u32,
    },
}

impl Trap {
    pub fn draw(&self, tile_map_pos: TileMapPos) {
        let color = match self {
            Trap::Simple { .. } => BLUE,
            Trap::DamageOverTime { .. } => PURPLE,
            Trap::Slow { .. } => ORANGE,
        };
        draw_circle(tile_map_pos.into_absolute_mid(), 0.3, color, 0)
    }

    pub fn draw_activation_effect(tile_map_pos: TileMapPos, color: Color) {
        draw_rect(
            tile_map_pos.into_absolute_mid(),
            Vec2::new(tile_map::TILE_SIZE, tile_map::TILE_SIZE),
            color,
            2,
        )
    }
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
    Slow {
        cooldown_secs: f32,
        duration_secs: f32,
        slow_effect: f32,
        area: u32,
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

    pub fn default_slow() -> Self {
        Self::Slow {
            cooldown_secs: 1.,
            duration_secs: 2.,
            slow_effect: 0.2,
            area: 1,
            gold_cost: 30,
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
            TrapBuilder::Slow { .. } => "Slow",
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
            TrapBuilder::Slow {
                cooldown_secs,
                duration_secs,
                slow_effect,
                area,
                gold_cost: _,
            } => Trap::Slow {
                cooldown: Timer::from_seconds(*cooldown_secs, true),
                duration_secs: *duration_secs,
                slow_effect: *slow_effect,
                area: *area,
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
                let default_slow = match self {
                    TrapBuilder::Slow { .. } => *self,
                    _ => Self::default_slow(),
                };
                ui.selectable_value(self, default_simple, default_simple.name());
                ui.selectable_value(self, default_dot, default_dot.name());
                ui.selectable_value(self, default_slow, default_slow.name());
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
            TrapBuilder::Slow {
                cooldown_secs,
                duration_secs,
                slow_effect,
                area,
                gold_cost,
            } => {
                ui.label(&format!("Slow effect: {}", slow_effect));
                ui.label(&format!("Area: {}", area));
                ui.label(&format!("Cooldown (s): {}", cooldown_secs));
                ui.label(&format!("Duration (s): {}", duration_secs));
                ui.label(&format!("Gold Cost: {}", gold_cost));
            }
        }
        if ui.button("Build").clicked() {
            let gold_cost = match self {
                TrapBuilder::Simple { gold_cost, .. }
                | TrapBuilder::DamageOverTime { gold_cost, .. }
                | TrapBuilder::Slow { gold_cost, .. } => *gold_cost,
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

impl TrapTile {
    pub fn debug_ui(&mut self, ui: &mut egui::Ui, player_gold: &mut u32) {
        match self {
            TrapTile::Built(trap) => match trap {
                Trap::Simple { cooldown, damage } => {
                    ui.label("Simple");
                    ui.horizontal(|ui| {
                        ui.label("Damage:");
                        ui.add(
                            egui::DragValue::new(damage)
                                .speed(1.0)
                                .clamp_range(1. ..=100.),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("Frequency (s):");
                        let mut trigger_cooldown = cooldown.duration().as_secs_f32();
                        ui.add(
                            egui::DragValue::new(&mut trigger_cooldown)
                                .speed(0.1)
                                .clamp_range(0.1..=100.),
                        );
                        cooldown.set_duration(Duration::from_secs_f32(trigger_cooldown));
                    });
                }
                Trap::DamageOverTime {
                    cooldown,
                    duration_secs,
                    damage_per_second,
                } => {
                    ui.label("Damage over time");
                    ui.horizontal(|ui| {
                        ui.label("Damage per second:");
                        ui.add(
                            egui::DragValue::new(damage_per_second)
                                .speed(1.0)
                                .clamp_range(1. ..=100.),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("Duration (s):");
                        ui.add(
                            egui::DragValue::new(duration_secs)
                                .speed(1.0)
                                .clamp_range(1. ..=100.),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("Frequency (s):");
                        let mut trigger_cooldown = cooldown.duration().as_secs_f32();
                        ui.add(
                            egui::DragValue::new(&mut trigger_cooldown)
                                .speed(0.1)
                                .clamp_range(0.1..=100.),
                        );
                        cooldown.set_duration(Duration::from_secs_f32(trigger_cooldown));
                    });
                }
                Trap::Slow {
                    cooldown,
                    duration_secs,
                    slow_effect,
                    area,
                } => {
                    ui.label("Slow");
                    ui.horizontal(|ui| {
                        ui.label("Slow effect");
                        ui.add(
                            egui::DragValue::new(slow_effect)
                                .speed(1.)
                                .clamp_range(1. ..=100.),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("Area");
                        ui.add(egui::DragValue::new(area).speed(1).clamp_range(0..=3));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Duration (s):");
                        ui.add(
                            egui::DragValue::new(duration_secs)
                                .speed(1.0)
                                .clamp_range(1. ..=100.),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("Frequency (s):");
                        let mut trigger_cooldown = cooldown.duration().as_secs_f32();
                        ui.add(
                            egui::DragValue::new(&mut trigger_cooldown)
                                .speed(0.1)
                                .clamp_range(0.1..=100.),
                        );
                        cooldown.set_duration(Duration::from_secs_f32(trigger_cooldown));
                    });
                }
            },
            TrapTile::ToBeBuild(trap_builder) => {
                if let Some(trap) = trap_builder.as_ui(ui, player_gold) {
                    *self = TrapTile::Built(trap);
                }
            }
        }
    }
}

impl GameState {
    pub fn draw_traps(&self) {
        for (tile_map_pos, trap_tile) in self.trap_tiles.iter() {
            if let TrapTile::Built(trap) = trap_tile {
                trap.draw(*tile_map_pos)
            }
        }
    }

    pub fn activate_traps(&mut self) {
        if self.is_paused {
            return;
        }

        for (tile_map_pos, trap_tile) in self.trap_tiles.iter_mut() {
            if let TrapTile::Built(trap) = trap_tile {
                match trap {
                    Trap::Simple { cooldown, damage } => {
                        cooldown.tick_secs(self.delta);
                        if cooldown.just_finished() {
                            Trap::draw_activation_effect(*tile_map_pos, RED);
                            // TODO: Potentially remake this to be more efficient using a hashmap of positions to list of enemies
                            self.enemies.iter_mut().for_each(|enemy| {
                                if enemy.on_tiles().contains(tile_map_pos) {
                                    enemy.hp.take_damage(*damage)
                                }
                            });
                        }
                    }
                    Trap::DamageOverTime {
                        cooldown,
                        duration_secs,
                        damage_per_second,
                    } => {
                        cooldown.tick_secs(self.delta);
                        if cooldown.just_finished() {
                            Trap::draw_activation_effect(*tile_map_pos, YELLOW);
                            // TODO: Potentially remake this to be more efficient using a hashmap of positions to list of enemies
                            self.enemies.iter_mut().for_each(|enemy| {
                                if enemy.on_tiles().contains(tile_map_pos) {
                                    enemy.damage_over_time_effects.push(DamageOverTimeEffect {
                                        timer: Timer::from_seconds(*duration_secs, false),
                                        damage_per_second: *damage_per_second,
                                    })
                                }
                            });
                        }
                    }
                    Trap::Slow {
                        cooldown,
                        duration_secs,
                        slow_effect,
                        area,
                    } => {
                        cooldown.tick_secs(self.delta);
                        if cooldown.just_finished() {
                            let affected_tiles = tile_map_pos.area_til_distance(*area);
                            for tile in affected_tiles.iter() {
                                Trap::draw_activation_effect(*tile, BLUE);
                            }
                            self.enemies.iter_mut().for_each(|enemy| {
                                if enemy
                                    .on_tiles()
                                    .iter()
                                    .any(|tile| affected_tiles.contains(tile))
                                {
                                    enemy.slow_effects.push(SlowEffect {
                                        timer: Timer::from_seconds(*duration_secs, false),
                                        strength: *slow_effect,
                                    })
                                }
                            })
                        }
                    }
                }
            }
        }
    }

    pub fn selected_tile_debug_ui(&mut self, ui: &mut egui::Ui) {
        match self.selected_tile {
            None => {
                ui.heading("No tile selected");
            }
            Some(tile_map_pos) => {
                ui.heading(&format!("Trap on ({},{})", tile_map_pos.x, tile_map_pos.y));
                self.trap_tiles
                    .entry(tile_map_pos)
                    .or_default()
                    .debug_ui(ui, &mut self.player.gold);
            }
        }
    }
}
