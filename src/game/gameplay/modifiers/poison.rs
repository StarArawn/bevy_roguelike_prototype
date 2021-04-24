use super::Modifier;

pub struct Poison {
    last_update: f64,
    current: u32,
    // Time in seconds before the modifier applies itself to the attribute.
    pub seconds: f64,
    // Amount of poison damage to apply on each "tick".
    pub damage: f32,
    // Total number of times to apply damage 
    pub total: u32,
}

impl Poison {
    pub fn new(seconds: f32, damage: f32, total: u32) -> Self {
        Self {
            last_update: 0.0,
            current: 0,
            seconds: seconds as f64,
            damage,
            total,
        }
    }
}

impl Modifier for Poison {
    fn modify_tick(&mut self, time: &bevy::core::Time, mut current_attr_value: f32, _max: f32) -> (f32, bool) {
        if self.current >= self.total {
            return (current_attr_value, false);
        }

        let current_time = time.seconds_since_startup();

        if (current_time - self.last_update) >= self.seconds {
            // Apply damage
            current_attr_value -= self.damage;
            self.total += 1;
        }

        (current_attr_value, true)
    }

    fn modify(&mut self, _current: f32, _max: f32) -> f32 {
        todo!()
    }

    fn get_type(&self) -> super::ModifierType {
        super::ModifierType::TICK
    }
}