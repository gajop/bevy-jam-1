use bevy::prelude::*;

use bevy_prototype_lyon::prelude::*;
use ctrl_macros::ok_or_continue;

use crate::{
    players::{OwnedBy, Player},
    selection::*,
};

#[derive(Component)]
pub struct Selected;

pub struct SelectionUIPlugin;

impl Plugin for SelectionUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mark_selected_with_rectangle);
    }
}

fn mark_selected_with_rectangle(
    mut ev_selected: EventReader<OnSelected>,

    q_selected_marker: Query<Entity, With<Selected>>,
    q_selectable: Query<(&Selectable, &Transform)>,
    q_player: Query<&Player>,
    q_owner: Query<&OwnedBy>,

    mut commands: Commands,
) {
    for event in ev_selected.iter() {
        if event.mouse_button != MouseButton::Left {
            continue;
        }

        for entity in q_selected_marker.iter() {
            commands.entity(entity).despawn_recursive();
        }

        for &entity in event.entities.iter() {
            let owner = ok_or_continue!(q_owner.get(entity));

            let player = ok_or_continue!(q_player.get(owner.player));
            if !player.is_human {
                continue;
            }

            let (selectable, transform) = q_selectable.get(entity).unwrap();

            let rect_shape = shapes::Rectangle {
                extents: Vec2::new(selectable.width, selectable.height),
                origin: shapes::RectangleOrigin::Center,
            };

            let child = commands
                .spawn((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&rect_shape),
                        transform: Transform::from_xyz(0.0, 0.0, 5.0),
                        ..default()
                    },
                    Fill::color(Color::rgba(0.0, 0.0, 0.0, 0.0)),
                    Stroke {
                        options: StrokeOptions::default().with_line_width(2.0 * transform.scale.x),
                        color: Color::PURPLE,
                    },
                ))
                .insert(Selected)
                .id();

            commands.entity(entity).push_children(&[child]);
        }
    }
}
