use comfy::{egui::ProgressBar, *};
use trapify::*;

simple_game!("Trapify", GameState, setup, update);

pub struct GameState {
    enemies: Vec<Enemy>,
    enemy_spawner: EnemySpawner,
    player: Player,
    is_paused: bool,
    is_game_over: bool,
    selected_tile: Option<TileMapPos>,
}

impl GameState {
    pub fn new(_c: &EngineState) -> Self {
        Self {
            enemy_spawner: EnemySpawner {
                timer: Timer::from_seconds(1., true),
                maximum_hp: 10.,
                speed: 1.,
                damage: 3.,
            },
            enemies: vec![],
            player: Player {
                hp: HitPoints::new_full(100.),
            },
            is_paused: false,
            is_game_over: false,
            selected_tile: None,
        }
    }
}

pub struct EnemySpawner {
    timer: Timer,
    maximum_hp: f32,
    speed: f32,
    damage: f32,
}

impl EnemySpawner {
    pub fn spawn(&self) -> Enemy {
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
        }
    }
}

pub struct Enemy {
    hp: HitPoints,
    speed: f32,
    damage: f32,
    position: Vec2,
    size: f32,
}

impl Enemy {
    pub fn move_and_deal_damage(&mut self, delta_secs: f32) -> Option<f32> {
        self.position.x += self.speed * delta_secs;
        match self.position.x >= tile_map::x_max() {
            true => Some(self.damage),
            false => None,
        }
    }

    pub fn show(&self) {
        // resolves the case at the beginning of the map, when the whole square isn't seen
        let x_min = tile_map::x_min().max(self.position.x - 0.5 * self.size);
        let x_max = self.position.x + 0.5 * self.size;
        let x_mid = (x_min + x_max) * 0.5;
        let x_size = x_max - x_min;

        // life-bar
        let y_min = self.position.y - 0.5 * self.size;
        let y_max = y_min + self.size * self.hp.remaining_fraction();
        let y_mid = (y_min + y_max) * 0.5;
        let y_size = y_max - y_min;

        draw_rect(
            Vec2::new(x_mid, self.position.y),
            Vec2::new(x_size, self.size),
            RED,
            10,
        );
        draw_rect(
            Vec2::new(x_mid, y_mid),
            Vec2::new(x_size, y_size),
            DARKGREEN,
            11,
        );
    }
}

pub struct HitPoints {
    maximum: f32,
    current: f32,
}

impl HitPoints {
    pub fn new_full(maximum: f32) -> Self {
        Self {
            maximum,
            current: maximum,
        }
    }

    pub fn remaining_fraction(&self) -> f32 {
        self.current / self.maximum
    }

    pub fn take_damage_and_die(&mut self, damage: f32) -> bool {
        self.current -= damage;
        self.current <= 0.
    }

    pub fn reset(&mut self) {
        self.current = self.maximum;
    }
}

pub struct Player {
    hp: HitPoints,
}

fn setup(_state: &mut GameState, _c: &mut EngineContext) {}

fn update(state: &mut GameState, _c: &mut EngineContext) {
    let delta = delta();
    clear_background(LIGHTGRAY);

    tile_map::draw();

    if !state.is_paused {
        // Move the enemies
        state.enemies.retain_mut(|enemy| {
            if let Some(damage) = enemy.move_and_deal_damage(delta) {
                if state.player.hp.take_damage_and_die(damage) {
                    state.is_paused = true;
                    state.is_game_over = true;
                };
                return false;
            }
            true
        });

        // Spawn new enemies
        state.enemy_spawner.timer.tick_secs(delta);
        if state.enemy_spawner.timer.just_finished() {
            state.enemies.push(state.enemy_spawner.spawn())
        }
    }

    // Show enemies
    for enemy in state.enemies.iter() {
        enemy.show()
    }

    if state.is_game_over {
        egui::Window::new("GAME OVER").show(egui(), |ui| ui.heading("GAME OVER!"));
    }

    if is_mouse_button_pressed(MouseButton::Left) {
        state.selected_tile = tile_map::TileMapPos::from_absolute(mouse_world())
    }

    if let Some(tile_map_pos) = state.selected_tile.as_ref() {
        draw_rect(
            tile_map_pos.into_absolute_mid(),
            Vec2::new(tile_map::TILE_SIZE, tile_map::TILE_SIZE),
            PINK,
            0,
        );
    }

    egui::panel::TopBottomPanel::bottom("spreadsheet")
        .exact_height(300.)
        .show(egui(), |ui| {
            ui.columns(2, |columns| {
                let left_panel = &mut columns[0];

                left_panel.heading("Player");
                left_panel.separator();
                left_panel.horizontal(|ui| {
                    ui.label("Player HP: ");
                    ui.add(
                        ProgressBar::new(state.player.hp.remaining_fraction())
                            .text(format!(
                                "{} / {}",
                                state.player.hp.current, state.player.hp.maximum
                            ))
                            .fill(RED.into()),
                    )
                });
                left_panel.horizontal(|ui| {
                    let pause_text = match state.is_paused {
                        true => "Unpause",
                        false => "Pause",
                    };
                    if ui.button(pause_text).clicked() && !state.is_game_over {
                        state.is_paused = !state.is_paused
                    };
                    if ui.button("Reset HP").clicked() {
                        state.player.hp.reset();
                    }
                });

                let right_panel = &mut columns[1];
                right_panel.heading("Enemy spawner");
                right_panel.separator();
                right_panel.horizontal(|ui| {
                    ui.label("Damage:");
                    ui.add(
                        egui::DragValue::new(&mut state.enemy_spawner.damage)
                            .speed(1.0)
                            .clamp_range(1. ..=100.),
                    )
                });
                right_panel.horizontal(|ui| {
                    ui.label("Speed:");
                    ui.add(
                        egui::DragValue::new(&mut state.enemy_spawner.speed)
                            .speed(1.0)
                            .clamp_range(1. ..=100.),
                    )
                });
                right_panel.horizontal(|ui| {
                    ui.label("Maximum HP:");
                    ui.add(
                        egui::DragValue::new(&mut state.enemy_spawner.maximum_hp)
                            .speed(1.0)
                            .clamp_range(1. ..=100.),
                    )
                });
                right_panel.horizontal(|ui| {
                    ui.label("Frequency (s):");
                    let mut spawn_cooldown = state.enemy_spawner.timer.duration().as_secs_f32();
                    ui.add(
                        egui::DragValue::new(&mut spawn_cooldown)
                            .speed(0.1)
                            .clamp_range(0.1..=100.),
                    );
                    state
                        .enemy_spawner
                        .timer
                        .set_duration(Duration::from_secs_f32(spawn_cooldown));
                });
            });
        });
}
