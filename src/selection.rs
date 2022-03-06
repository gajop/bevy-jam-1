use bevy::{input::mouse::*, prelude::*};

use bevy_prototype_lyon::prelude::*;

use crate::top_down_camera::{screen_to_world, TopDownCamera};

#[derive(Component)]
pub struct Selectable {
    pub width: f32,
    pub height: f32,
}

#[derive(Component, Default, Debug)]
pub struct SelectionRect {
    pub first_x: Option<f32>,
    pub first_y: Option<f32>,
    pub second_x: Option<f32>,
    pub second_y: Option<f32>,
}

#[derive(Component)]
struct SelectionRectMarker;

pub struct SelectionChanged {
    pub mouse_button: MouseButton,
}
pub struct OnSelected {
    pub entities: Vec<Entity>,
    pub mouse_button: MouseButton,
}

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SelectionChanged>()
            .add_event::<OnSelected>()
            .insert_resource(SelectionRect::default())
            .add_system(mouse_button_input)
            .add_system(selection_changed);
    }
}

fn mouse_button_input(
    buttons: Res<Input<MouseButton>>,
    mut selection_rect: ResMut<SelectionRect>,
    windows: Res<Windows>,
    mut query: Query<&mut Transform, With<TopDownCamera>>,
    mut ev_selection_changed: EventWriter<SelectionChanged>,

    mut commands: Commands,
    q_selection_marker: Query<Entity, With<SelectionRectMarker>>,
) {
    let selection_marker = q_selection_marker.get_single();
    if let Ok(selection_marker) = selection_marker {
        commands.entity(selection_marker).despawn();
    }

    let window = some_or_return!(windows.get_primary());
    let cursor_position = some_or_return!(window.cursor_position());
    let transform = ok_or_return!(query.get_single_mut());

    let just_pressed_left = buttons.just_pressed(MouseButton::Left);
    let just_pressed_right = buttons.just_pressed(MouseButton::Right);
    let just_pressed = just_pressed_left || just_pressed_right;

    if just_pressed {
        let world_pos = screen_to_world(
            &transform,
            cursor_position,
            Vec2::new(window.width() as f32, window.height() as f32),
        );

        selection_rect.first_x = Some(world_pos.x);
        selection_rect.first_y = Some(world_pos.y);
        selection_rect.second_x = None;
        selection_rect.second_y = None;

        ev_selection_changed.send(SelectionChanged {
            mouse_button: if just_pressed_left {
                MouseButton::Left
            } else {
                MouseButton::Right
            },
        });
    }

    if buttons.pressed(MouseButton::Left) || buttons.pressed(MouseButton::Right) {
        let world_pos = screen_to_world(
            &transform,
            cursor_position,
            Vec2::new(window.width() as f32, window.height() as f32),
        );

        let (x1, x2) = (some_or_return!(selection_rect.first_x), world_pos.x);
        let (y1, y2) = (some_or_return!(selection_rect.first_y), world_pos.y);

        let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        let (y1, y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

        let rect_shape = shapes::Rectangle {
            extents: Vec2::new(x2 - x1, y2 - y1),
            origin: shapes::RectangleOrigin::BottomLeft,
        };
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &rect_shape,
                // ShapeColors::outlined(Color::rgba(0.0, 0.0, 0.0, 0.0), Color::WHITE),
                DrawMode::Outlined {
                    fill_mode: FillMode {
                        options: FillOptions::default(),
                        color: Color::rgba(0.0, 0.0, 0.0, 0.0),
                    },
                    outline_mode: StrokeMode {
                        options: StrokeOptions::default().with_line_width(2.0 * transform.scale.x),
                        color: Color::WHITE,
                    },
                },
                Transform::from_xyz(x1, y1, 10.0),
            ))
            .insert(SelectionRectMarker);
    }

    let just_released_left = buttons.just_released(MouseButton::Left);
    let just_released_right = buttons.just_released(MouseButton::Right);
    let just_released = just_released_left || just_released_right;

    if just_released {
        let world_pos = screen_to_world(
            &transform,
            cursor_position,
            Vec2::new(window.width() as f32, window.height() as f32),
        );

        selection_rect.second_x = Some(world_pos.x);
        selection_rect.second_y = Some(world_pos.y);

        ev_selection_changed.send(SelectionChanged {
            mouse_button: if just_released_left {
                MouseButton::Left
            } else {
                MouseButton::Right
            },
        });
    }
}

fn selection_changed(
    selection_rect: Res<SelectionRect>,
    mut ev_selection_changed: EventReader<SelectionChanged>,
    mut ev_selected: EventWriter<OnSelected>,
    q_selectable: Query<(&Transform, Entity), With<Selectable>>,
) {
    for event in ev_selection_changed.iter() {
        let (x1, x2) = (
            some_or_return!(selection_rect.first_x),
            some_or_return!(selection_rect.second_x),
        );
        let (y1, y2) = (
            some_or_return!(selection_rect.first_y),
            some_or_return!(selection_rect.second_y),
        );

        let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        let (y1, y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

        let mut selected = Vec::new();

        for (transform, entity) in q_selectable.iter() {
            let pos_x = transform.translation.x;
            let pos_y = transform.translation.y;
            if !(pos_x > x1 && x2 > pos_x && pos_y > y1 && y2 > pos_y) {
                continue;
            }

            selected.push(entity);
        }

        ev_selected.send(OnSelected {
            entities: selected,
            mouse_button: event.mouse_button,
        });
    }
}
