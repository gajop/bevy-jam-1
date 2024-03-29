use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::Rng;
use rand_distr::{Distribution, Normal};

use crate::selection::Selectable;

const BAND_SIZE_MIN: f32 = 170.0;
const BAND_SIZE_MAX: f32 = 200.0;
const BAND_Z_INDEX_START: f32 = -10.0;
const EMPTY_AREA_SIZE_MIN: f32 = 250.0;
const EMPTY_AREA_SIZE_MAX: f32 = 500.0;
const MIN_STARS_IN_CLUSTER: u32 = 1;
const MAX_STARS_IN_CLUSTER: u32 = 5;
const STAR_SIZE_MEAN: f32 = 2.0;
const STAR_SIZE_DEVIATION: f32 = 2.0;

#[derive(Component)]
struct Band {
    // index: i32,
    cluster_count: i32,
    distance_from_center: f32,
    size: f32,
}

pub struct NewStar {
    x: f32,
    y: f32,
    size: f32,
}

#[derive(Component)]
pub struct Star {
    pub size: f32,
}

pub struct StarGenerationPlugin;

impl Plugin for StarGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_bands);
    }
}

fn generate_bands(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Generate bands!");
    // Empty area between bands
    let size = rand::thread_rng().gen_range(EMPTY_AREA_SIZE_MIN..=EMPTY_AREA_SIZE_MAX);
    let mut band_size_total = size;

    let shape = shapes::Circle {
        radius: band_size_total,
        ..shapes::Circle::default()
    };
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            transform: Transform::from_xyz(0.0, 0.0, BAND_Z_INDEX_START + 1.0),
            ..default()
        },
        Fill::color(Color::Rgba {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }),
    ));

    for index in 0..5 {
        let size = rand::thread_rng().gen_range(BAND_SIZE_MIN..=BAND_SIZE_MAX);
        let distance_from_center = band_size_total;
        band_size_total += size;

        // Band
        let shape = shapes::Circle {
            radius: distance_from_center + size,
            ..shapes::Circle::default()
        };

        let cluster_count = ((index + 1) as f32 * 2.0 * PI) as i32;

        let band = Band {
            // index,
            distance_from_center,
            size,
            cluster_count,
        };
        generate_clusters_for_band(&band, &mut commands, &asset_server);
        commands
            .spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    transform: Transform::from_xyz(
                        0.0,
                        0.0,
                        BAND_Z_INDEX_START - (index as f32) * 2.0,
                    ),
                    ..default()
                },
                Fill::color(Color::Rgba {
                    red: rand::thread_rng().gen_range(0.0..=1.0),
                    green: rand::thread_rng().gen_range(0.0..=1.0),
                    blue: rand::thread_rng().gen_range(0.0..=1.0),
                    // alpha: 1.0,
                    alpha: 0.0,
                }),
            ))
            .insert(band);

        // Empty area between bands
        let size = rand::thread_rng().gen_range(EMPTY_AREA_SIZE_MIN..EMPTY_AREA_SIZE_MAX);
        band_size_total += size;

        let shape = shapes::Circle {
            radius: band_size_total,
            ..shapes::Circle::default()
        };
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                transform: Transform::from_xyz(
                    0.0,
                    0.0,
                    BAND_Z_INDEX_START - (index as f32) * 2.0 - 1.0,
                ),
                ..default()
            },
            Fill::color(Color::Rgba {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                // alpha: 1.0,
                alpha: 0.0,
            }),
        ));
    }
}

// fn generate_clusters(
//     query: Query<&Band, Added<Band>>,
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
// ) {
//     // for band in query.iter() {
//     //     generate_clusters_for_band(band, &mut commands, &asset_server);
//     // }
// }

fn generate_clusters_for_band(
    band: &Band,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let star_size_gen = Normal::new(STAR_SIZE_MEAN, STAR_SIZE_DEVIATION).unwrap();

    for cluster_index in 0..band.cluster_count {
        let dist = band.distance_from_center + band.size * 0.5;

        let cluster_angle = (cluster_index as f32 / band.cluster_count as f32) * PI * 2.0;
        let cluster_x = cluster_angle.sin() * dist;
        let cluster_y = cluster_angle.cos() * dist;

        let total_stars = rand::thread_rng().gen_range(MIN_STARS_IN_CLUSTER..=MAX_STARS_IN_CLUSTER);
        for start_index in 0..total_stars {
            let star_dist = rand::thread_rng().gen_range(0.1..=0.5) * band.size;
            let star_angle = (start_index as f32 / total_stars as f32) * PI * 2.0;

            let x = cluster_x + star_angle.sin() * star_dist;
            let y = cluster_y + star_angle.cos() * star_dist;
            // mean 2, standard deviation 3
            let star_size = star_size_gen
                .sample(&mut rand::thread_rng())
                .clamp(0.1, 10.0);

            add_star(
                commands,
                asset_server,
                NewStar {
                    x,
                    y,
                    size: star_size,
                },
            );
        }
    }
}

fn add_star(commands: &mut Commands, asset_server: &Res<AssetServer>, star: NewStar) {
    let size = Vec2::new(10.0 * star.size.sqrt(), 10.0 * star.size.sqrt());
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("star_large.png"),
            transform: Transform::from_xyz(star.x, star.y, 0.0),
            sprite: Sprite {
                custom_size: Some(size),
                ..default()
            },
            ..default()
        })
        .insert(Star { size: star.size })
        .insert(Selectable {
            width: size.x,
            height: size.y,
        });
}
