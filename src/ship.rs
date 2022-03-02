use bevy::{core::FixedTimestep, prelude::*};

use crate::{
    players::{find_player_by_id, OwnedBy, Player},
    star_generation::Star,
};

const TWICE_PER_SECOND: f64 = 30.0 / 60.0;
const FLY_TO_TIME_STEP: f64 = 1.0 / 60.0;

pub struct ShipPlugin;

#[derive(Component)]
pub struct Fleet {
    pub player_id: usize,
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
        let player = find_player_by_id(owned_by.player_id, &player_query);
        if player.is_none() {
            continue;
        }
        let player = player.unwrap();

        let fleet = commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("ship_L.png"),
                transform: Transform::from_xyz(50.0, 50.0, 0.0),
                sprite: Sprite {
                    color: player.color,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Fleet {
                player_id: owned_by.player_id,
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
    query: Query<(Entity, &Fleet, &FlyTo), (Added<FlyTo>)>,
    q_origin: Query<&Transform>,
    player_query: Query<&Player>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (entity, fleet, fly_to) in query.iter() {
        let player = find_player_by_id(fleet.player_id, &player_query);
        if player.is_none() {
            continue;
        }
        let player = player.unwrap();

        let mut transform = q_origin.get(fly_to.origin_star).unwrap().clone();
        transform.scale = Vec3::new(0.1, 0.1, 0.1);

        commands.entity(entity).insert_bundle(SpriteBundle {
            texture: asset_server.load("ship_L.png"),
            sprite: Sprite {
                color: player.color,
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
        let delta = (transform.translation - destination_transform.translation).normalize()
            * (FLY_TO_TIME_STEP as f32)
            * 100.0;
        transform.translation -= delta;
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
                if target_fleet.player_id != fleet.player_id {
                    target_fleet.size -= fleet.size;
                } else {
                    target_fleet.size += fleet.size;
                }
                if target_fleet.size < 0.0 {
                    owned_by.unwrap().player_id = fleet.player_id;
                    target_fleet.player_id = fleet.player_id;
                    target_fleet.size *= -1.0;
                };
            } else {
                commands.entity(fly_to.destination_star).insert(OwnedBy {
                    player_id: fleet.player_id,
                });
            };

            commands.entity(entity).despawn();
        }
    }
}

fn change_fleet_ownership(
    mut query: Query<(&mut Sprite, &Fleet), (Changed<Fleet>)>,
    player_query: Query<&Player>,
) {
    for (mut sprite, fleet) in query.iter_mut() {
        let player = find_player_by_id(fleet.player_id, &player_query);
        if player.is_none() {
            continue;
        }
        let player = player.unwrap();

        sprite.color = player.color;
    }
}
