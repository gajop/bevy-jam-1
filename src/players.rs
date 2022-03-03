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
    pub player_id: usize,
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

    commands.spawn().insert(Player {
        name: "You".to_string(),
        is_human: true,
        id: 0,
        color: Color::Rgba {
            red: rand::thread_rng().gen_range(0.0..=1.0),
            green: rand::thread_rng().gen_range(0.0..=1.0),
            blue: rand::thread_rng().gen_range(0.0..=1.0),
            alpha: 1.0,
        },
    });

    assign_random_star_to_player(0, &star_query, &mut commands);

    for i in 0..5 {
        let player_id = 1 + i;
        commands.spawn().insert(Player {
            name: format!("AI: {i}"),
            is_human: false,
            id: player_id,
            color: Color::Rgba {
                red: rand::thread_rng().gen_range(0.0..=1.0),
                green: rand::thread_rng().gen_range(0.0..=1.0),
                blue: rand::thread_rng().gen_range(0.0..=1.0),
                alpha: 1.0,
            },
        });
        assign_random_star_to_player(player_id, &star_query, &mut commands);
    }
}

fn assign_random_star_to_player(
    player_id: usize,
    star_query: &Query<Entity, With<Star>>,
    commands: &mut Commands,
) {
    let random_star = rand::thread_rng().gen_range(0..star_query.iter().count());
    let entity = star_query.iter().nth(random_star).unwrap();
    commands.entity(entity).insert(OwnedBy { player_id });
}

pub fn find_player_by_id<'a>(
    player_id: usize,
    player_query: &'a Query<&Player>,
) -> Option<&'a Player> {
    for player in player_query.iter() {
        if player.id == player_id {
            return Some(player);
        }
    }
    None
}
