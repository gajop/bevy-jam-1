use bevy::prelude::*;

use crate::{
    players::{find_player_by_id, OwnedBy, Player},
    ship::{AttachedFleet, Fleet},
    star_generation::Star,
};

pub struct GameUiPlugin;

#[derive(Component)]
pub struct StarText;

#[derive(Component)]
pub struct PlayerStarText;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_assigned_star)
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
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Top,
        horizontal: HorizontalAlign::Center,
    };
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 40.0,
        color: Color::WHITE,
    };

    for (entity, mut sprite, owned_by) in query.iter_mut() {
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
                transform: Transform::from_xyz(0.0, -15.0, 0.0)
                    .with_scale(Vec3::new(0.2, 0.2, 0.2)),
                ..Default::default()
            })
            .insert(PlayerStarText)
            .id();

        commands.entity(entity).push_children(&[label]);
    }
}

fn star_assignment_changed(
    mut query_star: Query<(&mut Sprite, &OwnedBy, &Children), (With<Star>, Changed<OwnedBy>)>,
    mut q_player_star_text: Query<&mut Text, With<PlayerStarText>>,
    player_query: Query<&Player>,
) {
    for (mut sprite, owned_by, children) in query_star.iter_mut() {
        let player = find_player_by_id(owned_by.player_id, &player_query);
        if player.is_none() {
            continue;
        }
        let player = player.unwrap();

        sprite.color = player.color;

        // q_attached_fleet: Query<(&AttachedFleet, &Children)>,
        // mut q_star_text: Query<&mut Text, With<StarText>>,

        for &child in children.iter() {
            let text = q_player_star_text.get_mut(child);
            if let Ok(mut text) = text {
                text.sections[0].value = player.name.to_string();
            }
        }
    }
}

fn star_resource_label(
    query: Query<(Entity, &Star), Added<Star>>,
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
        font_size: 40.0,
        color: Color::WHITE,
    };

    for (entity, star) in query.iter() {
        let label = commands
            .spawn_bundle(Text2dBundle {
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
                },
                transform: Transform::from_xyz(0.0, 15.0, 0.0).with_scale(Vec3::new(0.2, 0.2, 0.2)),
                ..Default::default()
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
