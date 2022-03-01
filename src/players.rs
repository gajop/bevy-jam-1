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

pub struct PlayerPlugin;

struct GeneratedPlayers {
    generated: bool,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GeneratedPlayers { generated: false })
            .add_system(generate_players.after("star-generation"))
            .add_system(color_assigned_star);
    }
}

fn generate_players(
    mut commands: Commands,
    star_query: Query<(Entity, &Star)>,
    mut generated_players: ResMut<GeneratedPlayers>,
) {
    if generated_players.generated {
        return;
    }

    if star_query.iter().len() == 0 {
        return;
    }

    info!("Generate players!");
    info!("Stars count: {}", star_query.iter().len());

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

    for i in 0..50 {
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
    star_query: &Query<(Entity, &Star)>,
    commands: &mut Commands,
) {
    let random_star = rand::thread_rng().gen_range(0..star_query.iter().len());
    let (entity, star) = star_query.iter().nth(random_star).unwrap();
    commands.entity(entity).insert(OwnedBy { player_id });
}

fn color_assigned_star(
    mut query: Query<(Entity, &Star, &mut Sprite, &OwnedBy), Added<OwnedBy>>,
    player_query: Query<&Player>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Bottom,
        horizontal: HorizontalAlign::Center,
    };
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };

    for (entity, assigned_star, mut sprite, owned_by) in query.iter_mut() {
        let player = find_player_by_id(owned_by.player_id, &player_query);
        if player.is_none() {
            continue;
        }
        let player = player.unwrap();

        sprite.color = player.color;

        let label = commands
            .spawn_bundle(Text2dBundle {
                text: Text::with_section(
                    player.name.to_string(),
                    text_style.clone(),
                    text_alignment,
                ),
                transform: Transform::from_xyz(0.0, 55.0, 0.0),
                ..Default::default()
            })
            .id();

        commands.entity(entity).push_children(&[label]);
    }
}

fn find_player_by_id<'a>(player_id: usize, player_query: &'a Query<&Player>) -> Option<&'a Player> {
    for player in player_query.iter() {
        if player.id == player_id {
            return Some(player);
        }
    }
    None
}
