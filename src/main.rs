use bevy::{
    input::mouse::MouseMotion,
    pbr::{DirectionalLightShadowMap, NotShadowCaster, NotShadowReceiver, PointLightShadowMap},
    prelude::*,
    render::mesh::VertexAttributeValues,
    scene::InstanceId,
    utils::HashSet,
};

fn main() {
    App::new()
        .insert_resource(PointLightShadowMap {
            size: 2_usize.pow(11),
        })
        .insert_resource(DirectionalLightShadowMap {
            size: 2_usize.pow(13),
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(info)
        .add_system(night_and_day)
        .add_system(scene_update)
        .add_system(input)
        .add_system(camera_controller)
        .run();
}

fn info() {
    info!("Welcome to Bevy demo with the Bistro Scene");
    info!("Controls:");
    info!("  spacebar - enable shadows");
    info!("  1 - enable / disable the ceiling lights");
    info!("  2 - enable / disable the wall lights");
    info!("  3 - enable / disable the lanterns");
    info!("  4 - enable / disable the streetlights");
    info!("  i - get informations on the lights");
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
) {
    let exterior = scene_spawner.spawn(asset_server.load("BistroExterior.glb#Scene0"));
    let interior = scene_spawner.spawn(asset_server.load("BistroInterior_Wine.glb#Scene0"));

    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-16., 6., 1.0)
                .looking_at(Vec3::new(0.0, 1., 0.0), Vec3::Y),
            ..Default::default()
        })
        .insert(CameraController::default());

    commands.insert_resource(Scenes {
        interior: Some(interior),
        exterior: Some(exterior),
    });

    commands
        .spawn_bundle(DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Sun);
}

struct Scenes {
    interior: Option<InstanceId>,
    exterior: Option<InstanceId>,
}

#[derive(Component)]
struct Sun;
#[derive(Component)]
struct Lantern;
#[derive(Component)]
struct StreetLight;
#[derive(Component)]
struct Ceiling;
#[derive(Component)]
struct Wall;

struct LightSettings {
    ceiling: f32,
    lantern: f32,
    streetlight: f32,
    range_ratio: f32,
}

const LIGHT_SETTINGS: LightSettings = LightSettings {
    ceiling: 1000.0,
    lantern: 200.0,
    streetlight: 800.0,
    range_ratio: 50.0,
};

// This system will fix the scene by removing a few items, changing transparency on materials and adding point lights
// This should be done in Blender by modifying the scenes that are to be imported, but here I am doing it in Bevy to
// work on the unmodified scenes from nvidia
fn scene_update(
    mut commands: Commands,
    scene_spawner: Res<SceneSpawner>,
    mut scene_instance: ResMut<Scenes>,
    named_entities: Query<(Entity, &Name, &Children)>,
    has_material: Query<&Handle<StandardMaterial>>,
    has_mesh: Query<&Handle<Mesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Some(instance_id) = scene_instance.interior {
        if let Some(entity_iter) = scene_spawner.iter_instance_entities(instance_id) {
            entity_iter.for_each(|entity| {
                if let Ok((entity, name, children)) = named_entities.get(entity) {
                    if name.starts_with("Bistro_Research_Interior_Paris_Ceiling_Light") {
                        // One of the interior ceiling light:
                        // - Spawn a point light
                        // - Make the mesh not casting shadows
                        let child = children[0];
                        commands.entity(child).insert(NotShadowCaster);
                        // For those lights, they are not transformed to their place, but the mesh is moved.
                        // We find the center of the mesh, which is where the light should be.
                        if let Ok(mesh_handle) = has_mesh.get(child) {
                            if let Some(mesh) = meshes.get(mesh_handle) {
                                if let Some(VertexAttributeValues::Float32x3(attr)) =
                                    mesh.attribute(Mesh::ATTRIBUTE_POSITION)
                                {
                                    let sum =
                                        attr.iter().fold(Vec3::ZERO, |acc, v| acc + Vec3::from(*v));
                                    let center = sum / attr.iter().count() as f32 * 0.016;
                                    commands
                                        .spawn_bundle(PointLightBundle {
                                            transform: Transform::from_translation(center)
                                                .with_scale(Vec3::splat(0.16)),
                                            point_light: PointLight {
                                                color: Color::rgb(1.0, 0.9, 0.4),
                                                intensity: LIGHT_SETTINGS.ceiling,
                                                range: LIGHT_SETTINGS.ceiling
                                                    / LIGHT_SETTINGS.range_ratio,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(Ceiling);
                                }
                            }
                        }
                    }
                    if name.starts_with("Bistro_Research_Interior_Paris_Wall_Light_Interior") {
                        // One of the interior wall light:
                        // - Spawn a point light
                        // - Make the mesh not casting shadows
                        let child = children[0];
                        commands.entity(child).insert(NotShadowCaster);
                        // For those lights, they are not transformed to their place, but the mesh is moved.
                        // We find the center of the mesh, which is where the light should be.
                        if let Ok(mesh_handle) = has_mesh.get(child) {
                            if let Some(mesh) = meshes.get(mesh_handle) {
                                if let Some(VertexAttributeValues::Float32x3(attr)) =
                                    mesh.attribute(Mesh::ATTRIBUTE_POSITION)
                                {
                                    let sum =
                                        attr.iter().fold(Vec3::ZERO, |acc, v| acc + Vec3::from(*v));
                                    let center = sum / attr.iter().count() as f32 * 0.016;
                                    commands
                                        .spawn_bundle(PointLightBundle {
                                            transform: Transform::from_translation(center)
                                                .with_scale(Vec3::splat(0.16)),
                                            point_light: PointLight {
                                                color: Color::rgb(1.0, 0.9, 0.4),
                                                intensity: LIGHT_SETTINGS.ceiling,
                                                range: LIGHT_SETTINGS.ceiling
                                                    / LIGHT_SETTINGS.range_ratio,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(Wall);
                                }
                            }
                        }
                    } // This are exterior elements from the interior scene, remove them
                    if name.contains("Exterior") {
                        commands.entity(entity).despawn_recursive();
                    }
                    if name.ends_with("WineGlass")
                        || name.ends_with("WineGlass2.008")
                        || name.ends_with("WineGlass4.008")
                    {
                        commands.entity(entity).despawn_recursive();
                    }
                }
            });
            for (_, mut material) in materials.iter_mut() {
                material.flip_normal_map_y = true;
            }
            scene_instance.interior = None;
        }
    }
    if let Some(instance_id) = scene_instance.exterior {
        let sphere = meshes.add(Mesh::from(shape::UVSphere {
            radius: 5.0,
            ..Default::default()
        }));
        let material = materials.add(StandardMaterial {
            base_color: Color::YELLOW,
            unlit: true,
            ..Default::default()
        });
        let mut materials_to_fix = HashSet::default();
        if let Some(entity_iter) = scene_spawner.iter_instance_entities(instance_id) {
            entity_iter.for_each(|entity| {
                if let Ok((entity, name, children)) = named_entities.get(entity) {
                    if name.starts_with("Lantern_Wind") {
                        // One of the lantern:
                        // - Spawn a point light
                        // - Make the mesh not casting shadows
                        // - Make the material transparent
                        // - Spawn a "lightbulb"
                        commands.entity(entity).with_children(|lantern| {
                            lantern
                                .spawn_bundle(PbrBundle {
                                    mesh: sphere.clone(),
                                    material: material.clone(),
                                    transform: Transform::from_xyz(0.0, -80.0, 0.0),
                                    ..Default::default()
                                })
                                .insert_bundle((NotShadowCaster, NotShadowReceiver));
                            lantern
                                .spawn_bundle(PointLightBundle {
                                    transform: Transform::from_xyz(0.0, -80.0, 0.0),
                                    point_light: PointLight {
                                        color: Color::rgb(1.0, 0.9, 0.5),
                                        intensity: LIGHT_SETTINGS.lantern,
                                        range: LIGHT_SETTINGS.lantern / LIGHT_SETTINGS.range_ratio,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(Lantern);
                        });
                        for child in children.iter() {
                            commands.entity(*child).insert(NotShadowCaster);
                            if let Ok(material) = has_material.get(*child) {
                                materials_to_fix.insert(material);
                            }
                        }
                    }
                    if name.starts_with("Bistro_Research_Exterior_Paris_StreetLight") {
                        let child = children[0];
                        commands.entity(child).insert(NotShadowCaster);
                    }
                    if name.starts_with("Bistro_Research_Exterior_Paris_Streetlight_Glass")
                        || name.starts_with("Bistro_Research_Exterior_Paris_StreetLight_Glass")
                    {
                        // One of the streetlights:
                        // - Spawn a point light
                        // - Make the mesh not casting shadows
                        // - Make the material transparent
                        let child = children[0];
                        commands.entity(child).insert(NotShadowCaster);
                        if let Ok(material) = has_material.get(child) {
                            materials_to_fix.insert(material);
                        }
                        // For those lights, they are not transformed to their place, but the mesh is moved.
                        // We find the center of the mesh, which is where the light should be.
                        if let Ok(mesh_handle) = has_mesh.get(child) {
                            if let Some(mesh) = meshes.get(mesh_handle) {
                                if let Some(VertexAttributeValues::Float32x3(attr)) =
                                    mesh.attribute(Mesh::ATTRIBUTE_POSITION)
                                {
                                    let sum =
                                        attr.iter().fold(Vec3::ZERO, |acc, v| acc + Vec3::from(*v));
                                    let center = sum / attr.iter().count() as f32 * 0.016;
                                    commands
                                        .spawn_bundle(PointLightBundle {
                                            transform: Transform::from_translation(center)
                                                .with_scale(Vec3::splat(0.16)),
                                            point_light: PointLight {
                                                color: Color::rgb(1.0, 0.9, 0.65),
                                                intensity: LIGHT_SETTINGS.streetlight,
                                                range: LIGHT_SETTINGS.streetlight
                                                    / LIGHT_SETTINGS.range_ratio,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(StreetLight);
                                }
                            }
                        }
                    }
                    if *name
                        == Name::new(
                            "Bistro_Research_Exterior_Paris_Building_01_paris_buildi_19bd23d",
                        )
                    {
                        // This is the glass of the front door, make it transparent
                        let child = children[0];
                        commands.entity(child).insert(NotShadowCaster);
                        if let Ok(material) = has_material.get(child) {
                            materials_to_fix.insert(material);
                        }
                    }
                }
            });
            for material in materials_to_fix.drain() {
                let material = materials.get_mut(material).unwrap();
                if material.alpha_mode == AlphaMode::Opaque {
                    material.base_color.set_a(0.2);
                    material.alpha_mode = AlphaMode::Blend;
                }
            }
            for (_, mut material) in materials.iter_mut() {
                material.flip_normal_map_y = true;
            }
            scene_instance.exterior = None;
        }
    }
}

fn night_and_day(
    time: Res<Time>,
    mut sun: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
    mut ambient: ResMut<AmbientLight>,
) {
    let (mut transform, mut light) = sun.single_mut();
    transform.rotation = Quat::from_euler(
        EulerRot::ZYX,
        time.seconds_since_startup() as f32 * std::f32::consts::TAU / 20.0,
        0.0,
        -std::f32::consts::FRAC_PI_4,
    );
    let (angle, _, _) = transform.rotation.to_euler(EulerRot::XYZ);
    light.illuminance = (-angle - 0.1).max(0.0) * 142000.0;
    ambient.brightness = (light.illuminance / 400000.0).max(0.01);
}

fn input(
    input: Res<Input<KeyCode>>,
    mut lights: Query<(
        &mut PointLight,
        Option<&Ceiling>,
        Option<&Wall>,
        Option<&Lantern>,
        Option<&StreetLight>,
    )>,
    mut shadow_enabled: Local<bool>,
    camera: Query<&Transform, With<Camera>>,
) {
    if input.just_pressed(KeyCode::Space) {
        *shadow_enabled = !*shadow_enabled;
        for (mut light, ..) in lights.iter_mut() {
            light.shadows_enabled = *shadow_enabled;
        }
    }
    if input.just_pressed(KeyCode::Key1) {
        info!("toggling Ceiling");
        for (mut light, ceiling, _, _, _) in lights.iter_mut() {
            if ceiling.is_some() {
                if light.intensity == 0.0 {
                    light.intensity = LIGHT_SETTINGS.ceiling;
                } else {
                    light.intensity = 0.0;
                }
                light.range = light.intensity / LIGHT_SETTINGS.range_ratio;
                light.shadows_enabled = *shadow_enabled;
            }
        }
    }
    if input.just_pressed(KeyCode::Key2) {
        info!("toggling Wall");
        for (mut light, _, wall, _, _) in lights.iter_mut() {
            if wall.is_some() {
                if light.intensity == 0.0 {
                    light.intensity = LIGHT_SETTINGS.ceiling;
                } else {
                    light.intensity = 0.0;
                }
                light.range = light.intensity / LIGHT_SETTINGS.range_ratio;
                light.shadows_enabled = *shadow_enabled;
            }
        }
    }
    if input.just_pressed(KeyCode::Key3) {
        info!("toggling Lantern");
        for (mut light, _, _, lantern, _) in lights.iter_mut() {
            if lantern.is_some() {
                if light.intensity == 0.0 {
                    light.intensity = LIGHT_SETTINGS.lantern;
                } else {
                    light.intensity = 0.0;
                }
                light.range = light.intensity / LIGHT_SETTINGS.range_ratio;
                light.shadows_enabled = *shadow_enabled;
            }
        }
    }
    if input.just_pressed(KeyCode::Key4) {
        info!("toggling Streetlight");
        for (mut light, _, _, _, street) in lights.iter_mut() {
            if street.is_some() {
                if light.intensity == 0.0 {
                    light.intensity = LIGHT_SETTINGS.streetlight;
                } else {
                    light.intensity = 0.0;
                }
                light.range = light.intensity / LIGHT_SETTINGS.range_ratio;
                light.shadows_enabled = *shadow_enabled;
            }
        }
    }
    if input.just_pressed(KeyCode::I) {
        let count = lights.iter().count();
        info!("There are {count} lights");
        for (light, ceiling, wall, lantern, street) in lights.iter() {
            match (ceiling, wall, lantern, street) {
                (Some(_), None, None, None) => info!(
                    "Ceiling light | status: {} - shadows: {}",
                    light.intensity != 0.0,
                    light.shadows_enabled
                ),
                (None, Some(_), None, None) => info!(
                    "Wall light | status: {} - shadows: {}",
                    light.intensity != 0.0,
                    light.shadows_enabled
                ),
                (None, None, Some(_), None) => info!(
                    "Lantern | status: {} - shadows: {}",
                    light.intensity != 0.0,
                    light.shadows_enabled
                ),
                (None, None, None, Some(_)) => info!(
                    "Street light | status: {} - shadows: {}",
                    light.intensity != 0.0,
                    light.shadows_enabled
                ),
                _ => unreachable!(),
            }
        }
        for transform in camera.iter() {
            info!("{:?}", transform);
        }
    }
}

#[derive(Component)]
struct CameraController {
    pub enabled: bool,
    pub initialized: bool,
    pub sensitivity: f32,
    pub key_forward: KeyCode,
    pub key_back: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub key_run: KeyCode,
    pub key_enable_mouse: MouseButton,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub friction: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub velocity: Vec3,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            enabled: true,
            initialized: false,
            sensitivity: 0.5,
            key_forward: KeyCode::W,
            key_back: KeyCode::S,
            key_left: KeyCode::A,
            key_right: KeyCode::D,
            key_up: KeyCode::E,
            key_down: KeyCode::Q,
            key_run: KeyCode::LShift,
            key_enable_mouse: MouseButton::Left,
            walk_speed: 5.0,
            run_speed: 15.0,
            friction: 0.5,
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::ZERO,
        }
    }
}

fn camera_controller(
    time: Res<Time>,
    mut mouse_events: EventReader<MouseMotion>,
    mouse_button_input: Res<Input<MouseButton>>,
    key_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
) {
    let dt = time.delta_seconds();

    if let Ok((mut transform, mut options)) = query.get_single_mut() {
        if !options.initialized {
            let (_roll, yaw, pitch) = transform.rotation.to_euler(EulerRot::ZYX);
            options.yaw = yaw;
            options.pitch = pitch;
            options.initialized = true;
        }
        if !options.enabled {
            return;
        }

        // Handle key input
        let mut axis_input = Vec3::ZERO;
        if key_input.pressed(options.key_forward) {
            axis_input.z += 1.0;
        }
        if key_input.pressed(options.key_back) {
            axis_input.z -= 1.0;
        }
        if key_input.pressed(options.key_right) {
            axis_input.x += 1.0;
        }
        if key_input.pressed(options.key_left) {
            axis_input.x -= 1.0;
        }
        if key_input.pressed(options.key_up) {
            axis_input.y += 1.0;
        }
        if key_input.pressed(options.key_down) {
            axis_input.y -= 1.0;
        }

        // Apply movement update
        if axis_input != Vec3::ZERO {
            let max_speed = if key_input.pressed(options.key_run) {
                options.run_speed
            } else {
                options.walk_speed
            };
            options.velocity = axis_input.normalize() * max_speed;
        } else {
            let friction = options.friction.clamp(0.0, 1.0);
            options.velocity *= 1.0 - friction;
            if options.velocity.length_squared() < 1e-6 {
                options.velocity = Vec3::ZERO;
            }
        }
        let forward = transform.forward();
        let right = transform.right();
        transform.translation += options.velocity.x * dt * right
            + options.velocity.y * dt * Vec3::Y
            + options.velocity.z * dt * forward;

        // Handle mouse input
        let mut mouse_delta = Vec2::ZERO;
        if mouse_button_input.pressed(options.key_enable_mouse) {
            for mouse_event in mouse_events.iter() {
                mouse_delta += mouse_event.delta;
            }
        }

        if mouse_delta != Vec2::ZERO {
            // Apply look update
            let (pitch, yaw) = (
                (options.pitch - mouse_delta.y * 0.5 * options.sensitivity * dt).clamp(
                    -0.99 * std::f32::consts::FRAC_PI_2,
                    0.99 * std::f32::consts::FRAC_PI_2,
                ),
                options.yaw - mouse_delta.x * options.sensitivity * dt,
            );
            transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, yaw, pitch);
            options.pitch = pitch;
            options.yaw = yaw;
        }
    }
}
