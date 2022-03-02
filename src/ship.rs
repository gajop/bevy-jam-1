use bevy::{core::FixedTimestep, prelude::*};

use crate::{
    players::{find_player_by_id, OwnedBy, Player},
    star_generation::Star,
};

const TWICE_PER_SECOND: f64 = 30.0 / 60.0;

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

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TWICE_PER_SECOND))
                .with_system(generate_ships_at_owned_stars),
        )
        .add_system(generate_new_ships_at_owned_stars);
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
