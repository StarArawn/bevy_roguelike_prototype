use bevy::{ecs::system::SystemParam, prelude::*};
use serde::{Deserialize, Serialize};
use super::attributes::{Attribute, AttributeNames};

mod stat;
mod health;

pub use stat::{Stat};

#[derive(SystemParam)]
pub struct StatsQuery<'a> {
    entity_children_query: Query<'a, &'static Children, With<HasStats>>,
    stat_query: Query<'a, (Entity, &'static mut Stat)>,
}

impl<'a> StatsQuery<'a> {
    pub fn get_stat(&mut self, entity: Entity, stat_name: StatName) -> Option<(Entity, Mut<Stat>)> {
        let mut found_entity = None;
        if let Ok(children) = self.entity_children_query.get(entity) {
            for child in children.iter() {
                if let Ok((entity, stat)) = self.stat_query.get_mut(*child) {
                    if stat.name == stat_name {
                        found_entity = Some(entity);
                    }
                }
            }
        }

        if let Some(found_entity) = found_entity {
            if let Ok((entity, stat)) = self.stat_query.get_mut(found_entity) {
                return Some((entity, stat));
            }
        }

        None
    }
}


pub fn update_max_stats(
    mut _commands: Commands,
    attribute_query: Query<(&Parent, &Attribute), Changed<Attribute>>,
    mut stats_query: StatsQuery,
) {
    for (parent_entity, attribute) in attribute_query.iter() {
        if attribute.name == AttributeNames::Endurance {
            if let Some((_, mut health_stat)) = stats_query.get_stat(parent_entity.0, StatName::Health) {
                health_stat.max = health::get_max(attribute.value);
                if health_stat.value > health_stat.max {
                    health_stat.value = health_stat.max;
                }
            }
        }
    }
}

pub struct HasStats;

#[derive(Debug, Clone, Deserialize, Serialize, Copy, PartialEq, Eq, Hash)]
pub enum StatName {
    Health,
    Damage,
}
