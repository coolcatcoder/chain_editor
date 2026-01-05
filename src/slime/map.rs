use bevy::prelude::*;
pub fn spawn(mut parameters: crate::SpawnParameters) {
    parameters
        .set_node(
            0u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(-0.0051037073f32, 0.3236838f32, -0.01725471f32)),
        );
    parameters
        .set_node(
            2u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(-0.005103707f32, 0.3846648f32, -0.01725471f32)),
        );
    parameters
        .set_node(
            3u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(-0.0064967144f32, 0.44553572f32, -0.014930751f32)),
        );
    parameters
        .set_node(
            4u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(-0.009097198f32, 0.50613225f32, -0.0107427025f32)),
        );
    parameters
        .set_node(
            5u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(-0.0034906042f32, 0.56448954f32, -0.018893339f32)),
        );
    parameters
        .set_node(
            6u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(-0.008328577f32, 0.6237203f32, -0.0108374115f32)),
        );
    parameters
        .set_node(
            7u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(-0.0034480959f32, 0.68600595f32, -0.009373997f32)),
        );
    parameters
        .set_node(
            8u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(0.0048372257f32, 0.74592334f32, -0.014453673f32)),
        );
    parameters
        .set_node(
            9u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(-0.003956256f32, 0.8059047f32, -0.014800198f32)),
        );
    parameters
        .set_node(
            10u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(0.0027146887f32, 0.8658582f32, -0.013931742f32)),
        );
    parameters
        .set_node(
            11u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(0.003244504f32, 0.92683214f32, -0.014301384f32)),
        );
    parameters
        .set_node(
            12u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(0.0038040814f32, 0.9877142f32, -0.017212873f32)),
        );
    parameters
        .set_node(
            13u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(0.0012255321f32, 1.046557f32, -0.02392215f32)),
        );
    parameters
        .set_node(
            14u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(0.0015444577f32, 1.1075215f32, -0.024890898f32)),
        );
    parameters
        .set_node(
            15u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(-0.0006766469f32, 1.1676866f32, -0.0178699f32)),
        );
    parameters
        .set_node(
            16u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(0.0019372861f32, 1.2264192f32, -0.02456631f32)),
        );
}
