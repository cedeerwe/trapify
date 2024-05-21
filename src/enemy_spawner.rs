use comfy::*;

use crate::*;

pub struct EnemySpawner {
    timer: Timer,
    maximum_hp: f32,
    speed: f32,
    damage: f32,
    gold_for_kill: f32,
}

impl Default for EnemySpawner {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1., true),
            maximum_hp: 10.,
            speed: 1.,
            damage: 3.,
            gold_for_kill: 1.,
        }
    }
}

impl GameState {
    pub fn spawn_enemies(&mut self) {
        if self.is_paused {
            return;
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
    }
}
