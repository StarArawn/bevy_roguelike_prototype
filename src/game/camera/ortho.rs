use bevy::{prelude::*, render::{camera::{Camera, CameraProjection, DepthCalculation, ScalingMode, VisibleEntities, WindowOrigin}, render_graph::base}};
#[derive(Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct CustomOrthographicProjection {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,
    pub window_origin: WindowOrigin,
    pub scaling_mode: ScalingMode,
    pub scale: f32,
    pub depth_calculation: DepthCalculation,
}

impl CameraProjection for CustomOrthographicProjection {
    fn get_projection_matrix(&self) -> Mat4 {
        Mat4::orthographic_rh(
            self.left * self.scale,
            self.right * self.scale,
            self.bottom * self.scale,
            self.top * self.scale,
            self.near,
            self.far,
        )
    }

    fn update(&mut self, width: f32, height: f32) {
        match (&self.scaling_mode, &self.window_origin) {
            (ScalingMode::WindowSize, WindowOrigin::Center) => {
                let half_width = width / 4.0;
                let half_height = height / 4.0;
                self.left = -half_width;
                self.right = half_width;
                self.top = half_height;
                self.bottom = -half_height;
            }
            (ScalingMode::WindowSize, WindowOrigin::BottomLeft) => {
                self.left = 0.0;
                self.right = width;
                self.top = height;
                self.bottom = 0.0;
            }
            (ScalingMode::FixedVertical, WindowOrigin::Center) => {
                let aspect_ratio = width / height;
                self.left = -aspect_ratio;
                self.right = aspect_ratio;
                self.top = 1.0;
                self.bottom = -1.0;
            }
            (ScalingMode::FixedVertical, WindowOrigin::BottomLeft) => {
                let aspect_ratio = width / height;
                self.left = 0.0;
                self.right = aspect_ratio;
                self.top = 1.0;
                self.bottom = 0.0;
            }
            (ScalingMode::FixedHorizontal, WindowOrigin::Center) => {
                let aspect_ratio = height / width;
                self.left = -1.0;
                self.right = 1.0;
                self.top = aspect_ratio;
                self.bottom = -aspect_ratio;
            }
            (ScalingMode::FixedHorizontal, WindowOrigin::BottomLeft) => {
                let aspect_ratio = height / width;
                self.left = 0.0;
                self.right = 1.0;
                self.top = aspect_ratio;
                self.bottom = 0.0;
            }
            (ScalingMode::None, _) => {}
        }
    }

    fn depth_calculation(&self) -> DepthCalculation {
        self.depth_calculation
    }
}

impl Default for CustomOrthographicProjection {
    fn default() -> Self {
        CustomOrthographicProjection {
            left: -1.0,
            right: 1.0,
            bottom: -1.0,
            top: 1.0,
            near: 0.0,
            far: 1000.0,
            window_origin: WindowOrigin::Center,
            scaling_mode: ScalingMode::WindowSize,
            scale: 1.0,
            depth_calculation: DepthCalculation::Distance,
        }
    }
}

/// Component bundle for camera entities with orthographic projection
///
/// Use this for 2D games, isometric games, CAD-like 3D views.
#[derive(Bundle)]
pub struct CustomOrthographicCameraBundle {
    pub camera: Camera,
    pub orthographic_projection: CustomOrthographicProjection,
    pub visible_entities: VisibleEntities,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl CustomOrthographicCameraBundle {
    pub fn new_2d() -> Self {
        // we want 0 to be "closest" and +far to be "farthest" in 2d, so we offset
        // the camera's translation by far and use a right handed coordinate system
        let far = 1000.0;
        Self {
            camera: Camera {
                name: Some(base::camera::CAMERA_2D.to_string()),
                ..Default::default()
            },
            orthographic_projection: CustomOrthographicProjection {
                far,
                depth_calculation: DepthCalculation::ZDifference,
                ..Default::default()
            },
            visible_entities: Default::default(),
            transform: Transform::from_xyz(0.0, 0.0, far - 0.1),
            global_transform: Default::default(),
        }
    }
}