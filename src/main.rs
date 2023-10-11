use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (spin_camera, swap_material))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let pixel_art_material = materials.add(StandardMaterial {
        alpha_mode: AlphaMode::Mask(0.5),
        base_color_texture: Some(asset_server.load("bevy_pixel_dark.png")),
        pixel_art_anti_aliasing: true,
        cull_mode: None,
        ..default()
    });
    commands.insert_resource(PixelArtMaterial(pixel_art_material.clone()));
    commands.insert_resource(NormalMaterial(materials.add(StandardMaterial {
        alpha_mode: AlphaMode::Mask(0.5),
        base_color_texture: Some(asset_server.load("bevy_pixel_dark.png")),
        cull_mode: None,
        ..default()
    })));

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 1.5, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::rgb(0.1, 0.3, 0.5)),
                ..default()
            },
            ..default()
        },
        SpinCamera {
            offset: Vec3::new(-2.0, 1.5, 2.0),
            target: Vec3::ZERO,
            angle: 0.0,
        },
    ));

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // sprite
    commands.spawn((
        CurrentMaterial::PixelArt,
        PbrBundle {
            mesh: meshes.add(quad_mesh()),
            material: pixel_art_material,
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
    ));

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn quad_mesh() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    #[rustfmt::skip]
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![
        [-0.5,  0.5, 0.0],
        [-0.5, -0.5, 0.0],
        [ 0.5, -0.5, 0.0],
        [ 0.5,  0.5, 0.0],
    ]);

    #[rustfmt::skip]
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 4]);

    #[rustfmt::skip]
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![
        [0.0, 0.0],
        [0.0, 1.0],
        [1.0, 1.0],
        [1.0, 0.0],
    ]);

    #[rustfmt::skip]
    mesh.set_indices(Some(Indices::U32(vec![
        0, 1, 2,
        0, 2, 3,
    ])));

    return mesh;
}

#[derive(Resource, Deref)]
struct PixelArtMaterial(Handle<StandardMaterial>);
#[derive(Resource, Deref)]
struct NormalMaterial(Handle<StandardMaterial>);

#[derive(Component)]
enum CurrentMaterial {
    PixelArt,
    Normal,
}

fn swap_material(
    mut sprites: Query<(&mut CurrentMaterial, &mut Handle<StandardMaterial>)>,
    input: Res<Input<KeyCode>>,
    pixel_art_handle: Res<PixelArtMaterial>,
    normal_handle: Res<NormalMaterial>,
) {
    if input.just_pressed(KeyCode::Space) {
        for (mut current_material, mut handle) in &mut sprites {
            match *current_material {
                CurrentMaterial::PixelArt => {
                    *current_material = CurrentMaterial::Normal;
                    *handle = normal_handle.clone();
                }
                CurrentMaterial::Normal => {
                    *current_material = CurrentMaterial::PixelArt;
                    *handle = pixel_art_handle.clone();
                }
            }
        }
    }
}

#[derive(Component, Default)]
struct SpinCamera {
    target: Vec3,
    offset: Vec3,
    angle: f32,
}

fn spin_camera(mut cameras: Query<(&mut SpinCamera, &mut Transform)>, time: Res<Time>) {
    for (mut spin, mut transform) in &mut cameras {
        spin.angle += std::f32::consts::FRAC_PI_8 * time.delta_seconds();

        let rotation = Quat::from_axis_angle(Vec3::Y, spin.angle);

        transform.translation = spin.target + rotation * spin.offset;
        transform.look_at(spin.target, Vec3::Y);
    }
}
