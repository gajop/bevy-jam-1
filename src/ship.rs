use bevy::{core::FixedTimestep, prelude::*};
use ctrl_macros::ok_or_continue;

use crate::{
    players::{OwnedBy, Player},
    star_generation::Star,
};

const TWICE_PER_SECOND: f64 = 30.0 / 60.0;
const FLY_TO_TIME_STEP: f64 = 1.0 / 60.0;

pub struct ShipPlugin;

#[derive(Component)]
pub struct Fleet {
    pub player: Entity,
    pub size: f32,
}

#[derive(Component)]
pub struct AttachedFleet {
    pub fleet_id: Entity,
}

#[derive(Component)]
pub struct FlyTo {
    pub origin_star: Entity,
    pub destination_star: Entity,
}

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TWICE_PER_SECOND))
                .with_system(generate_ships_at_owned_stars),
        )
        .add_system(generate_new_ships_at_owned_stars)
        .add_system(generate_icon_for_fly_to_ships)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(FLY_TO_TIME_STEP))
                .with_system(fly_to),
        )
        .add_system(fight)
        .add_system(change_fleet_ownership);
    }
}

fn generate_ships_at_owned_stars(
    query: Query<(&AttachedFleet, &Star)>,
    mut fleet_query: Query<&mut Fleet>,
) {
    for (attached_fleet, star) in query.iter() {
        let mut fleet = fleet_query.get_mut(attached_fleet.fleet_id).unwrap();
        fleet.size += star.size * 0.1;
    }
}

fn generate_new_ships_at_owned_stars(
    mut query: Query<(Entity, &OwnedBy), (With<Star>, Without<AttachedFleet>)>,
    player_query: Query<&Player>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (entity, owned_by) in query.iter_mut() {
        let player = ok_or_continue!(player_query.get(owned_by.player));

        let fleet = commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("enemy_E.png"),
                transform: Transform::from_xyz(10.0, 10.0, 0.0),
                sprite: Sprite {
                    color: player.color,
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Fleet {
                player: owned_by.player,
                size: 0.0,
            })
            .id();

        commands.entity(entity).push_children(&[fleet]);
        commands
            .entity(entity)
            .insert(AttachedFleet { fleet_id: fleet });
    }
}

fn generate_icon_for_fly_to_ships(
    query: Query<(Entity, &Fleet, &FlyTo), Added<FlyTo>>,
    q_origin: Query<&Transform>,
    q_player: Query<&Player>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (entity, fleet, fly_to) in query.iter() {
        let player = ok_or_continue!(q_player.get(fleet.player));

        let transform = *q_origin.get(fly_to.origin_star).unwrap();

        commands.entity(entity).insert_bundle(SpriteBundle {
            texture: asset_server.load("enemy_E.png"),
            sprite: Sprite {
                color: player.color,
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..Default::default()
            },
            transform,
            ..Default::default()
        });
    }
}

fn fly_to(
    mut q_fly_to: Query<(&FlyTo, &mut Transform)>,
    q_destination: Query<&Transform, Without<FlyTo>>,
) {
    for (fly_to, mut transform) in q_fly_to.iter_mut() {
        let destination_transform = q_destination.get(fly_to.destination_star).unwrap();
        let delta = (destination_transform.translation - transform.translation).normalize()
            * (FLY_TO_TIME_STEP as f32)
            * 100.0;
        transform.translation += delta;

        // TODO: figure out proper rotation later

        // let target = destination_transform.translation.truncate();
        // let pos = Vec2::new(transform.translation.x, transform.translation.y);
        // let angle = (target - pos).angle_between(pos);
        // transform.rotation = Quat::from_rotation_z(angle);
    }
}

fn fight(
    q_fly_to: Query<(Entity, &FlyTo, &Fleet, &Transform)>,
    mut q_destination: Query<
        (
            &Transform,
            &Star,
            Option<&AttachedFleet>,
            Option<&mut OwnedBy>,
        ),
        Without<FlyTo>,
    >,
    mut q_destination_fleet: Query<&mut Fleet, Without<FlyTo>>,
    mut commands: Commands,
) {
    for (entity, fly_to, fleet, transform) in q_fly_to.iter() {
        let (destination_transform, star, attached_fleet, owned_by) =
            q_destination.get_mut(fly_to.destination_star).unwrap();
        let distance = transform
            .translation
            .distance(destination_transform.translation);
        if distance < 1.0 {
            if let Some(attached_fleet) = attached_fleet {
                let mut target_fleet = q_destination_fleet
                    .get_mut(attached_fleet.fleet_id)
                    .unwrap();
                if target_fleet.player != fleet.player {
                    target_fleet.size -= fleet.size;
                } else {
                    target_fleet.size += fleet.size;
                }
                if target_fleet.size < 0.0 {
                    owned_by.unwrap().player = fleet.player;
                    target_fleet.player = fleet.player;
                    target_fleet.size *= -1.0;
                };
            } else {
                commands.entity(fly_to.destination_star).insert(OwnedBy {
                    player: fleet.player,
                });
            };

            commands.entity(entity).despawn();
        }
    }
}

fn change_fleet_ownership(
    mut query: Query<(&mut Sprite, &Fleet), Changed<Fleet>>,
    q_player: Query<&Player>,
) {
    for (mut sprite, fleet) in query.iter_mut() {
        let player = ok_or_continue!(q_player.get(fleet.player));

        sprite.color = player.color;
    }
}
