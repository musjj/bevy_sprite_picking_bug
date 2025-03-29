use bevy::{
    color::palettes::css::PINK,
    image::ImageSampler,
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_sprite)
        .run()
}

fn setup_sprite(mut commands: Commands, mut assets: ResMut<Assets<Image>>) {
    commands.spawn(Camera2d);

    let size = Extent3d {
        width: 500,
        height: 500,
        ..default()
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::bevy_default(),
            usage: TextureUsages::COPY_DST
                | TextureUsages::TEXTURE_BINDING
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        sampler: ImageSampler::nearest(),
        ..default()
    };

    image.resize(size);

    let image_handle = assets.add(image);

    commands
        .spawn((
            Sprite::from_image(image_handle.clone()),
            Pickable::default(),
        ))
        .observe(|_: Trigger<Pointer<Over>>| {
            info!("hovering!");
        });

    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(PINK.into()),
            target: RenderTarget::Image(image_handle.into()),
            ..default()
        },
    ));
}
