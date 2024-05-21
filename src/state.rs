use crate::*;
use comfy::{egui::Align2, *};

pub struct GameState {
    pub delta: f32,
    pub enemies: Vec<Enemy>,
    pub enemy_spawner: EnemySpawner,
    pub player: Player,
    pub is_paused: bool,
    pub is_game_over: bool,
    pub selected_tile: Option<TileMapPos>,
    pub trap_tiles: HashMap<TileMapPos, TrapTile>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            // Trick mentioned here: https://comfyengine.org/book/context/
            delta: 0., // Should be set each iteration
            enemy_spawner: EnemySpawner::default(),
            enemies: vec![],
            player: Player {
                hp: HitPoints::new_full(100.),
                gold: 100,
            },
            is_paused: false,
            is_game_over: false,
            selected_tile: None,
            trap_tiles: HashMap::default(),
        }
    }
}

impl GameState {
    pub fn new(_c: &EngineState) -> Self {
        Self::default()
    }

    pub fn restart(&mut self) {
        *self = Self::default();
    }

    pub fn draw(&self) {
        clear_background(LIGHTGRAY);
        self.draw_tilemap();
        self.draw_enemies();
        self.draw_traps();
    }

    pub fn check_game_over(&mut self) {
        if self.is_game_over {
            egui::Window::new("GAME OVER")
                .anchor(Align2::CENTER_TOP, [0., 0.])
                .collapsible(false)
                .show(egui(), |ui| {
                    if ui.button("Restart game").clicked() {
                        self.restart()
                    }
                });
        }
    }

    pub fn check_inputs(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            self.selected_tile = tile_map::TileMapPos::from_absolute(mouse_world())
        }

        if is_key_pressed(KeyCode::P) {
            self.is_paused = !self.is_paused;
        }
    }

    pub fn general_debug_ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.is_paused, "Paused");
            ui.label(&format!("FPS: {}", get_fps()))
        });
    }

    pub fn ui(&mut self) {
        egui::panel::TopBottomPanel::bottom("spreadsheet")
            .exact_height(300.) // TODO: This shouldn't be absolute
            .show(egui(), |ui| {
                ui.columns(2, |columns| {
                    let left_panel = &mut columns[0];
                    self.general_debug_ui(left_panel);
                    self.player.debug_ui(left_panel);
                    self.selected_tile_debug_ui(left_panel);

                    let right_panel = &mut columns[1];
                    self.enemy_spawner.debug_ui(right_panel)
                });
            });
    }
}
