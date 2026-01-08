use bevy::prelude::*;
pub fn spawn(mut parameters: crate::SpawnParameters) {
    parameters
        .set_node(
            0u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(0.0057866573f32, 0.32701734f32, 0.0064867586f32)),
        );
    parameters
        .set_node(
            2u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(-0.01250705f32, 0.31281292f32, -0.04859745f32)),
        );
    parameters
        .set_node(
            3u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(0.0038794589f32, 0.35911578f32, -0.067878835f32)),
        );
    parameters
        .set_node(
            4u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(0.020620622f32, 0.40739465f32, -0.08477401f32)),
        );
    parameters
        .set_node(
            5u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(0.030557737f32, 0.45023948f32, -0.109918065f32)),
        );
    parameters
        .set_node(
            6u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(0.043138362f32, 0.48992246f32, -0.13532142f32)),
        );
    parameters
        .set_node(
            7u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(0.06929875f32, 0.5152485f32, -0.14889584f32)),
        );
    parameters
        .set_node(
            8u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(0.08571911f32, 0.56893855f32, -0.14121868f32)),
        );
    parameters
        .set_node(
            9u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(-0.378732f32, 0.030399157f32, 0.94448394f32)),
        );
    parameters
        .set_node(
            10u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(-0.37873268f32, 0.09073181f32, 0.9444845f32)),
        );
    parameters
        .set_node(
            11u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(-0.37873346f32, 0.15099792f32, 0.94448525f32)),
        );
    parameters
        .set_node(
            12u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(-0.3787344f32, 0.2111975f32, 0.944486f32)),
        );
    parameters
        .set_node(
            13u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(-0.3787369f32, 0.34133056f32, 0.9444881f32)),
        );
    parameters
        .set_node(
            14u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(-0.37874168f32, 0.5413971f32, 0.944492f32)),
        );
    parameters
        .set_node(
            15u32,
            crate::NodeBuilder::default()
                .mesh(crate::NodeMeshSerialised::Sphere(1u32))
                .translation(Vec3::new(-0.37874535f32, 0.7413971f32, 0.9444952f32)),
        );
}
