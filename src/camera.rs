use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

#[derive(Component)]
pub struct TopDownCamera {
    pub scroll_sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub min_zoom: Option<f32>,
    pub max_zoom: Option<f32>,
}

impl Default for TopDownCamera {
    fn default() -> Self {
        TopDownCamera {
            scroll_sensitivity: 1.0,
            zoom_sensitivity: 0.1,
            min_zoom: Some(0.05),
            max_zoom: Some(20.0),
        }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .add_system(mouse_control)
            .add_system(key_control);
    }
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(TopDownCamera::default());
}

fn mouse_control(
    mut scroll_evr: EventReader<MouseWheel>,
    mut query: Query<(&TopDownCamera, &mut Transform)>,
    windows: Res<Windows>,
) {
    let window = some_or_return!(windows.get_primary());
    let cursor_position = some_or_return!(window.cursor_position());
    let q = ok_or_return!(query.get_single_mut());

    let zoom_sensitivity = q.0.zoom_sensitivity;
    let min_zoom = q.0.min_zoom;
    let max_zoom = q.0.max_zoom;
    let mut transform = q.1;

    let delta_zoom = 1.0 + zoom_sensitivity;

    let rel_x = cursor_position.x / (window.physical_width() as f32);
    let rel_y = cursor_position.y / (window.physical_height() as f32);

    for ev in scroll_evr.iter() {
        match ev.unit {
            MouseScrollUnit::Line | MouseScrollUnit::Pixel => {
                if ev.y > 0.0 {
                    if let Some(max_zoom) = max_zoom {
                        if max_zoom * transform.scale.x < 1.0 {
                            continue;
                        }
                    }
                } else if let Some(min_zoom) = min_zoom {
                    if min_zoom * transform.scale.x > 1.0 {
                        continue;
                    }
                }

                if ev.y > 0.0 {
                    let zoom_x = transform.translation.z * transform.scale.x;
                    let zoom_y = transform.translation.z * transform.scale.y;
                    transform.translation.x += (rel_x - 0.5) * 0.1 * zoom_x * delta_zoom;
                    transform.translation.y += (rel_y - 0.5) * 0.1 * zoom_y * delta_zoom;

                    transform.scale.x /= delta_zoom;
                    transform.scale.y /= delta_zoom;
                } else {
                    transform.scale.x *= delta_zoom;
                    transform.scale.y *= delta_zoom;
                }
            }
        }
    }
}

fn key_control(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&TopDownCamera, &mut Transform)>,
) {
    let q = ok_or_return!(query.get_single_mut());

    let scroll_sensitivity = q.0.scroll_sensitivity;
    let mut transform = q.1;

    let scroll_delta_base = scroll_sensitivity * transform.translation.z * time.delta_seconds();

    if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
        transform.translation.y += scroll_delta_base * transform.scale.y;
    } else if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
        transform.translation.y -= scroll_delta_base * transform.scale.y;
    }

    if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        transform.translation.x -= scroll_delta_base * transform.scale.x;
    } else if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        transform.translation.x += scroll_delta_base * transform.scale.x;
    }
}

pub fn screen_to_world(camera_transform: &Transform, cursor_pos: Vec2, screen_size: Vec2) -> Vec2 {
    let left_x = camera_transform.translation.x;
    let left_y = camera_transform.translation.y;

    let scaled_screen_x =
        screen_size.x * camera_transform.scale.x * (cursor_pos.x / screen_size.x - 0.5);
    let scaled_screen_y =
        screen_size.y * camera_transform.scale.y * (cursor_pos.y / screen_size.y - 0.5);

    Vec2::new(left_x + scaled_screen_x, left_y + scaled_screen_y)
}
