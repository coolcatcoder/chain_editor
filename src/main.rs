use std::{collections::HashMap, f32::consts::FRAC_PI_2, fs::write};
use avian3d::prelude::*;
use bevy::{
    camera::RenderTarget, color::palettes::css::RED, ecs::entity::EntityHashMap, feathers::{FeathersPlugins, controls::{ButtonProps, button, radio}, dark_theme::create_dark_theme, theme::{ThemeBackgroundColor, ThemedText, UiTheme}, tokens}, input::{
        ButtonState,
        keyboard::KeyboardInput,
        mouse::{MouseButtonInput, MouseMotion},
    }, input_focus::tab_navigation::TabGroup, prelude::{Node as UiNode, *}, ui::Checked, ui_widgets::{Activate, RadioGroup, observe}, window::{CursorGrabMode, CursorOptions, WindowRef}
};
use prelude::*;
use proc_macro2::TokenStream;
use quote::quote;

mod slime;

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
    app.add_plugins((MeshPickingPlugin, FeathersPlugins))
        .add_systems(Startup, start)
        .add_systems(Update, (render_node, controls, mouse, selected))
        .init_resource::<Materials>()
        .init_resource::<Selected>()
        .insert_resource(UiTheme(create_dark_theme()))
        .add_observer(join);
}

fn start(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.insert_resource(SphereMesh(meshes.add(Sphere::new(0.03))));
    // let material = materials.add(StandardMaterial {
    //     ..default()
    // });

    let window = commands
        .spawn((Window::default(), EditorWindow))
        .observe(save)
        .id();
    let camera = commands.spawn((
        Camera3d::default(),
        Transform::default(),
        Camera {
            target: RenderTarget::Window(WindowRef::Entity(window)),
            ..default()
        },
        EditorCamera,
    )).id();

    commands.spawn((ui(), UiTargetCamera(camera)));
}

#[derive(Component)]
struct EditorWindow;

#[derive(Component)]
struct EditorCamera;

#[derive(Component, Debug)]
struct Node {
    mesh: NodeMesh,
    // Will not be accurate for shapes that are not perfect spheres.
    radius: f32,
}

fn render_node(
    nodes: Query<(Entity, &Node), Changed<Node>>,
    sphere_mesh: Res<SphereMesh>,
    mut materials: ResMut<Materials>,
    mut commands: Commands,
) {
    for (entity, node) in nodes {
        commands.entity(entity).remove::<(SceneRoot, Mesh3d)>();

        match &node.mesh {
            NodeMesh::Sphere(material) => {
                commands.entity(entity).insert((
                    Mesh3d(sphere_mesh.0.clone()),
                    MeshMaterial3d(material.clone()),
                ));
                materials.0.insert(material.clone());
            }
            NodeMesh::FromGltf(path) => {
                todo!();
            }
        }
    }
}

#[derive(Debug)]
enum NodeMesh {
    Sphere(Handle<StandardMaterial>),
    FromGltf(&'static str),
}

struct Join(usize, usize);

#[derive(Resource)]
struct SphereMesh(Handle<Mesh>);

#[derive(Resource, Default)]
struct Materials(std::collections::HashSet<Handle<StandardMaterial>>);

#[derive(Resource, Default)]
struct Selected(Option<Entity>);

fn selected(selected: Res<Selected>, node: Query<(&Node, &Transform)>, mut gizmos: Gizmos) {
    let Some(selected) = selected.0 else {
        return;
    };
    let Ok((node, transform)) = node.get(selected) else {
        error!("Failed to get selected node");
        return;
    };

    gizmos.sphere(transform.translation, node.radius + 0.01, RED);
}

fn join(
    on: On<Pointer<Click>>,
    transform_rigid_body_and_has_mesh: Query<(&GlobalTransform, Has<RigidBody>), With<Mesh3d>>,
    materials: Res<Materials>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
    mut selected: ResMut<Selected>,
    mut commands: Commands,
) {
    // Surprisingly the window and its ilk can send Pointer<Click> events.
    // Checking for the mesh while getting the transform filters them out.
    let Ok((transform, has_rigid_body)) = transform_rigid_body_and_has_mesh.get(on.entity) else {
        return;
    };

    match on.button {
        PointerButton::Primary => (),
        PointerButton::Secondary => return,
        PointerButton::Middle => {
            selected.0 = Some(on.entity);
            return;
        }
    }

    // Joints only work if both entities have rigid bodies.
    if !has_rigid_body {
        commands.entity(on.entity).insert(RigidBody::Static);
    }

    let hit_position = on.hit.position.unwrap();

    let material = materials
        .0
        .iter()
        .next()
        .cloned()
        .unwrap_or_else(|| material_assets.add(StandardMaterial::default()));
    let new_entity = commands
        .spawn((
            Transform::from_translation(hit_position),
            Node {
                mesh: NodeMesh::Sphere(material),
                radius: 0.03,
            },
            LockedAxes::ROTATION_LOCKED,
            Mass(1.),
            AngularInertia::from_shape(&Collider::sphere(0.03), 1.),
            RigidBody::Dynamic,
            GravityScale(-1.),
            //SleepingDisabled,
        ))
        .id();

    commands.spawn(
        DistanceJoint::new(on.entity, new_entity)
            .with_limits(0., 0.03)
            .with_local_anchor1(hit_position - transform.translation()),
    );

    info!("Spawned.");
}

fn mouse(
    mut window: Single<(&Window, &mut CursorOptions), With<EditorWindow>>,
    mut camera: Single<&mut Transform, With<EditorCamera>>,
    mut mouse_motion: MessageReader<MouseMotion>,
    mut mouse_button_input: MessageReader<MouseButtonInput>,
    mut pressed: Local<bool>,
    mut ui_visibility: Single<&mut Visibility, With<Ui>>,
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
    let nodes = nodes
        .iter()
        .map(|(entity, node, transform)| serialise.node(entity, node, transform.translation));

    let token_stream = quote! {
        use bevy::prelude::*;

        pub fn spawn(asset_server: &AssetServer, commands: &mut Commands) {
            let mut deserialise = crate::Deserialise::new(asset_server, commands);
            #(
                let node = #nodes;
                deserialise.set_node(node);
            )*
        }

    };

    write(
        "/home/coolcatcoder/Documents/GitHub/chain_editor/src/slime/map.rs",
        token_stream.to_string(),
    )
    .unwrap();
}

#[derive(Default)]
struct Serialise {
    entity: EntityHashMap<u32>,
    material: HashMap<AssetId<StandardMaterial>, u32>,
    next: u32,
}

impl Serialise {
    fn next(&mut self) -> u32 {
        let next = self.next;
        self.next += 1;
        next
    }

    fn node(&mut self, entity: Entity, node: &Node, translation: Vec3) -> TokenStream {
        let next = self.next();
        let entity = *self.entity.entry(entity).or_insert(next);

        let translation = translation.to_array();

        let mesh = match &node.mesh {
            NodeMesh::Sphere(material) => {
                let next = self.next();
                let material = self.material.entry(material.id()).or_insert(next);

                quote! {
                    crate::NodeMesh::Sphere(deserialise.get_material(#material))
                }
            }
            NodeMesh::FromGltf(path) => {
                quote! {
                    crate::NodeMesh::FromGltf(#path)
                }
            }
        };

        quote! {
            (
            #entity,
            crate::Node {
                mesh: #mesh,
            },
            Vec3::new(#(#translation),*),
            )
        }
    }
}

struct Deserialise<'a, 'c, 'w, 's> {
    asset_server: &'a AssetServer,
    commands: &'c mut Commands<'w, 's>,

    entity: HashMap<u32, Entity>,
    material: HashMap<u32, Handle<StandardMaterial>>,
}

impl<'a, 'c, 'w, 's> Deserialise<'a, 'c, 'w, 's> {
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

    fn set_node(&mut self, (key, node, translation): (u32, Node, Vec3)) {
        let entity = self
            .commands
            .spawn((node, Transform::from_translation(translation)))
            .id();
        if self.entity.insert(key, entity).is_some() {
            panic!("Double set has occurred!");
        }
    }
}

#[derive(Component)]
struct Ui;

fn ui() -> impl Bundle {
    (
        Ui,
        UiNode {
            width: percent(30),
            height: percent(100),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: px(10),
            ..default()
        },
        TabGroup::default(),
        ThemeBackgroundColor(tokens::WINDOW_BG),
        children![(
            UiNode {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                justify_content: JustifyContent::Start,
                padding: UiRect::all(px(8)),
                row_gap: px(8),
                width: percent(100),
                min_width: px(200),
                ..default()
            },
            children![
                (
                    UiNode {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Start,
                        column_gap: px(8),
                        ..default()
                    },
                    children![
                        (
                            button(
                                ButtonProps::default(),
                                (),
                                Spawn((Text::new("Normal"), ThemedText))
                            ),
                            observe(|_activate: On<Activate>| {
                                info!("Normal button clicked!");
                            })
                        ),
                        (
                            RadioGroup,
                            Visibility::Inherited,
                            children![
                                radio(Checked, Spawn((Text::new("One"), ThemedText))),
                                radio((), Spawn((Text::new("Two"), ThemedText))),
                                radio((), Spawn((Text::new("Three"), ThemedText))),
                            ]
                        )
                    ]
                ),
            ]
        ),],
    )
}
