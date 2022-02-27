use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;

use camera::CameraPlugin;
use debug::DebugPlugin;
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

mod camera;
mod debug;
mod star_generation;
mod top_down_camera;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(StarGenerationPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(CameraPlugin)
        .insert_resource(Msaa { samples: 4 })
        .add_startup_system(setup);

    #[cfg(target_arch = "wasm32")]
    {
        app.add_system(bevy_web_resizer::web_resize_system);
    }

    app.add_system(bevy_web_resizer::web_resize_system).run();
}

fn setup(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    // mut events: EventWriter<MakeCluster>,
) {
    // commands
    //     .spawn_bundle(OrthographicCameraBundle::new_2d().insert(camera::TopDownCamera::default()));
}
