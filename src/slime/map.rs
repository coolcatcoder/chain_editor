use bevy::prelude::*;
pub fn spawn(mut parameters: crate::SpawnParameters) {
    parameters
        .set_node(
            0u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(0.0026738343f32, 0.3226377f32, -0.023313284f32)),
        );
    parameters
        .set_node(
            2u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(0.002673834f32, 1.3527042f32, -0.023313284f32)),
        );
    parameters
        .set_node(
            3u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(0.0026738318f32, 4.352704f32, -0.02331328f32)),
        );
}
