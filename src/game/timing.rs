use bevy::prelude::*;

#[derive(Default)]
pub struct Timing {
    pub last_time: f64,
    pub should_update: bool,
}

pub fn update(time: Res<Time>, mut timing: ResMut<Timing>) {
    let current = time.seconds_since_startup();
    if (current - timing.last_time) > 0.010 {
        timing.last_time = current;
        timing.should_update = true;
    }
}
