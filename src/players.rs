use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use rand::Rng;

use crate::star_generation::Star;

#[derive(Component, Inspectable)]
pub struct Player {
    pub name: String,
    pub is_human: bool,
    pub id: usize,
    pub color: Color,
}

#[derive(Component)]
pub struct OwnedBy {
    pub player: Entity,
}

struct GeneratedPlayers {
    generated: bool,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GeneratedPlayers { generated: false })
            .add_system(generate_players);
    }
}

fn generate_players(
    mut commands: Commands,
    star_query: Query<Entity, With<Star>>,
    mut generated_players: ResMut<GeneratedPlayers>,
) {
    if generated_players.generated {
        return;
    }

    if star_query.iter().count() == 0 {
        return;
    }

    info!("Generate players!");
    info!("Stars count: {}", star_query.iter().count());

    generated_players.generated = true;
    // let stars = star_query.iter() {

    // }

    for i in 0..10 {
        let player_id = 1 + i;
        let player = commands
            .spawn()
            .insert(Player {
                name: format!("AI: {}", i + 1),
                is_human: false,
                id: player_id,
                color: Color::Rgba {
                    red: rand::thread_rng().gen_range(0.0..=1.0),
                    green: rand::thread_rng().gen_range(0.0..=1.0),
                    blue: rand::thread_rng().gen_range(0.0..=1.0),
                    alpha: 1.0,
                },
            })
            .id();
        assign_random_star_to_player(player, &star_query, &mut commands);
    }

    let player = commands
        .spawn()
        .insert(Player {
            name: "You".to_string(),
            is_human: true,
            id: 0,
            color: Color::Rgba {
                red: rand::thread_rng().gen_range(0.0..=1.0),
                green: rand::thread_rng().gen_range(0.0..=1.0),
                blue: rand::thread_rng().gen_range(0.0..=1.0),
                alpha: 1.0,
            },
        })
        .id();

    assign_random_star_to_player(player, &star_query, &mut commands);
}

fn assign_random_star_to_player(
    player: Entity,
    star_query: &Query<Entity, With<Star>>,
    commands: &mut Commands,
) {
    let random_star = rand::thread_rng().gen_range(0..star_query.iter().count());
    let entity = star_query.iter().nth(random_star).unwrap();
    commands.entity(entity).insert(OwnedBy { player });
}