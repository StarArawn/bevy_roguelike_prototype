use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Attribute {
    pub parent: Entity,
    pub name: AttributeNames,
    pub value: f32,
    pub base_value: f32,
}

pub struct TimedAttribute {
    // Time in seconds the attribute lasts for.
    pub time: f64,
    pub last_time: f64,
}

impl Attribute {
    pub fn create_base(parent: Entity, child_builder: &mut ChildBuilder, name: AttributeNames, value: f32) {
        child_builder.spawn()
            .insert(Attribute::new(parent, name, value));
    }

    pub fn create_timed(parent: Entity, child_builder: &mut ChildBuilder, name: AttributeNames, value: f32, time: f64) {
        child_builder.spawn()
            .insert(Attribute::new(parent, name, value))
            .insert(TimedAttribute {
                time,
                last_time: 0.0,
            });
    }

    pub fn new(parent: Entity, name: AttributeNames, value: f32) -> Self {
        Self {
            parent,
            name,
            value,
            base_value: value,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum AttributeNames {
    Dexterity,
    Endurance,
    Intelligence,
    Luck,
    Poise,
    Strength,
}

impl Default for AttributeNames {
    fn default() -> Self {
        Self::Endurance
    }
}

pub fn build_basic_character_attributes(commands: &mut Commands, parent: Entity) {
    commands.entity(parent).with_children(|child_builder| {
        Attribute::create_base(parent, child_builder, AttributeNames::Dexterity, 1.0);
        Attribute::create_base(parent, child_builder, AttributeNames::Endurance, 5.0);
        Attribute::create_base(parent, child_builder, AttributeNames::Intelligence, 1.0);
        Attribute::create_base(parent, child_builder, AttributeNames::Luck, 1.0);
        Attribute::create_base(parent, child_builder, AttributeNames::Poise, 1.0);
        Attribute::create_base(parent, child_builder, AttributeNames::Strength, 1.0);
    });
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use bevy::ecs::system::CommandQueue;
    use poison::Poison;
    use super::{Attribute, AttributeNames, build_basic_character_attributes};
    use crate::game::{gameplay::stats::{Health, update_max_stats}, helpers::run_system};
    use crate::game::gameplay::modifiers::*;
    use crate::game::gameplay::character::Character;

    #[test]
    fn basic_attributes() {
        let mut world = World::default();
        world.insert_resource(Time::default());

        let player = world
            .spawn()
            .insert(Character::default())
            .insert(Health::new(100.0))
            .id();

        let mut command_queue = CommandQueue::default();
        let mut commands = Commands::new(&mut command_queue, &world);

        build_basic_character_attributes(&mut commands, player);

        commands.entity(player).with_children(|child_builder| {
            Poison::apply(child_builder, player, 0.0, 10.0, 1);
        });

        command_queue.apply(&mut world);

        let mut query = world.query_filtered::<&Children, With<Character>>();
        let mut attribute_query = world.query::<&Attribute>();

        // Verify that attribute values are correctly set on the character entity as child entities.
        for children in query.iter(&world) {
            for child in children.iter() {
                let attribute = attribute_query.get(&world, *child);
                if attribute.is_ok() {
                    let attribute = attribute.unwrap();
                    if attribute.name == AttributeNames::Endurance {
                        assert!(attribute.value == 5.0);
                    } else {
                        assert!(attribute.value == 1.0);
                    }
                }
            }
        }

        // Update all of the attributes so that the max stat values are values we expect.
        run_system(&mut world, update_max_stats.system());

        // Max health should be equal to 97.5 with an endurance of 5.
        let player_health = world.entity(player).get::<Health>().unwrap();
        assert!(player_health.value == 97.5);

        // Runs the poison update system which should tick off poison damage on the player health once.
        run_system(&mut world, poison::update.system());
        run_system(&mut world, poison::update.system());

        // 10 points of poison damage applied.
        let player_health = world.entity(player).get::<Health>().unwrap();
        assert!(player_health.value == 87.5);
    }
}
