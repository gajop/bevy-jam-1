use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use ctrl_macros::ok_or_continue;

use crate::{
    players::{OwnedBy, Player},
    ship::{AttachedFleet, Fleet, FlyTo},
    star_generation::Star,
};

const EVERY_FIVE_SECONDS: f32 = 5.0;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, send_fleet.run_if(on_timer(Duration::from_secs_f32(EVERY_FIVE_SECONDS))));
    }
}

fn send_fleet(
    q_attached_fleet: Query<(Entity, &AttachedFleet, &Transform)>,
    q_enemy_stars: Query<(Entity, Option<&OwnedBy>, &Transform), With<Star>>,
    mut q_fleet: Query<&mut Fleet>,
    q_player: Query<&Player>,
    mut commands: Commands,
) {
    for (first_entity, attached_fleet, transform) in q_attached_fleet.iter() {
        let mut fleet = q_fleet.get_mut(attached_fleet.fleet_id).unwrap();

        let player = ok_or_continue!(q_player.get(fleet.player));
        if player.is_human {
            continue;
        }

        let mut closest_distance = f32::MAX;
        let mut selected_enemy = None;

        for (enemy, other_star, other_transfrorm) in q_enemy_stars.iter() {
            if first_entity == enemy {
                continue;
            }
            if let Some(other_star) = other_star {
                if fleet.player == other_star.player {
                    continue;
                }
            }

            let dx = transform.translation.x - other_transfrorm.translation.x;
            let dy = transform.translation.y - other_transfrorm.translation.y;
            let distance_squared = dx * dx + dy * dy;
            if distance_squared < closest_distance {
                closest_distance = distance_squared;
                selected_enemy = Some(enemy);
            }
        }

        if let Some(selected_enemy) = selected_enemy {
            let send_fleet_size = fleet.size * 0.5;
            fleet.size -= send_fleet_size;

            commands
                .spawn_empty()
                .insert(Fleet {
                    player: fleet.player,
                    size: send_fleet_size,
                })
                .insert(FlyTo {
                    origin_star: first_entity,
                    destination_star: selected_enemy,
                });
            // info!("Send fleet size: {send_fleet_size} {closest_distance}");
        }
    }
}
