use bevy::prelude::*;
use pathfinding::num_traits::Pow;
use crate::game::gameplay::attributes::{Attribute, AttributeNames};

#[derive(Debug, Clone, Copy)]
pub struct Health {
    pub value: f32,
    pub max: f32,
}

impl Health {
    pub fn new(value: f32) -> Self {
        Self {
            value,
            max: 0.0,
        }
    }
}

pub fn round(value: f32) -> f32 {
    (value * 10.0).round() / 10.0
}

pub fn get_max_health(endurance: f32) -> f32 {
    // 1.09f32.powf(1.1f32.powf(endurance / 100.0) * 100.0) / (101.0 - endurance)

    round((-((endurance - 100.0) / 10.0).powf(2.0) + 100.0) * 10.0)
}

#[cfg(test)]
mod tests {
    use super::get_max_health;

    #[test]
    fn max_health() {
        let max_health = get_max_health(5.0);
        dbg!(max_health);

        let max_health = get_max_health(10.0);
        dbg!(max_health);

        let max_health = get_max_health(20.0);
        dbg!(max_health);

        let max_health = get_max_health(40.0);
        dbg!(max_health);

        let max_health = get_max_health(50.0);
        dbg!(max_health);

        let max_health = get_max_health(80.0);
        dbg!(max_health);


        let max_health = get_max_health(99.0);
        dbg!(max_health);

        assert!(max_health == 999.9)
    }
}
