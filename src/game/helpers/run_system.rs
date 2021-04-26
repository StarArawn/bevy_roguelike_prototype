use bevy::prelude::*;

pub fn run_system<S: System<In = (), Out = ()>>(world: &mut World, system: S) {
    let mut schedule = Schedule::default();
    let mut update_stage = SystemStage::parallel();
    update_stage.add_system(system);
    schedule.add_stage("update", update_stage);
    schedule.run(world);
}
