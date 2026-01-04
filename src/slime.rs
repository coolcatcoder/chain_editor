use crate::{SpawnParameters, prelude::*};
use avian3d::prelude::*;
use bevy::prelude::*;

mod map;

pub fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            crate::plugin,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin,
        ))
        .insert_gizmo_config(
            PhysicsGizmos {
                axis_lengths: None,
                ..default()
            },
            GizmoConfig::default(),
        )
        .add_systems(Startup, start)
        .run();
}

fn start(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SceneRoot(asset_server.load("slime_map/mesh.glb#Scene0")));
    //map::spawn(SpawnParameters::new(&asset_server, &mut commands));

    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3(7., 7., -7.)).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
