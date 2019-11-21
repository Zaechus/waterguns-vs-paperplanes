///
pub struct HitPoints {
    hp: u32,
    max_hp: u32,
}

impl HitPoints {
    /// Create a new instance of the HP type
    pub fn new(hp: u32) -> Self {
        Self { hp, max_hp: hp }
    }

    /// Return current HP
    pub fn curr_hp(&self) -> u32 {
        self.hp
    }
    /// Return the current HP as a percentage
    pub fn percent(&self) -> f64 {
        self.hp as f64 / self.max_hp as f64
    }

    /// Returns true if the current HP is 0
    pub fn is_dead(&self) -> bool {
        self.hp == 0
    }

    /// Reduce the HP by a damage value
    pub fn take_damage(&mut self, dmg: u32) {
        self.hp = if let Some(hp) = self.hp.checked_sub(dmg) {
            hp
        } else {
            0
        };
    }
}
