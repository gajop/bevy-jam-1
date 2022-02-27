use bevy::prelude::*;

use crate::top_down_camera::{TopDownCamera, TopDownCameraPlugin};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .add_plugin(TopDownCameraPlugin);
    }
}

fn setup_camera(mut commands: Commands) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    info!(camera_bundle.camera.near);
    camera_bundle.transform.translation.z = 500.0;

    commands.spawn_bundle(camera_bundle).insert(TopDownCamera {
        scroll_sensitivity: 2.0,
        ..TopDownCamera::default()
    });
}
