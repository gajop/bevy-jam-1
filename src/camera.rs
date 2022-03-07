use bevy::prelude::*;

use crate::{
    players::{OwnedBy, Player},
    top_down_camera::{TopDownCamera, TopDownCameraPlugin},
};

struct ZoomedIn(bool);

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .insert_resource(ZoomedIn(false))
            .add_system(zoom_camera_to_player)
            .add_plugin(TopDownCameraPlugin);
    }
}

fn setup_camera(mut commands: Commands) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.transform.translation.z = 500.0;
    camera_bundle.transform.scale.x *= 0.5;
    camera_bundle.transform.scale.y *= 0.5;

    commands.spawn_bundle(camera_bundle).insert(TopDownCamera {
        scroll_sensitivity: 2.0,
        ..TopDownCamera::default()
    });
}

fn zoom_camera_to_player(
    q_player_star_added: Query<(&Transform, &OwnedBy), (Added<OwnedBy>, Without<TopDownCamera>)>,
    q_player: Query<&Player>,
    mut q_camera: Query<&mut Transform, With<TopDownCamera>>,
    mut zoomed_in: ResMut<ZoomedIn>,
) {
    if zoomed_in.0 {
        return;
    }
    for (player_transform, owned_by) in q_player_star_added.iter() {
        let player = ok_or_continue!(q_player.get(owned_by.player));
        if !player.is_human {
            continue;
        }

        zoomed_in.0 = true;

        let mut camera_transform = q_camera.get_single_mut().unwrap();
        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y = player_transform.translation.y;
    }
}
