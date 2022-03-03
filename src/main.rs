#![allow(clippy::type_complexity)]

use ai::AiPlugin;
use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;

use camera::CameraPlugin;
use debug::DebugPlugin;
use game_ui::GameUiPlugin;
use players::PlayerPlugin;
use ship::ShipPlugin;
use star_generation::StarGenerationPlugin;

macro_rules! ok_or_return {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return,
        }
    };
}

macro_rules! some_or_return {
    ( $e:expr ) => {
        match $e {
            Some(x) => x,
            None => return,
        }
    };
}

mod ai;
mod camera;
mod debug;
mod game_ui;
mod players;
mod ship;
mod star_generation;
mod top_down_camera;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(AiPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(GameUiPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(ShipPlugin)
        .add_plugin(StarGenerationPlugin)
        .insert_resource(Msaa { samples: 4 });

    #[cfg(target_arch = "wasm32")]
    {
        app.add_system(bevy_web_resizer::web_resize_system);
    }

    app.run();
}
