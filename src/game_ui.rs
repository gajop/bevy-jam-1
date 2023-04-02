use std::collections::HashMap;

use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::*, shapes};
use ctrl_macros::{ok_or_continue, ok_or_return, some_or_return};

use crate::{
    players::{OwnedBy, Player},
    ship::{AttachedFleet, Fleet},
    star_generation::Star,
};

pub struct GameUiPlugin;

#[derive(Component)]
pub struct StarText;

#[derive(Component)]
pub struct OwnershipCircle;

#[derive(Component)]
pub struct PlayerStarText;

#[derive(Component)]
pub struct PlayerScoreHolder;

#[derive(Component)]
pub struct PlayerScore {
    player: Entity,
}

#[derive(Component)]
pub struct ResultText;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_player_score_ui)
            .add_system(add_player_score)
            .add_system(update_player_score)
            .add_system(player_assigned_star)
            .add_system(star_assignment_changed)
            .add_system(star_resource_label)
            .add_system(update_star_text);
    }
}

fn player_assigned_star(
    mut query: Query<(Entity, &mut Sprite, &OwnedBy), (With<Star>, Added<OwnedBy>)>,
    player_query: Query<&Player>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let text_alignment = TextAlignment::Center;
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 40.0,
        color: Color::WHITE,
    };

    let shape = shapes::Circle {
        radius: 100.0,
        ..shapes::Circle::default()
    };

    for (entity, mut sprite, owned_by) in query.iter_mut() {
        let player = ok_or_continue!(player_query.get(owned_by.player));

        sprite.color = player.color;

        let label = commands
            .spawn(Text2dBundle {
                text: Text::from_section(player.name.to_string(), text_style.clone())
                    .with_alignment(text_alignment),
                transform: Transform::from_xyz(0.0, -15.0, 0.0)
                    .with_scale(Vec3::new(0.2, 0.2, 0.2)),
                // text_anchor: Anchor::TopCenter,
                ..default()
            })
            .insert(PlayerStarText)
            .id();

        let ownership_circle = commands
            .spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    ..default()
                },
                Fill::color(*player.color.clone().set_a(0.2)),
            ))
            .insert(OwnershipCircle)
            .id();

        commands
            .entity(entity)
            .push_children(&[label, ownership_circle]);
    }
}

fn star_assignment_changed(
    mut query_star: Query<(&mut Sprite, &OwnedBy, &Children), (With<Star>, Changed<OwnedBy>)>,
    mut q_player_star_text: Query<&mut Text, With<PlayerStarText>>,
    mut q_ownership_circle: Query<&mut Fill, With<OwnershipCircle>>,
    player_query: Query<&Player>,
) {
    for (mut sprite, owned_by, children) in query_star.iter_mut() {
        let player = ok_or_continue!(player_query.get(owned_by.player));

        sprite.color = player.color;

        // q_attached_fleet: Query<(&AttachedFleet, &Children)>,
        // mut q_star_text: Query<&mut Text, With<StarText>>,

        for &child in children.iter() {
            let text = q_player_star_text.get_mut(child);
            if let Ok(mut text) = text {
                text.sections[0].value = player.name.to_string();
            }

            let fill = q_ownership_circle.get_mut(child);
            if let Ok(mut fill) = fill {
                fill.color = *player.color.clone().set_a(0.2);
            }
        }
    }
}

fn star_resource_label(
    query: Query<(Entity, &Star), Added<Star>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let text_alignment = TextAlignment::Center;
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 40.0,
        color: Color::WHITE,
    };

    for (entity, star) in query.iter() {
        let label = commands
            .spawn(Text2dBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: format!("M: {:.2}", star.size),
                            style: text_style.clone(),
                        },
                        TextSection {
                            value: "".to_string(), // fleet placeholder
                            style: text_style.clone(),
                        },
                    ],
                    alignment: text_alignment,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 15.0, 0.0).with_scale(Vec3::new(0.2, 0.2, 0.2)),
                ..default()
            })
            .insert(StarText)
            .id();

        commands.entity(entity).push_children(&[label]);
    }
}

fn update_star_text(
    q_attached_fleet: Query<(&AttachedFleet, &Children)>,
    mut q_star_text: Query<&mut Text, With<StarText>>,
    q_fleet: Query<&Fleet>,
) {
    for (attached_fleet, children) in q_attached_fleet.iter() {
        let fleet = q_fleet.get(attached_fleet.fleet_id).unwrap();

        for &child in children.iter() {
            let text = q_star_text.get_mut(child);
            if let Ok(mut text) = text {
                text.sections[1].value = format!("  F: {:.2}", fleet.size);
            }
        }
    }
}

fn setup_player_score_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // PlayerScoreHolder
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .insert(PlayerScoreHolder);

    commands
        .spawn(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Percent(45.0),
                    right: Val::Percent(50.0),
                    bottom: Val::Percent(0.0),
                    ..default()
                },

                ..default()
            },
            text: Text::from_section(
                "".to_string(),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::Center),

            ..default()
        })
        .insert(ResultText);
}

fn add_player_score(
    q_player_add: Query<(&Player, Entity), Added<Player>>,
    q_holder: Query<Entity, With<PlayerScoreHolder>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let holder = ok_or_return!(q_holder.get_single());
    for (i, (player, player_entity)) in q_player_add.iter().enumerate() {
        let player_score = commands
            .spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        top: Val::Px(22.0 * ((i + 1) as f32)),
                        right: Val::Px(15.0),
                        ..default()
                    },
                    ..default()
                },
                text: Text::from_section(
                    format!("{}:   1 stars", player.name),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.0,
                        color: player.color,
                    },
                )
                .with_alignment(TextAlignment::Center),
                ..default()
            })
            .insert(PlayerScore {
                player: player_entity,
            })
            .id();

        commands.entity(holder).add_child(player_score);
    }
}

fn update_player_score(
    q_owned_star: Query<&OwnedBy, With<Star>>,
    q_player: Query<&Player>,
    mut q_player_score: Query<(&mut Text, &PlayerScore), Without<ResultText>>,
    mut q_result_text: Query<&mut Text, With<ResultText>>,
) {
    let mut score_map = HashMap::new();
    let mut total_stars = 0;
    for owned_by in q_owned_star.iter() {
        *score_map.entry(owned_by.player).or_insert(0) += 1;
        total_stars += 1;
    }

    for (mut text, player_score) in q_player_score.iter_mut() {
        let player = ok_or_continue!(q_player.get(player_score.player));
        text.sections[0].value = format!(
            "{}:   {:?} stars",
            player.name,
            score_map.get(&player_score.player).unwrap_or(&0)
        );
    }

    let mut human = None;
    for (_, player_score) in q_player_score.iter() {
        let player = ok_or_continue!(q_player.get(player_score.player));
        if player.is_human {
            human = Some(player_score.player);
            break;
        }
    }

    let &our_score = score_map.get(&some_or_return!(human)).unwrap_or(&0);
    let mut result_text = ok_or_return!(q_result_text.get_single_mut());
    result_text.sections[0].value = if our_score == 0 {
        "Defeat!\nRefresh to play again".to_string()
    } else if our_score as f32 > total_stars as f32 * 0.8 {
        "Victory!\nRefresh to play again".to_string()
    } else {
        "TEST".to_string()
    };
}
