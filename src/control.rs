use bevy::prelude::*;

use crate::{
    players::{OwnedBy, Player},
    selection::OnSelected,
    selection_ui::Selected,
    ship::{AttachedFleet, Fleet, FlyTo},
    top_down_camera::{screen_to_world, TopDownCamera},
};

struct SelectedSingle {
    fleet: Option<Entity>,
    star: Option<Entity>,
}

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedSingle {
            fleet: None,
            star: None,
        })
        // .add_system(mouse_select)
        // .add_system(mouse_send)
        .add_system(attack_selection);
    }
}

/*

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
            window.width() as f32,
            window.height() as f32,
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

 */

fn mouse_send(
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut selected: ResMut<SelectedSingle>,
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
        Vec2::new(window.width() as f32, window.height() as f32),
    );

    if buttons.just_pressed(MouseButton::Right) {
        if selected.fleet.is_none() {
            return;
        }
        let selected_fleet = selected.fleet.unwrap();
        let selected_star = selected.star.unwrap();
        let mut fleet = q_fleet.get_mut(selected_fleet).unwrap();
        for (entity, sprite, transform) in q_stars.iter() {
            let player = ok_or_continue!(q_player.get(fleet.player));
            if !player.is_human {
                continue;
            }

            if in_sprite(world_pos, sprite, transform) {
                let send_fleet_size = fleet.size * 0.5;
                fleet.size -= send_fleet_size;

                commands
                    .spawn()
                    .insert(Fleet {
                        player: fleet.player,
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

fn attack_selection(
    mut ev_selected: EventReader<OnSelected>,

    q_selected: Query<&Parent, With<Selected>>,
    q_attached_fleet: Query<(Option<&OwnedBy>, Option<&AttachedFleet>, Entity)>,
    mut q_fleet: Query<&mut Fleet>,
    q_player: Query<&Player>,

    mut commands: Commands,
) {
    for event in ev_selected.iter() {
        if event.mouse_button != MouseButton::Right {
            continue;
        }

        let my_fleets_stars: Vec<_> = q_selected
            .iter()
            .filter_map(|&my_entity| {
                let (owned_by, attached_fleet, entity) = q_attached_fleet.get(my_entity.0).ok()?;
                let owned_by = owned_by?;
                let attached_fleet = attached_fleet?;

                let player = q_player.get(owned_by.player).ok()?;
                if !player.is_human {
                    return None;
                }

                // let mut fleet = q_fleet.get_mut(attached_fleet.fleet_id).ok()?;
                let fleet = q_fleet.get_mut(attached_fleet.fleet_id).ok()?;

                Some((attached_fleet.fleet_id, entity))
            })
            .collect();

        let target_stars = event.entities.iter().filter_map(|&target_entity| {
            let (owned_by, _, star) = q_attached_fleet.get(target_entity).ok()?;
            let owned_by = some_or_return!(owned_by, Some(star));

            let player = q_player.get(owned_by.player).ok()?;
            if player.is_human {
                return None;
            }

            Some(star)
        });

        // info!("Our stars: {}", my_fleets_stars.count());
        // info!("Target stars: {}", target_stars.count());

        for ((fleet_id, star_entity), target_star) in
            my_fleets_stars.iter().zip(target_stars.cycle())
        {
            let mut my_fleet = ok_or_continue!(q_fleet.get_mut(*fleet_id));

            let send_fleet_size = my_fleet.size * 0.5;
            my_fleet.size -= send_fleet_size;

            commands
                .spawn()
                .insert(Fleet {
                    player: my_fleet.player,
                    size: send_fleet_size,
                })
                .insert(FlyTo {
                    origin_star: *star_entity,
                    destination_star: target_star,
                });
        }
    }
}
