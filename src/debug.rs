use bevy::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // TODO: Re-add bevy_inspector_egui when it's ported to Bevy 0.11
        // app.add_plugins(WorldInspectorPlugin::new());
    }
}
