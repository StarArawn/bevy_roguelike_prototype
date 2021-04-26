use bevy::prelude::*;
use super::attributes::{Attribute, AttributeNames};

mod health;
pub use health::{Health, get_max_health};

pub fn update_max_stats(
    mut _commands: Commands,
    mut health_query: Query<(&mut Health, &Children)>,
    attribute_query: Query<&Attribute, Changed<Attribute>>,
) {
    for (mut health, children) in health_query.iter_mut() {
        for child in children.iter() {
            let attribute = attribute_query.get(*child);
            if attribute.is_ok() {
                let attribute = attribute.unwrap();
                if attribute.name == AttributeNames::Endurance {
                    health.max = get_max_health(attribute.value);
                    if health.value > health.max {
                        health.value = health.max;
                    }
                }
            }
        }
    }
}
