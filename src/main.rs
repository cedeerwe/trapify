use comfy::{egui::ProgressBar, *};
use trapify::*;

simple_game!("Trapify", GameState, config, setup, update);

fn config(config: GameConfig) -> GameConfig {
    GameConfig {
        // This overrides the default ResolutionConfig::Logical(1106, 526) for WASM targets
        resolution: ResolutionConfig::Physical(1920, 1080),
        ..config
    }
}

pub struct GameState {
    enemies: Vec<Enemy>,
    enemy_spawner: EnemySpawner,
    player: Player,
    is_paused: bool,
    is_game_over: bool,
    selected_tile: Option<TileMapPos>,
    trap_tiles: HashMap<TileMapPos, TrapTile>,
}

pub enum TrapTile {
    Built(Trap),
    ToBeBuild(TrapLabel),
}

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

#[derive(Debug, PartialEq)]
pub enum TrapLabel {
    Simple,
    DamageOverTime,
}

impl TrapLabel {
    pub fn as_str(&self) -> &'static str {
        match self {
            TrapLabel::Simple => "Simple",
            TrapLabel::DamageOverTime => "DamageOverTime",
        }
    }
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
            trap_tiles: HashMap::default(),
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
            damage_over_time_effects: vec![],
        }
    }
}

pub struct Enemy {
    hp: HitPoints,
    speed: f32,
    damage: f32,
    position: Vec2,
    size: f32,
    damage_over_time_effects: Vec<DamageOverTimeEffect>,
}

pub struct DamageOverTimeEffect {
    timer: Timer,
    damage_per_second: f32,
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

    pub fn on_tiles(&self) -> HashSet<TileMapPos> {
        // TODO: This only works if size < TILE_SIZE
        let min_x = self.position.x - self.size / 2.;
        let max_x = self.position.x + self.size / 2.;

        // TODO: This only make sense if every enemy is in a specific row
        let y = tile_map::y_from_absolute(self.position.y);

        let mut result = HashSet::new();
        if let Some(y) = y {
            if let Some(x) = tile_map::x_from_absolute(min_x) {
                result.insert(TileMapPos { x, y });
            }
            if let Some(x) = tile_map::x_from_absolute(max_x) {
                result.insert(TileMapPos { x, y });
            }
        }
        result
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
            // First take DOT damage
            let mut died_to_dot = false;
            enemy.damage_over_time_effects.retain_mut(|dot| {
                let initial_elapsed = dot.timer.elapsed_secs();
                dot.timer.tick_secs(delta);

                // makes sure it does not deal damage over the max time.
                let current_elapsed = match dot.timer.just_finished() {
                    true => dot.timer.duration().as_secs_f32(),
                    false => dot.timer.elapsed_secs(),
                };
                let dot_duration = current_elapsed - initial_elapsed;
                if enemy
                    .hp
                    .take_damage_and_die(dot.damage_per_second * dot_duration)
                {
                    died_to_dot = true;
                }
                !dot.timer.just_finished()
            });

            if died_to_dot {
                return false;
            }

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

    // draw traps
    for (tile_map_pos, trap_tile) in state.trap_tiles.iter() {
        if let TrapTile::Built(trap) = trap_tile {
            let color = match trap {
                Trap::Simple { .. } => BLUE,
                Trap::DamageOverTime { .. } => PURPLE,
            };
            draw_circle(tile_map_pos.into_absolute_mid(), 0.3, color, 0)
        }
    }

    // deal trap damage
    if !state.is_paused {
        for (tile_map_pos, trap_tile) in state.trap_tiles.iter_mut() {
            if let TrapTile::Built(trap) = trap_tile {
                match trap {
                    Trap::Simple { cooldown, damage } => {
                        cooldown.tick_secs(delta);
                        if cooldown.just_finished() {
                            draw_rect(
                                tile_map_pos.into_absolute_mid(),
                                Vec2::new(tile_map::TILE_SIZE, tile_map::TILE_SIZE),
                                RED,
                                2,
                            );
                            // TODO: Potentially remake this to be more efficient using a hashmap of positions to list of enemies
                            state.enemies.retain_mut(|enemy| {
                                if enemy.on_tiles().contains(tile_map_pos) {
                                    return !enemy.hp.take_damage_and_die(*damage);
                                }
                                true
                            });
                        }
                    }
                    Trap::DamageOverTime {
                        cooldown,
                        duration_secs,
                        damage_per_second,
                    } => {
                        cooldown.tick_secs(delta);
                        if cooldown.just_finished() {
                            draw_rect(
                                tile_map_pos.into_absolute_mid(),
                                Vec2::new(tile_map::TILE_SIZE, tile_map::TILE_SIZE),
                                YELLOW,
                                2,
                            );
                            // TODO: Potentially remake this to be more efficient using a hashmap of positions to list of enemies
                            state.enemies.iter_mut().for_each(|enemy| {
                                if enemy.on_tiles().contains(tile_map_pos) {
                                    enemy.damage_over_time_effects.push(DamageOverTimeEffect {
                                        timer: Timer::new(
                                            Duration::from_secs_f32(*duration_secs),
                                            false,
                                        ),
                                        damage_per_second: *damage_per_second,
                                    })
                                }
                            });
                        }
                    }
                }
            }
        }
    }
    if is_key_pressed(KeyCode::P) {
        state.is_paused = !state.is_paused;
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
                    ui.checkbox(&mut state.is_paused, "Paused");
                    if ui.button("Reset HP").clicked() {
                        state.player.hp.reset();
                    }
                    ui.label(&format!("FPS: {}", get_fps()))
                });
                if let Some(tile_map_pos) = state.selected_tile {
                    left_panel.heading(&format!("Trap on ({},{})", tile_map_pos.x, tile_map_pos.y));
                    let trap_tile = state
                        .trap_tiles
                        .entry(tile_map_pos)
                        .or_insert(TrapTile::ToBeBuild(TrapLabel::Simple));

                    match trap_tile {
                        TrapTile::Built(trap) => match trap {
                            Trap::Simple { cooldown, damage } => {
                                left_panel.label("Simple");
                                left_panel.horizontal(|ui| {
                                    ui.label("Damage:");
                                    ui.add(
                                        egui::DragValue::new(damage)
                                            .speed(1.0)
                                            .clamp_range(1. ..=100.),
                                    );
                                });
                                left_panel.horizontal(|ui| {
                                    ui.label("Frequency (s):");
                                    let mut trigger_cooldown = cooldown.duration().as_secs_f32();
                                    ui.add(
                                        egui::DragValue::new(&mut trigger_cooldown)
                                            .speed(0.1)
                                            .clamp_range(0.1..=100.),
                                    );
                                    cooldown
                                        .set_duration(Duration::from_secs_f32(trigger_cooldown));
                                });
                            }
                            Trap::DamageOverTime {
                                cooldown,
                                duration_secs,
                                damage_per_second,
                            } => {
                                left_panel.label("Damage over time");
                                left_panel.horizontal(|ui| {
                                    ui.label("Damage per second:");
                                    ui.add(
                                        egui::DragValue::new(damage_per_second)
                                            .speed(1.0)
                                            .clamp_range(1. ..=100.),
                                    );
                                });
                                left_panel.horizontal(|ui| {
                                    ui.label("Duration (s):");
                                    ui.add(
                                        egui::DragValue::new(duration_secs)
                                            .speed(1.0)
                                            .clamp_range(1. ..=100.),
                                    );
                                });
                                left_panel.horizontal(|ui| {
                                    ui.label("Frequency (s):");
                                    let mut trigger_cooldown = cooldown.duration().as_secs_f32();
                                    ui.add(
                                        egui::DragValue::new(&mut trigger_cooldown)
                                            .speed(0.1)
                                            .clamp_range(0.1..=100.),
                                    );
                                    cooldown
                                        .set_duration(Duration::from_secs_f32(trigger_cooldown));
                                });
                            }
                        },
                        TrapTile::ToBeBuild(selected_trap) => {
                            egui::ComboBox::from_label("Choose a trap")
                                .selected_text(selected_trap.as_str())
                                .show_ui(left_panel, |ui| {
                                    ui.selectable_value(
                                        selected_trap,
                                        TrapLabel::Simple,
                                        TrapLabel::Simple.as_str(),
                                    );
                                    ui.selectable_value(
                                        selected_trap,
                                        TrapLabel::DamageOverTime,
                                        TrapLabel::DamageOverTime.as_str(),
                                    );
                                });
                            if left_panel.button("Build").clicked() {
                                let trap_to_be_built = TrapTile::Built(match selected_trap {
                                    TrapLabel::Simple => Trap::Simple {
                                        cooldown: Timer::from_seconds(1., true),
                                        damage: 1.,
                                    },
                                    TrapLabel::DamageOverTime => Trap::DamageOverTime {
                                        cooldown: Timer::from_seconds(1., true),
                                        duration_secs: 2.,
                                        damage_per_second: 1.,
                                    },
                                });

                                *trap_tile = trap_to_be_built;
                            }
                        }
                    }
                }

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
