use comfy::*;

use crate::*;

pub struct Enemy {
    pub hp: HitPoints,
    pub speed: f32,
    pub damage: f32,
    pub position: Vec2,
    pub size: f32,
    pub damage_over_time_effects: Vec<DamageOverTimeEffect>,
    pub gold_for_kill: u32,
    pub slow_effects: Vec<SlowEffect>,
}

pub struct DamageOverTimeEffect {
    pub timer: Timer,
    pub damage_per_second: f32,
}

pub struct SlowEffect {
    pub timer: Timer,
    pub strength: f32,
}

impl Enemy {
    pub fn move_and_deal_damage(&mut self, delta_secs: f32) -> Option<f32> {
        self.slow_effects.retain_mut(|effect| {
            effect.timer.tick_secs(delta_secs);
            !effect.timer.just_finished()
        });
        let total_slow_effect: f32 = self.slow_effects.iter().map(|effect| effect.strength).sum();
        let speed = self.speed / (1. + total_slow_effect);
        self.position.x += speed * delta_secs;
        match self.position.x >= tile_map::x_max() {
            true => Some(self.damage),
            false => None,
        }
    }

    pub fn draw(&self) {
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

impl GameState {
    pub fn cleanup_dead_enemies(&mut self) {
        if self.is_paused {
            return;
        }
        self.enemies.retain(|enemy| {
            let is_dead = enemy.hp.is_dead();
            if is_dead {
                self.player.gold += enemy.gold_for_kill
            }
            !is_dead
        })
    }

    pub fn draw_enemies(&self) {
        for enemy in self.enemies.iter() {
            enemy.draw()
        }
    }

    pub fn move_enemies_and_deal_damage_to_player(&mut self) {
        if self.is_paused {
            return;
        }
        self.enemies
            .retain_mut(|enemy| match enemy.move_and_deal_damage(self.delta) {
                None => true,
                Some(damage) => {
                    self.player.hp.take_damage(damage);
                    false
                }
            })
    }

    pub fn deal_damage_over_time_to_enemies(&mut self) {
        if self.is_paused {
            return;
        }
        self.enemies.iter_mut().for_each(|enemy| {
            enemy.damage_over_time_effects.retain_mut(|dot| {
                let initial_elapsed = dot.timer.elapsed_secs();
                dot.timer.tick_secs(self.delta);

                // makes sure it does not deal damage over the max time.
                let current_elapsed = match dot.timer.just_finished() {
                    true => dot.timer.duration().as_secs_f32(),
                    false => dot.timer.elapsed_secs(),
                };
                let dot_duration = current_elapsed - initial_elapsed;
                enemy.hp.take_damage(dot.damage_per_second * dot_duration);
                !dot.timer.just_finished()
            });
        })
    }
}
