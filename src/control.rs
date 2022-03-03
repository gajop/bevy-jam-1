use bevy::prelude::*;

use crate::{
    players::{find_player_by_id, OwnedBy, Player},
    ship::{AttachedFleet, Fleet, FlyTo},
    top_down_camera::{screen_to_world, TopDownCamera},
};

struct Selected {
    fleet: Option<Entity>,
    star: Option<Entity>,
}

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Selected {
            fleet: None,
            star: None,
        })
        .add_system(mouse_select)
        .add_system(mouse_send);
    }
}

fn mouse_select(
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut selected: ResMut<Selected>,
    mut query: Query<&mut Transform, With<TopDownCamera>>,
    q_stars: Query<
        (Entity, &Sprite, &GlobalTransform, &OwnedBy, &AttachedFleet),
        Without<TopDownCamera>,
    >,
    q_player: Query<&Player>,
) {
    let window = some_or_return!(windows.get_primary());
    let cursor_position = some_or_return!(window.cursor_position());
    let transform = ok_or_return!(query.get_single_mut());

    let world_pos = screen_to_world(
        &transform,
        cursor_position,
        Vec2::new(
            window.physical_width() as f32,
            window.physical_height() as f32,
        ),
    );

    if buttons.just_pressed(MouseButton::Left) {
        for (entity, sprite, transform, owned_by, attached_fleet) in q_stars.iter() {
            let player = find_player_by_id(owned_by.player_id, &q_player);
            if player.is_none() {
                continue;
            }
            let player = player.unwrap();
            if !player.is_human {
                continue;
            }

            if in_sprite(world_pos, sprite, transform) {
                selected.fleet = Some(attached_fleet.fleet_id);
                selected.star = Some(entity);
                break;
            }
            // let height = sprite.size.height;

            // if transform.translation.x < world_pos.x &&
            //     transform.translation.x + custom.size.
        }
    }
}

fn mouse_send(
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut selected: ResMut<Selected>,
    mut query: Query<&mut Transform, With<TopDownCamera>>,
    q_stars: Query<(Entity, &Sprite, &GlobalTransform), Without<TopDownCamera>>,
    mut q_fleet: Query<&mut Fleet>,
    q_player: Query<&Player>,
    mut commands: Commands,
) {
    let window = some_or_return!(windows.get_primary());
    let cursor_position = some_or_return!(window.cursor_position());
    let transform = ok_or_return!(query.get_single_mut());

    let world_pos = screen_to_world(
        &transform,
        cursor_position,
        Vec2::new(
            window.physical_width() as f32,
            window.physical_height() as f32,
        ),
    );

    if buttons.just_pressed(MouseButton::Right) {
        if selected.fleet.is_none() {
            return;
        }
        let selected_fleet = selected.fleet.unwrap();
        let selected_star = selected.star.unwrap();
        let mut fleet = q_fleet.get_mut(selected_fleet).unwrap();
        for (entity, sprite, transform) in q_stars.iter() {
            let player = find_player_by_id(fleet.player_id, &q_player);
            if player.is_none() {
                continue;
            }
            let player = player.unwrap();
            if !player.is_human {
                continue;
            }

            if in_sprite(world_pos, sprite, transform) {
                let send_fleet_size = fleet.size * 0.5;
                fleet.size -= send_fleet_size;

                commands
                    .spawn()
                    .insert(Fleet {
                        player_id: fleet.player_id,
                        size: send_fleet_size,
                    })
                    .insert(FlyTo {
                        origin_star: selected_star,
                        destination_star: entity,
                    });

                break;
            }
        }
    }
}

fn in_sprite(world_pos: Vec2, sprite: &Sprite, transform: &GlobalTransform) -> bool {
    let size = sprite.custom_size.unwrap();
    let rect = Rect {
        left: transform.translation.x - size.x / 2.0,
        right: transform.translation.x + size.x / 2.0,
        bottom: transform.translation.y - size.y / 2.0,
        top: transform.translation.y + size.y / 2.0,
    };

    world_pos.x >= rect.left
        && world_pos.x < rect.right
        && world_pos.y >= rect.bottom
        && world_pos.y < rect.top
}
