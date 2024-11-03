//! Shows how to render UI to a texture. Useful for displaying UI in 3D space.
//!
//! You can also change the scale factor of the render target by pressing the up
//! or down arrow keys. This will change the size at which the UI renders.

use std::f32::consts::PI;

use bevy::{
    color::palettes::css::GOLD,
    prelude::*,
    render::{
        camera::RenderTarget,
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (rotator_system, change_scale_factor))
        .run();
}

// Marks the cube, to which the UI texture is applied.
#[derive(Component)]
struct Cube;

// Marker component for the camera which renders into the UI texture.
#[derive(Component)]
struct TextureCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image::new_fill(
        size,
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::default(),
    );
    // You need to set these texture usage flags in order to use the image as a render target
    image.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;

    let image_handle = images.add(image);

    // Light
    commands.spawn(DirectionalLight::default());

    let texture_camera = commands
        .spawn((
            Camera2d,
            Camera {
                target: RenderTarget::from(image_handle.clone()),
                ..default()
            },
            TextureCamera,
        ))
        .id();

    commands
        .spawn((
            Node {
                // Cover the whole image
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(GOLD.into()),
            TargetCamera(texture_camera),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("This is a cube"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor::BLACK,
            ));
        });

    let cube_size = 4.0;
    let cube_handle = meshes.add(Cuboid::new(cube_size, cube_size, cube_size));

    // This material has the texture that has been rendered.
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle),
        reflectance: 0.02,
        unlit: false,

        ..default()
    });

    // Cube with material containing the rendered UI texture.
    commands.spawn((
        Mesh3d(cube_handle),
        MeshMaterial3d(material_handle),
        Transform::from_xyz(0.0, 0.0, 1.5).with_rotation(Quat::from_rotation_x(-PI / 5.0)),
        Cube,
    ));

    // The main pass camera.
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

const ROTATION_SPEED: f32 = 0.5;

fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Cube>>) {
    for mut transform in &mut query {
        transform.rotate_x(1.0 * time.delta_secs() * ROTATION_SPEED);
        transform.rotate_y(0.7 * time.delta_secs() * ROTATION_SPEED);
    }
}

const UI_SCALE_CHANGE_STEP: f32 = 0.25;

fn change_scale_factor(
    input: Res<ButtonInput<KeyCode>>,
    mut texture_camera: Single<&mut Camera, With<TextureCamera>>,
) {
    let mut scale_factor_change = 0.0;
    if input.just_pressed(KeyCode::ArrowUp) {
        scale_factor_change += UI_SCALE_CHANGE_STEP;
    }
    if input.just_pressed(KeyCode::ArrowDown) {
        scale_factor_change -= UI_SCALE_CHANGE_STEP;
    }

    if scale_factor_change != 0.0 {
        let RenderTarget::Image { scale_factor, .. } = &mut texture_camera.target else {
            panic!("render target should be an image");
        };

        *scale_factor += scale_factor_change;
        info!("Changed render target scale factor by {scale_factor_change}, now {scale_factor}");
    }
}
