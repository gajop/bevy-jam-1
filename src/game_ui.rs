use bevy::prelude::*;

use crate::{
    players::{find_player_by_id, OwnedBy, Player},
    star_generation::Star,
};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(color_assigned_star)
            .add_system(star_resource_label);
    }
}

fn color_assigned_star(
    mut query: Query<(Entity, &mut Sprite, &OwnedBy), (With<Star>, Added<OwnedBy>)>,
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
                transform: Transform::from_xyz(0.0, 55.0, 0.0),
                ..Default::default()
            })
            .id();

        commands.entity(entity).push_children(&[label]);
    }
}

fn star_resource_label(
    query: Query<(Entity, &Star), Added<Star>>,
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
        font_size: 60.0,
        color: Color::WHITE,
    };

    for (entity, star) in query.iter() {
        let label = commands
            .spawn_bundle(Text2dBundle {
                text: Text::with_section(
                    format!("{:.2}M", star.size),
                    text_style.clone(),
                    text_alignment,
                ),
                transform: Transform::from_xyz(0.0, -55.0, 0.0),
                ..Default::default()
            })
            .id();

        commands.entity(entity).push_children(&[label]);
    }
}
