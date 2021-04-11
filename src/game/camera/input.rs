use bevy::{
    // input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    render::camera::Camera,
};

pub struct KeyboardConf {
    pub forward: Box<[KeyCode]>,
    pub backward: Box<[KeyCode]>,
    pub left: Box<[KeyCode]>,
    pub right: Box<[KeyCode]>,
    pub move_sensitivity: f32,
} 
 
impl Default for KeyboardConf {
    fn default() -> Self {
        KeyboardConf {
            forward: Box::new([KeyCode::W, KeyCode::Up]),
            backward: Box::new([KeyCode::S, KeyCode::Down]),
            left: Box::new([KeyCode::A, KeyCode::Left]),
            right: Box::new([KeyCode::D, KeyCode::Right]),
            move_sensitivity: 2.0,
        }
    }
}

pub fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Camera, &mut Transform)>,
) {
    for (camera, mut transform) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        let scale = transform.scale.x;

        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Z) && scale < 100. {
            let scale = ((scale + (time.delta_seconds() * 1.5)) * 100.0).round() / 100.0;
            transform.scale = Vec3::new(scale, scale, scale);
        }

        if keyboard_input.pressed(KeyCode::X) && scale > 1.1 {
            let scale = ((scale - (time.delta_seconds() * 1.5)) * 100.0).round() / 100.0;
            transform.scale = Vec3::new(scale, scale, scale);
        }

        transform.translation += time.delta_seconds() * direction * 1000.;
    }
}