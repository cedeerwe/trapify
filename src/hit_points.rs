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

    pub fn take_damage(&mut self, damage: f32) {
        self.current -= damage
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.
    }

    pub fn reset(&mut self) {
        self.current = self.maximum;
    }

    pub fn as_text(&self) -> String {
        format!("{:.2} / {:.2}", self.current, self.maximum)
    }
}
