use comfy::*;

use crate::*;

pub struct EnemySpawner {
    timer: Timer,
    maximum_hp: f32,
    speed: f32,
    damage: f32,
    gold_for_kill: f32,
    progression: EnemySpawnerProgression,
}

pub struct EnemySpawnerProgression {
    timer: Timer,
    maximum_hp_increase: f32,
    speed_increase: f32,
    damage_increase: f32,
    gold_for_kill_increase: f32,
}

impl Default for EnemySpawner {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1., true),
            maximum_hp: 10.,
            speed: 1.,
            damage: 3.,
            gold_for_kill: 1.,
            progression: EnemySpawnerProgression {
                timer: Timer::from_seconds(30., true),
                maximum_hp_increase: 0.3,
                speed_increase: 0.1,
                damage_increase: 0.2,
                gold_for_kill_increase: 0.1,
            },
        }
    }
}

impl GameState {
    pub fn spawn_enemies(&mut self) {
        if self.is_paused {
            return;
        }
        self.enemy_spawner.progression.timer.tick_secs(self.delta);
        if self.enemy_spawner.progression.timer.just_finished() {
            self.enemy_spawner.damage *= 1. + self.enemy_spawner.progression.damage_increase;
            self.enemy_spawner.gold_for_kill *=
                1. + self.enemy_spawner.progression.gold_for_kill_increase;
            self.enemy_spawner.speed *= 1. + self.enemy_spawner.progression.speed_increase;
            self.enemy_spawner.maximum_hp *=
                1. + self.enemy_spawner.progression.maximum_hp_increase;
        }

        self.enemy_spawner.timer.tick_secs(self.delta);
        if self.enemy_spawner.timer.just_finished() {
            self.enemies.push(self.enemy_spawner.spawn_single())
        }
    }
}

impl EnemySpawner {
    fn spawn_single(&self) -> Enemy {
        let size = 0.5;
        Enemy {
            hp: HitPoints::new_full(self.maximum_hp),
            speed: self.speed,
            damage: self.damage,
            position: Vec2::new(
                tile_map::x_min() - size / 2.,
                tile_map::y_into_absolute_mid(rand() % tile_map::ROWS),
            ),
            size,
            damage_over_time_effects: vec![],
            gold_for_kill: self.gold_for_kill,
            slow_effects: vec![],
        }
    }

    pub fn debug_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Enemy spawner");
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Damage:");
            ui.add(
                egui::DragValue::new(&mut self.damage)
                    .speed(1.0)
                    .clamp_range(1. ..=100.),
            )
        });
        ui.horizontal(|ui| {
            ui.label("Speed:");
            ui.add(
                egui::DragValue::new(&mut self.speed)
                    .speed(1.0)
                    .clamp_range(1. ..=100.),
            )
        });
        ui.horizontal(|ui| {
            ui.label("Maximum HP:");
            ui.add(
                egui::DragValue::new(&mut self.maximum_hp)
                    .speed(1.0)
                    .clamp_range(1. ..=100.),
            )
        });
        ui.horizontal(|ui| {
            ui.label("Gold for kill:");
            ui.add(
                egui::DragValue::new(&mut self.gold_for_kill)
                    .speed(1.0)
                    .clamp_range(1. ..=100.),
            )
        });
        ui.horizontal(|ui| {
            ui.label("Frequency (s):");
            let mut spawn_cooldown = self.timer.duration().as_secs_f32();
            ui.add(
                egui::DragValue::new(&mut spawn_cooldown)
                    .speed(0.1)
                    .clamp_range(0.1..=100.),
            );
            self.timer
                .set_duration(Duration::from_secs_f32(spawn_cooldown));
        });

        ui.separator();
        ui.heading("Enemy spawner progression");
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Damage Increase:");
            ui.add(
                egui::DragValue::new(&mut self.progression.damage_increase)
                    .speed(0.01)
                    .clamp_range(0.01..=1.),
            )
        });
        ui.horizontal(|ui| {
            ui.label("Speed Increase:");
            ui.add(
                egui::DragValue::new(&mut self.progression.speed_increase)
                    .speed(0.01)
                    .clamp_range(0.01..=1.),
            )
        });
        ui.horizontal(|ui| {
            ui.label("Maximum HP Increase:");
            ui.add(
                egui::DragValue::new(&mut self.progression.maximum_hp_increase)
                    .speed(0.01)
                    .clamp_range(0.01..=1.),
            )
        });
        ui.horizontal(|ui| {
            ui.label("Gold for Kill Increase:");
            ui.add(
                egui::DragValue::new(&mut self.progression.gold_for_kill_increase)
                    .speed(0.01)
                    .clamp_range(0.01..=1.),
            )
        });
        ui.horizontal(|ui| {
            ui.label("Next increase:");
            ui.add(
                egui::ProgressBar::new(self.progression.timer.percent_left()).text(format!(
                    "{:.2}s / {:.2}s",
                    (self.progression.timer.duration() - self.progression.timer.elapsed())
                        .as_secs_f32(),
                    self.progression.timer.duration().as_secs_f32()
                )),
            )
        });
    }
}
