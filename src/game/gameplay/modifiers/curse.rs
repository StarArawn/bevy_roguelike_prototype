use bevy::prelude::*;
use crate::game::gameplay::attributes::{AttributeNames};

pub struct Curse {
    pub parent: Entity,
    // Time in seconds until the curse is lifted.
    pub seconds: f64,
    pub start_time: f64,
    // pub affected_attributes: Attributes,
}

// pub fn update(
//     time: Res<Time>,
//     mut commands: Commands,
//     mut curse_query: Query<(Entity, &mut Curse)>,
//     mut attributes_query: Query<&mut Attributes>,
// ) {
//     for (entity, mut curse) in curse_query.iter_mut() {
//         let parent_attributes = attributes_query.get_mut(curse.parent);
//         if parent_attributes.is_ok() {
//             let mut parent_attributes = parent_attributes.unwrap();
//             let health_attribute = parent_attributes.get_mut(AttributeNames::Health);
//             if health_attribute.is_some() {
                
//             } else {
//                 // Do nothing
//             }
//         } else {
//             commands.entity(entity).despawn_recursive();
//         }
//     }
// }