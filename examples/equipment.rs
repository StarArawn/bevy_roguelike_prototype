use std::fs::File;
use std::io::Write;
use bevy::prelude::*;
use rogue_like_prototype::game::{gameplay::{attributes::{Attribute, AttributeNames, build_basic_character_attributes}, character::Character, equipment::EquipmentBase, modifiers::{ModifierBase, poison::Poison}, stats::{Stat, StatName}}};
use ron::ser::PrettyConfig;

fn startup(
    mut commands: Commands
) {
    let character = commands.spawn()
        .insert(Character::default())
        .id();
    build_basic_character_attributes(&mut commands, character);


    let sword = EquipmentBase {
        name: String::from("sword"),
        attributes: vec![
            Attribute {
                name: AttributeNames::Endurance,
                value: 5.0,
                base_value: 5.0,
                ..Default::default()
            },
            Attribute {
                name: AttributeNames::Strength,
                value: 2.0,
                base_value: 2.0,
                ..Default::default()
            },
        ],
        stats: vec![
            Stat::new(StatName::Damage, 10.0),
        ],
        modifier: vec![
            ModifierBase::Poison(Poison::new(Entity::new(0), 5.0, 2.0, 5)),
        ],
    };

    let ron_file_data = ron::ser::to_string_pretty(&sword, PrettyConfig::default()).unwrap();

    let mut output = File::create("./test.ron").unwrap();
    write!(output, "{}", ron_file_data).unwrap();
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            width: 1270.0,
            height: 720.0,
            title: String::from("equipment"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup.system())
        .run();
}
