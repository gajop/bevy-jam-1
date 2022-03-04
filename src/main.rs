#![allow(clippy::type_complexity)]

use ai::AiPlugin;
use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;

use camera::CameraPlugin;
use control::ControlPlugin;
use debug::DebugPlugin;
use game_ui::GameUiPlugin;
use players::PlayerPlugin;
use selection::SelectionPlugin;
use selection_ui::SelectionUIPlugin;
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

macro_rules! ok_or_continue {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => continue,
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
    ( $e:expr, $return_value:expr ) => {
        match $e {
            Some(x) => x,
            None => return $return_value,
        }
    };
}

macro_rules! some_or_continue {
    ( $e:expr ) => {
        match $e {
            Some(x) => x,
            None => continue,
        }
    };
}

mod ai;
mod camera;
mod control;
mod debug;
mod game_ui;
mod players;
mod selection;
mod selection_ui;
mod ship;
mod star_generation;
mod top_down_camera;
fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(AiPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(ControlPlugin)
        // .add_plugin(DebugPlugin)
        .add_plugin(GameUiPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(ShipPlugin)
        .add_plugin(SelectionPlugin)
        .add_plugin(SelectionUIPlugin)
        .add_plugin(StarGenerationPlugin)
        .insert_resource(Msaa { samples: 4 });

    #[cfg(target_arch = "wasm32")]
    {
        app.add_system(bevy_web_resizer::web_resize_system);
    }

    app.run();
}
