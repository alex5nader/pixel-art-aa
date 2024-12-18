use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    pbr::TextureSampler,
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
        texture_sampler: TextureSampler::PixelArt,
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
        Transform::from_xyz(-2.0, 1.5, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        Camera3d::default(),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.1, 0.3, 0.5)),
            ..default()
        },
        SpinCamera {
            offset: Vec3::new(-2.0, 1.5, 2.0),
            target: Vec3::ZERO,
            angle: 0.0,
        },
    ));

    // plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d { normal: Dir3::Y, half_size: Vec2::splat(2.5) })),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    // sprite
    commands.spawn((
        CurrentMaterial::PixelArt,
        Mesh3d(meshes.add(quad_mesh())),
        MeshMaterial3d(pixel_art_material),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

fn quad_mesh() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);

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
    mesh.insert_indices(Indices::U32(vec![
        0, 1, 2,
        0, 2, 3,
    ]));

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
    mut sprites: Query<(&mut CurrentMaterial, &mut MeshMaterial3d<StandardMaterial>)>,
    input: Res<ButtonInput<KeyCode>>,
    pixel_art_material: Res<PixelArtMaterial>,
    normal_material: Res<NormalMaterial>,
) {
    if input.just_pressed(KeyCode::Space) {
        for (mut current_material, mut material) in &mut sprites {
            match *current_material {
                CurrentMaterial::PixelArt => {
                    *current_material = CurrentMaterial::Normal;
                    material.0 = normal_material.clone();
                }
                CurrentMaterial::Normal => {
                    *current_material = CurrentMaterial::PixelArt;
                    material.0 = pixel_art_material.clone();
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
        spin.angle += std::f32::consts::FRAC_PI_8 * time.delta_secs();

        let rotation = Quat::from_axis_angle(Vec3::Y, spin.angle);

        transform.translation = spin.target + rotation * spin.offset;
        transform.look_at(spin.target, Vec3::Y);
    }
}
