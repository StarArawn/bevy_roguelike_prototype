use bevy::reflect::TypeUuid;
use serde::{Deserialize, Serialize};

use super::{attributes::Attribute, modifiers::ModifierBase, stats::Stat};

#[derive(Debug, Deserialize, Serialize, TypeUuid, Clone)]
#[uuid = "0ca8e168-cfd1-4b49-ae85-12d8e94070fd"]
pub struct EquipmentBase {
    pub name: String, // Random Equipment name idea: `{Person/Location/The}{equipment base name}{highest valued attribute} and {random modifier name}`
    pub attributes: Vec<Attribute>,
    pub stats: Vec<Stat>,
    pub modifier: Vec<ModifierBase>,
}

// Example of how equipment gets applied to a entity.
// equipment entity(sword) > 
//   attributes entities >
//     endurance + 5
//     strength + 2
//   stat entities >
//     damage = 10
