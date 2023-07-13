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
        .add_plugins(AiPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(ControlPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(GameUiPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(ShapePlugin)
        .add_plugins(ShipPlugin)
        .add_plugins(SelectionPlugin)
        .add_plugins(SelectionUIPlugin)
        .add_plugins(StarGenerationPlugin)
        .insert_resource(Msaa::Sample4);

    #[cfg(target_arch = "wasm32")]
    {
        app.add_plugins(bevy_web_resizer::Plugin);
    }

    app.run();
}
