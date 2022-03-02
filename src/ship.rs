use bevy::prelude::*;

use crate::{
    players::{find_player_by_id, OwnedBy, Player},
    star_generation::Star,
};

pub struct ShipPlugin;

#[derive(Component)]
pub struct Fleet {
    player_id: usize,
}

#[derive(Component)]
pub struct AttachedFleet {
    fleet_id: Entity,
}

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(generate_ships_at_owned_stars)
            .add_system(generate_new_ships_at_owned_stars);
    }
}

fn generate_ships_at_owned_stars(
    mut query: Query<(Entity, &Star, &OwnedBy)>,
    player_query: Query<&Player>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (entity, mut sprite, owned_by) in query.iter_mut() {
        // info!("Owned star: {}", owned_by.player_id);
    }
}

fn generate_new_ships_at_owned_stars(
    mut query: Query<(Entity, &Star, &OwnedBy), Without<AttachedFleet>>,
    player_query: Query<&Player>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (entity, mut star, owned_by) in query.iter_mut() {
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
            })
            .id();

        commands.entity(entity).push_children(&[fleet]);
        commands
            .entity(entity)
            .insert(AttachedFleet { fleet_id: fleet });
    }
}
