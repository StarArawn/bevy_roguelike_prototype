use bevy::prelude::*;
use crate::game::gameplay::modifiers::{Modifier, ModifierType};
use super::attribute::Attribute;

pub struct Health {
    pub modifiers: Vec<Box<dyn Modifier>>,
    pub max: f32,
    pub current: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self {
            modifiers: Vec::new(),
            max,
            current: max,
        }
    }
}

impl Attribute for Health {
    fn update(&mut self, time: &Time) {
        let mut removed_modifiers = Vec::new();
        for (index, modifier) in self.modifiers.iter_mut().enumerate() {
            let modifier_type = modifier.get_type();
            match modifier_type {
                ModifierType::TICK => {
                    let (new_value, continuing) = modifier.modify_tick(time, self.current, self.max);
                    self.current = new_value;
                    if !continuing {
                        removed_modifiers.push(index);
                    }
                },
                ModifierType::ONCE => {
                    self.current = modifier.modify(self.current, self.max);
                    removed_modifiers.push(index);
                },
                _ => (),
            }
        }

        removed_modifiers.iter().for_each(|index| { self.modifiers.remove(*index); });
    }
    
    fn get_modified_value(&mut self) -> f32 {
        let mut final_value = self.current;
        for modifier in self.modifiers.iter_mut() {
            let modifier_type = modifier.get_type();
            match modifier_type {
                ModifierType::PERMA => {
                    final_value = modifier.modify(self.current, self.max);
                },
                _ => (),
            }
        }

        final_value
    }
}

pub fn update_health(
    time: Res<Time>,
    mut query: Query<&mut Health>,
) {
    for mut health_attribute in query.iter_mut() {
        health_attribute.update(&time);
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use super::{Health, Attribute, update_health};
    use crate::game::gameplay::modifiers::Poison;

    fn run_system<S: System<In = (), Out = ()>>(world: &mut World, system: S) {
        let mut schedule = Schedule::default();
        let mut update_stage = SystemStage::parallel();
        update_stage.add_system(system);
        schedule.add_stage("update", update_stage);
        schedule.run(world);
    }

    #[test]
    fn should_damage_health() {
        let mut world = World::default();
        world.insert_resource(Time::default());

        let mut health_attr = Health::new(100.0);
        health_attr.modifiers.push(Box::new(Poison::new(0.0, 10.0, 1)));

        let player = world.spawn()
            .insert(health_attr).id();

        assert!(world.entity_mut(player).get_mut::<Health>().unwrap().get_modified_value() == 100.0);

        run_system(&mut world, update_health.system());
        
        let health = world.entity_mut(player).get_mut::<Health>().unwrap().get_modified_value();
        assert!(health == 90.0);
    }
}
