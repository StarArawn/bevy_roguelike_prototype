use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::game::gameplay::{stats::{StatName, StatsQuery}};

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub struct Poison {
    parent: Entity,
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

    pub fn apply(child_builder: &mut ChildBuilder, parent: Entity, seconds: f32, damage: f32, total: u32) {
        child_builder.spawn()
            .insert(Poison::new(parent, seconds, damage, total));
    }

    pub fn new(parent: Entity, seconds: f32, damage: f32, total: u32) -> Self {
        Self {
            parent,
            last_update: 0.0,
            current: 0,
            seconds: seconds as f64,
            damage,
            total,
        }
    }

    pub fn tick(&mut self, time: &bevy::core::Time, mut current_attr_value: f32) -> (f32, bool) {
        if self.current >= self.total {
            return (current_attr_value, false);
        }

        let current_time = time.seconds_since_startup();

        if (current_time - self.last_update) >= self.seconds {
            // Apply damage
            current_attr_value -= self.damage;
            self.current += 1;
        }

        (current_attr_value, true)
    }

}

pub fn update(
    time: Res<Time>,
    mut commands: Commands,
    mut poison_query: Query<(Entity, &mut Poison)>,
    mut stats_query: StatsQuery,
) {
    for (entity, mut poison) in poison_query.iter_mut() {
        let health = stats_query.get_stat(poison.parent, StatName::Health);
        if health.is_some() {
            let (_, mut health) = health.unwrap();
            let (new_value, should_continue) = poison.tick(&time, health.value);
            health.value = new_value;
            if !should_continue {
                commands.entity(entity).despawn_recursive();
            }
        } else {
            commands.entity(entity).despawn_recursive();
        }
    }
}