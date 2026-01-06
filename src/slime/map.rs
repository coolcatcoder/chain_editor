use bevy::prelude::*;
pub fn spawn(mut parameters: crate::SpawnParameters) {
    parameters
        .set_node(
            0u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(0.0075898906f32, 0.3270603f32, 0.0068903523f32)),
        );
    parameters
        .set_node(
            2u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(-0.017310793f32, 0.85747045f32, 0.0043839775f32)),
        );
    parameters
        .set_node(
            3u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(0.21349843f32, 1.2651612f32, -0.23856199f32)),
        );
    parameters
        .set_node(
            4u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(0.18761933f32, 1.3204546f32, -0.26490286f32)),
        );
    parameters
        .set_node(
            5u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(0.044704456f32, 1.0466682f32, -0.4550515f32)),
        );
    parameters
        .set_node(
            6u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(-0.054475263f32, 1.4381504f32, -0.067215525f32)),
        );
    parameters
        .set_node(
            7u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(-0.35883844f32, 1.161159f32, -0.27277297f32)),
        );
    parameters
        .set_node(
            8u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(-0.21612118f32, 0.95210606f32, -0.46449f32)),
        );
    parameters
        .set_node(
            9u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(0.42458472f32, 1.0709947f32, -0.08300074f32)),
        );
    parameters
        .set_node(
            10u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(-0.032187566f32, 1.3731585f32, -0.2523614f32)),
        );
    parameters
        .set_node(
            11u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(0.29555514f32, 0.9651181f32, -0.38283068f32)),
        );
    parameters
        .set_node(
            12u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(0.26856753f32, 1.3384591f32, 0.009253221f32)),
        );
    parameters
        .set_node(
            13u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(-0.21762113f32, 0.8616425f32, -0.44728655f32)),
        );
    parameters
        .set_node(
            14u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(0.14168382f32, 0.8662398f32, -0.47238436f32)),
        );
    parameters
        .set_node(
            15u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(-0.08860277f32, 1.2608554f32, -0.3896826f32)),
        );
    parameters
        .set_node(
            16u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(0.23517583f32, 1.2485181f32, -0.27850637f32)),
        );
}
