#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use avian3d::prelude::*;
use bevy::{
    camera::RenderTarget,
    color::palettes::css::RED,
    ecs::{entity::EntityHashMap, lifecycle::HookContext, world::DeferredWorld},
    feathers::{FeathersPlugins, dark_theme::create_dark_theme, theme::UiTheme},
    input::{
        ButtonState,
        keyboard::KeyboardInput,
        mouse::{MouseButtonInput, MouseMotion},
    },
    input_focus::InputDispatchPlugin,
    prelude::*,
    ui_widgets::Activate,
    window::{CursorGrabMode, CursorOptions, WindowRef},
};
use bevy_ui_text_input::TextInputPlugin;
use prelude::*;
use quote::quote;
use std::{collections::HashMap, f32::consts::FRAC_PI_2, fs::write};

use crate::ui::{TextInputModified, UiBuilder};

mod slime;
mod ui;

mod prelude {
    use bevy::prelude::*;

    #[allow(non_upper_case_globals)]
    pub const Vec3: fn(f32, f32, f32) -> Vec3 = |x, y, z| Vec3::new(x, y, z);
    #[allow(non_upper_case_globals)]
    pub const Vec2: fn(f32, f32) -> Vec2 = |x, y| Vec2::new(x, y);
}

fn main() {
    slime::main();
}

fn plugin(app: &mut App) {
    app.add_plugins((
        MeshPickingPlugin,
        FeathersPlugins.build().disable::<InputDispatchPlugin>(),
        TextInputPlugin,
        TextInputModified::plugin,
    ))
    .add_systems(Startup, start)
    .add_systems(
        Update,
        (
            create_node,
            controls,
            mouse,
            selected,
            edit_node,
            change_link,
            link_gizmos,
        ),
    )
    .init_resource::<Materials>()
    .init_resource::<Selected>()
    .insert_resource(UiTheme(create_dark_theme()))
    .insert_resource(Brush { radius: 0.03 })
    .add_observer(on_click)
    .init_gizmo_group::<LinkGizmos>()
    .insert_gizmo_config(LinkGizmos, GizmoConfig { depth_bias: -1., ..default() });
}

fn start(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut ui_builder: UiBuilder) {
    commands.insert_resource(SphereMesh(meshes.add(Sphere::new(1.))));
    // let material = materials.add(StandardMaterial {
    //     ..default()
    // });

    let window = commands
        .spawn((Window::default(), EditorWindow))
        .observe(save)
        .id();
    let camera = commands
        .spawn((
            Camera3d::default(),
            Transform::default(),
            Camera {
                target: RenderTarget::Window(WindowRef::Entity(window)),
                ..default()
            },
            EditorCamera,
        ))
        .id();

    //commands.spawn((ui(), UiTargetCamera(camera)));
    let mut ui = ui_builder.on_camera(camera);

    ui.text("Join Mode");
    ui.radio_buttons([
        ("Join at centre.", JoinMode::AtCentre),
        ("Join at position.", JoinMode::AtPosition),
    ]);
    ui.text("Brush");
    ui.text("Radius")
        .numerical_input(|brush: &mut Brush, radius| brush.radius = radius);
    ui.checkbox("Gravity", true);
}

#[derive(Component)]
struct EditorWindow;

#[derive(Component)]
struct EditorCamera;

enum LocalAnchor {
    MultiplyByRadius(Vec3),
    Fixed,
}

#[derive(Component, Clone)]
#[component(on_add = Self::on_add)]
struct Link {
    entity_one: Entity,
    local_anchor_one: Vec3,
    entity_two: Entity,
    local_anchor_two: Vec3,
}

fn change_link(links: Query<(&Link, &mut DistanceJoint), Changed<Link>>, nodes: Query<&Node>) {
    for (link, mut distance_joint) in links {
        info!("Changed link. ({:?}, {:?})", link.entity_one, link.entity_two);

        let mut local_anchor_one = link.local_anchor_one;
        let mut local_anchor_two = link.local_anchor_two;
        let mut max_distance = 0.;

        if let Ok(node) = nodes.get(link.entity_one) {
            if link.local_anchor_one == Vec3::ZERO {
                max_distance += node.radius;
            } else {
                local_anchor_one *= node.radius;
            }
        }
        if let Ok(node) = nodes.get(link.entity_two) {
            if link.local_anchor_two == Vec3::ZERO {
                max_distance += node.radius;
            } else {
                local_anchor_two *= node.radius;
            }
        }

        *distance_joint = DistanceJoint::new(link.entity_one, link.entity_two)
            .with_local_anchor1(local_anchor_one)
            .with_local_anchor2(local_anchor_two)
            .with_limits(0., max_distance);
    }
}

#[derive(GizmoConfigGroup, Default, Reflect)]
struct LinkGizmos;
fn link_gizmos(links: Query<&Link>, transforms: Query<&GlobalTransform>, mut gizmos: Gizmos<LinkGizmos>) {
    for link in links {
        let translation_one = transforms.get(link.entity_one).ok().map(|transform| transform.translation() + link.local_anchor_one );
        let translation_two = transforms.get(link.entity_two).ok().map(|transform| transform.translation() + link.local_anchor_two );
        let Some((translation_one, translation_two)) = translation_one.zip(translation_two) else {
            continue;
        };

        gizmos.arrow(translation_one, translation_two, RED).with_double_end();
    }
}

impl Link {
    fn on_add(mut world: DeferredWorld, context: HookContext) {
        let link = world
            .get::<Self>(context.entity)
            .expect("Expected Link to have a Link.")
            .clone();

        let mut commands = world.commands();

        // We need to start with an incorrect, but sane default distance joint, so that we can query and modify it later.
        commands
            .entity(context.entity)
            .insert(DistanceJoint::new(link.entity_one, link.entity_two).with_compliance(1.));

        commands
            .entity(link.entity_one)
            .entry::<Links>()
            .or_default()
            .and_modify(move |mut links| links.0.push(context.entity));
        commands
            .entity(link.entity_two)
            .entry::<Links>()
            .or_default()
            .and_modify(move |mut links| links.0.push(context.entity));
    }
}

/// The entities refer to the entity with a joint/link, not another node.
#[derive(Component, Default)]
struct Links(Vec<Entity>);

#[derive(Component, Debug)]
#[require(Visibility::Visible, Links, RigidBody)]
struct Node {
    mesh: NodeMesh,
    // Will not be accurate for shapes that are not perfect spheres.
    radius: f32,
}

#[derive(Component)]
struct FromNode(Entity);

fn create_node(
    nodes: Query<(Entity, &Node, &Links), Changed<Node>>,
    mut link: Query<&mut Link>,
    sphere_mesh: Res<SphereMesh>,
    mut materials: ResMut<Materials>,
    mut commands: Commands,
) {
    for (entity, node, links) in nodes {
        info!("Updated {entity:?}.");

        for link_entity in &links.0 {
            let mut link = link.get_mut(*link_entity).expect("All links have Link.");
            link.set_changed();
        }

        commands.entity(entity).despawn_children().insert((
                LockedAxes::ROTATION_LOCKED,
                Mass(1.),
                AngularInertia::from_shape(&Collider::sphere(node.radius), 1.),
                RigidBody::Dynamic,
                GravityScale(-1.),
                SleepingDisabled,
                LinearDamping(1.),));

        match &node.mesh {
            NodeMesh::Sphere(material) => {
                commands.entity(entity).with_child((
                    Mesh3d(sphere_mesh.0.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::from_scale(Vec3::splat(node.radius)),
                    FromNode(entity),
                ));
                materials.0.insert(material.clone());
            }
            NodeMesh::FromGltf(path) => {
                todo!("Render gltf from {path}.");
            }
        }
    }
}

#[derive(Debug)]
enum NodeMesh {
    Sphere(Handle<StandardMaterial>),
    FromGltf(&'static str),
}

#[derive(Resource)]
struct SphereMesh(Handle<Mesh>);

#[derive(Resource, Default)]
struct Materials(std::collections::HashSet<Handle<StandardMaterial>>);

#[derive(Resource, Default)]
enum Selected {
    #[default]
    None,
    Node(Entity),
    /// The entity referenced is the one with the Bridge component. Not either of the two anchor nodes.
    Bridge(Entity),
}

fn selected(selected: Res<Selected>, node: Query<(&Node, &Transform)>, mut gizmos: Gizmos) {
    let Selected::Node(selected) = *selected else {
        return;
    };
    let Ok((node, transform)) = node.get(selected) else {
        error!("Failed to get selected node");
        return;
    };

    gizmos.sphere(
        transform.translation,
        node.radius + (0.01 * node.radius),
        RED,
    );
}

#[derive(Resource, Component, Clone)]
enum JoinMode {
    AtPosition,
    AtCentre,
}

#[derive(Resource)]
struct Brush {
    radius: f32,
}

fn edit_node(brush: Res<Brush>, selected: Res<Selected>, mut node: Query<&mut Node>) {
    if !brush.is_changed() {
        return;
    }
    let Selected::Node(selected) = *selected else {
        return;
    };
    let Some(mut selected) = node.get_mut(selected).ok() else {
        return;
    };

    info!("Update entity.");
    selected.radius = brush.radius;
}

fn on_click(
    on: On<Pointer<Click>>,
    target: Query<(Option<&FromNode>, &GlobalTransform), With<Mesh3d>>,
    node: Query<(Entity, &Node)>,
    has_rigid_body: Query<(), With<RigidBody>>,
    materials: Res<Materials>,
    join_mode: Res<JoinMode>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
    mut selected: ResMut<Selected>,
    brush: Res<Brush>,
    mut commands: Commands,
) {
    match on.button {
        PointerButton::Primary => {
            // Makes certain that whatever we are clicking on has a Mesh3d, and might have a node.
            let Some((node, target_transform)) = target.get(on.entity).ok().map(|(from_node, transform)| (from_node.and_then(|from_node| node.get(from_node.0).ok()), transform)) else {
                return;
            };
            let target_entity = node.map(|(entity, _)| entity).unwrap_or(on.entity);

            // Joints only work if both entities have rigid bodies.
            if has_rigid_body.get(target_entity).is_err() {
                commands.entity(target_entity).insert(RigidBody::Static);
            }
            
            *selected = Selected::None;

            let Some(hit_translation) = on.hit.position else {
                error!("Failed to get hit.position.");
                return;
            };

            let material = materials
                .0
                .iter()
                .next()
                .cloned()
                .unwrap_or_else(|| material_assets.add(StandardMaterial::default()));

            let new_entity = commands.spawn((Transform::from_translation(hit_translation),
                Node {
                    mesh: NodeMesh::Sphere(material),
                    radius: brush.radius,
                },)).id();

            match (join_mode.clone(), node) {
                (JoinMode::AtCentre, Some(node)) => {
                    info!("Joined at centre.");
                    commands.spawn(Link {
                        entity_one: node.0,
                        local_anchor_one: Vec3::ZERO,
                        entity_two: new_entity,
                        local_anchor_two: Vec3::ZERO,
                    });
                }
                (JoinMode::AtPosition, Some(node)) => {
                    info!("Joined at position.");
                    commands.spawn(Link {
                        entity_one: node.0,
                        local_anchor_one: (hit_translation - target_transform.translation()) / node.1.radius,
                        entity_two: new_entity,
                        local_anchor_two: Vec3::ZERO,
                    });
                }
                (JoinMode::AtPosition, None) | (JoinMode::AtCentre, None) => {
                    info!("Joined at position.");
                    commands.spawn(Link {
                        entity_one: on.entity,
                        local_anchor_one: hit_translation - target_transform.translation(),
                        entity_two: new_entity,
                        local_anchor_two: Vec3::ZERO,
                    });
                }
            }
        }
        PointerButton::Secondary => (),
        PointerButton::Middle => {
            error!("Middle button not implemented.");
        }
    }

    // Old:
    /*
    if matches!(on.button, PointerButton::Secondary) {
        return;
    }

    let hit_position = on.hit.position.unwrap();

    let mut new_entity = move |commands: &mut Commands, brush: &Brush| {
        let material = materials
            .0
            .iter()
            .next()
            .cloned()
            .unwrap_or_else(|| material_assets.add(StandardMaterial::default()));

        commands
            .spawn((
                Transform::from_translation(hit_position),
                Node {
                    mesh: NodeMesh::Sphere(material),
                    radius: brush.radius,
                },
                LockedAxes::ROTATION_LOCKED,
                Mass(1.),
                AngularInertia::from_shape(&Collider::sphere(brush.radius), 1.),
                RigidBody::Dynamic,
                GravityScale(-1.),
                SleepingDisabled,
                LinearDamping(1.),
            ))
            .id()
    };

    // The only valid targets, are those that come from a node, and those with a mesh.
    match from_node.get(on.entity) {
        Ok(from_node) => {
            let button = on.button;
            let _on = on;
            let target_entity = from_node.0;

            let Ok((target_transform, target_node)) = target_with_node.get(target_entity) else {
                error!("Failed to get target query from node entity.");
                return;
            };

            if matches!(button, PointerButton::Middle) {
                selected.0 = Some(target_entity);
                return;
            }

            let new_entity = new_entity(&mut commands, &brush);

            match *join_mode {
                JoinMode::AtCentre => {
                    info!("Joined at centre.");
                    commands.spawn(Link {
                        entity_one: target_entity,
                        local_anchor_one: Vec3::ZERO,
                        entity_two: new_entity,
                        local_anchor_two: Vec3::ZERO,
                    });
                }
                JoinMode::AtPosition => {
                    info!("Joined at position.");
                    // commands.spawn(
                    //     DistanceJoint::new(target_entity, new_entity)
                    //         .with_limits(0., brush.radius)
                    //         .with_local_anchor1(hit_position - target_transform.translation()),
                    // );
                    commands.spawn(Link {
                        entity_one: target_entity,
                        local_anchor_one: (hit_position - target_transform.translation()) / target_node.radius,
                        entity_two: new_entity,
                        local_anchor_two: Vec3::ZERO,
                    });
                }
            }
        }
        Err(_) => {
            let target_entity = on.entity;
            let button = on.button;
            let _on = on;

            let Ok(target_transform) = target_without_node.get(target_entity) else {
                return;
            };

            // Joints only work if both entities have rigid bodies.
            if has_rigid_body.get(target_entity).is_err() {
                commands.entity(target_entity).insert(RigidBody::Static);
            }

            if matches!(button, PointerButton::Middle) {
                selected.0 = None;
                return;
            }

            let new_entity = new_entity(&mut commands, &brush);

            info!("Joined at position.");
            commands.spawn(
                DistanceJoint::new(target_entity, new_entity)
                    .with_limits(0., brush.radius)
                    .with_local_anchor1(hit_position - target_transform.translation()),
            );
        }
    }
    */
}

fn mouse(
    mut window: Single<(&Window, &mut CursorOptions), With<EditorWindow>>,
    mut camera: Single<&mut Transform, With<EditorCamera>>,
    mut mouse_motion: MessageReader<MouseMotion>,
    mut mouse_button_input: MessageReader<MouseButtonInput>,
    mut pressed: Local<bool>,
    mut ui_visibility: Single<&mut Visibility, With<UiTargetCamera>>,
) {
    for mouse_button_input in mouse_button_input.read() {
        if !matches!(mouse_button_input.button, MouseButton::Right) {
            continue;
        }

        *pressed = match mouse_button_input.state {
            ButtonState::Pressed => true,
            ButtonState::Released => false,
        };
    }

    let in_window = window.0.physical_cursor_position().is_some();

    if !(in_window && *pressed) {
        // If we are in the window, but we aren't pressing, then we have to make sure the mouse is normal.
        if in_window {
            window.1.grab_mode = CursorGrabMode::None;
            window.1.visible = true;
            **ui_visibility = Visibility::Visible;
        }
        mouse_motion.clear();
        return;
    }

    window.1.grab_mode = CursorGrabMode::Locked;
    window.1.visible = false;
    **ui_visibility = Visibility::Hidden;

    let mut delta = Vec2::ZERO;
    for mouse_motion in mouse_motion.read() {
        delta += mouse_motion.delta;
    }

    // I really need to keep learning maths.
    // Everything below here is taken from bevy's examples.
    // One day, I will understand this. That day will be wonderful.

    if delta == Vec2::ZERO {
        return;
    }

    let sensitivity = 0.003;
    delta = -delta * sensitivity;

    let mut rotation = camera.rotation.to_euler(EulerRot::YXZ);
    rotation.0 += delta.x;
    const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
    rotation.1 = (rotation.1 + delta.y).clamp(-PITCH_LIMIT, PITCH_LIMIT);

    camera.rotation = Quat::from_euler(EulerRot::YXZ, rotation.0, rotation.1, rotation.2);
}

fn controls(
    mut keyboard_input: MessageReader<KeyboardInput>,
    time: Res<Time>,
    mut held: Local<Vec3>,
    window: Single<Entity, With<EditorWindow>>,
    mut camera: Single<&mut Transform, With<EditorCamera>>,
) {
    for keyboard_input in keyboard_input.read() {
        if keyboard_input.window != *window {
            continue;
        }

        let (index, direction) = match keyboard_input.key_code {
            KeyCode::KeyW => (2, -1.),
            KeyCode::KeyS => (2, 1.),
            KeyCode::KeyA => (0, -1.),
            KeyCode::KeyD => (0, 1.),
            KeyCode::Space => (1, 1.),
            _ => continue,
        };

        held[index] = match keyboard_input.state {
            ButtonState::Pressed => direction,
            ButtonState::Released => 0.,
        };
    }

    let rotation = camera.rotation;
    // displacement = velocity * time
    // Then make it go in the right direction.
    camera.translation += rotation * (held.normalize_or_zero() * time.delta_secs());
}

fn save(_: On<Remove, Window>, nodes: Query<(Entity, &Node, &Transform), Changed<Node>>) {
    let mut serialise = Serialise::default();

    let nodes = nodes.iter().map(|(entity, node, transform)| {
        let entity = serialise.entity(entity);
        let mesh = match &node.mesh {
            NodeMesh::Sphere(material) => {
                let handle = serialise.material(material);

                quote! {
                    crate::NodeMeshSerialised::Sphere(#handle)
                }
            }
            NodeMesh::FromGltf(path) => {
                quote! {
                    crate::NodeMeshSerialised::FromGltf(#path)
                }
            }
        };

        let [x, y, z] = transform.translation.to_array();

        quote! {
            parameters.set_node(#entity, crate::NodeBuilder::default().mesh(#mesh).translation(Vec3::new(#x, #y, #z)));
        }
    });

    let token_stream = quote! {
        use bevy::prelude::*;

        pub fn spawn(mut parameters: crate::SpawnParameters) {
            #(
                #nodes
            )*
        }

    };

    // Format the token stream using prettyplease. This makes it easier to debug the generated code.
    let file = syn::parse2::<syn::File>(token_stream).unwrap();
    let formatted_token_stream = prettyplease::unparse(&file);

    write(
        "/home/coolcatcoder/Documents/GitHub/chain_editor/src/slime/map.rs",
        formatted_token_stream,
    )
    .unwrap();
}

#[derive(Default)]
struct Serialise {
    entity: EntityHashMap<u32>,
    material: HashMap<AssetId<StandardMaterial>, u32>,
    /// The next id to be used.
    /// An id is unique across all hashmaps.
    next: u32,
}

impl Serialise {
    fn entity(&mut self, entity: Entity) -> u32 {
        let mut next = self.next;
        let entity = *self.entity.entry(entity).or_insert_with(|| {
            let previous_next = next;
            next += 1;
            previous_next
        });
        self.next = next;
        entity
    }

    fn material(&mut self, handle: &Handle<StandardMaterial>) -> u32 {
        let mut next = self.next;
        let handle = *self.material.entry(handle.id()).or_insert_with(|| {
            let previous_next = next;
            next += 1;
            previous_next
        });
        self.next = next;
        handle
    }

    // fn node(&mut self, entity: Entity, node: &Node, translation: Vec3) -> TokenStream {
    //     let next = self.next();
    //     let entity = *self.entity.entry(entity).or_insert(next);

    //     let translation = translation.to_array();

    //     let mesh = match &node.mesh {
    //         NodeMesh::Sphere(material) => {
    //             let next = self.next();
    //             let material = self.material.entry(material.id()).or_insert(next);

    //             quote! {
    //                 crate::NodeMesh::Sphere(deserialise.get_material(#material))
    //             }
    //         }
    //         NodeMesh::FromGltf(path) => {
    //             quote! {
    //                 crate::NodeMesh::FromGltf(#path)
    //             }
    //         }
    //     };

    //     quote! {
    //         (
    //         #entity,
    //         crate::Node {
    //             mesh: #mesh,
    //         },
    //         Vec3::new(#(#translation),*),
    //         )
    //     }
    // }
}

struct SpawnParameters<'a, 'c, 'w, 's> {
    asset_server: &'a AssetServer,
    commands: &'c mut Commands<'w, 's>,

    entity: HashMap<u32, Entity>,
    material: HashMap<u32, Handle<StandardMaterial>>,
}

enum NodeMeshSerialised {
    Sphere(u32),
    FromGltf(&'static str),
}

#[derive(Default)]
struct NodeBuilder {
    mesh: Option<NodeMeshSerialised>,
    radius: Option<f32>,
    translation: Option<Vec3>,
}

impl NodeBuilder {
    fn mesh(self, mesh: NodeMeshSerialised) -> Self {
        Self {
            mesh: Some(mesh),
            ..self
        }
    }
    fn translation(self, translation: Vec3) -> Self {
        Self {
            translation: Some(translation),
            ..self
        }
    }
}

impl<'a, 'c, 'w, 's> SpawnParameters<'a, 'c, 'w, 's> {
    fn new(asset_server: &'a AssetServer, commands: &'c mut Commands<'w, 's>) -> Self {
        Self {
            asset_server,
            commands,
            entity: default(),
            material: default(),
        }
    }

    fn get_material(&mut self, key: u32) -> Handle<StandardMaterial> {
        self.material
            .entry(key)
            .or_insert(self.asset_server.add(StandardMaterial::default()))
            .clone()
    }

    fn set_node(&mut self, key: u32, node_builder: NodeBuilder) {
        let mesh = node_builder
            .mesh
            .map(|mesh| match mesh {
                NodeMeshSerialised::Sphere(handle) => NodeMesh::Sphere(self.get_material(handle)),
                NodeMeshSerialised::FromGltf(path) => NodeMesh::FromGltf(path),
            })
            .unwrap_or_else(|| todo!());

        let node = Node {
            mesh,
            radius: node_builder.radius.unwrap_or(0.03),
        };

        let entity = self
            .commands
            .spawn((
                node,
                Transform::from_translation(node_builder.translation.unwrap_or(Vec3::ZERO)),
            ))
            .id();
        if self.entity.insert(key, entity).is_some() {
            panic!("Double set has occurred!");
        }
    }
}
