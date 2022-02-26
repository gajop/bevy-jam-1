use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};

#[derive(Inspectable, Component)]
pub struct InspectableType;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct ReflectedType;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .register_inspectable::<InspectableType>() // tells bevy-inspector-egui how to display the struct in the world inspector
            .register_type::<ReflectedType>(); // registers the type in the `bevy_reflect` machinery, so that even without implementing `Inspectable` we can display the struct fields
    }
}
