use bevy::{
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
    ui::{widget::UiImageSize, FocusPolicy},
};
use common::input;

use crate::{
    camera::{self, MapCamera},
    prelude::*,
};

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct MapView;

#[derive(Deref, DerefMut, Resource, Default, PartialEq, Eq)]
pub struct MouseOnMap(pub bool);

pub fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    map_view: Query<Entity, With<MapView>>,
) {
    // Create the render target for the image.
    let image = {
        let size = Extent3d {
            width: 512,
            height: 512,
            ..default()
        };
        let mut image_ = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Bgra8UnormSrgb,
                usage: TextureUsages::COPY_DST
                    | TextureUsages::RENDER_ATTACHMENT
                    | TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            },
            ..default()
        };
        image_.resize(size);
        images.add(image_)
    };

    // Add the image to the map_view.
    let mut map_view = commands.entity(map_view.single());
    map_view.insert((
        UiImage::new(image.clone()),
        UiImageSize::default(),
        Interaction::None,
        FocusPolicy::Block,
    ));

    // Spawn the camera rendering the map_view.
    commands.spawn((
        camera::MapCamera,
        Camera3dBundle {
            camera: Camera {
                order: 0,
                target: image.clone().into(),
                ..default()
            },
            transform: Transform::from_translation(Vec3::splat(10.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        input::Mouse::default(),
        input::MovAxis3::default(),
        input::dont_update::<input::Mouse>(),
        input::dont_update::<input::MovAxis3>(),
    ));
}

pub fn update_map_focus(
    mut mouse_on_map: ResMut<MouseOnMap>,
    map_interaction: Query<&Interaction, (With<MapView>, Changed<Interaction>)>,
) {
    if let Ok(map_interaction) = map_interaction.get_single() {
        **mouse_on_map = matches!(map_interaction, Interaction::Hovered | Interaction::Pressed);
    }
}

pub fn set_map_view_cam_viewport(
    map_views: Query<&Node, (Changed<GlobalTransform>, With<MapView>)>,
    mut map_cam: Query<&mut Camera, With<MapCamera>>,
    mut images: ResMut<Assets<Image>>,
) {
    for node in &map_views {
        let Ok(mut camera) = map_cam.get_single_mut() else {
            continue;
        };

        let size = node.size();

        if size.x == 0. || size.y == 0. {
            camera.is_active = false;
            continue;
        }

        camera.is_active = true;

        if let RenderTarget::Image(render_texture) = camera.target.clone() {
            let Some(texture) = images.get_mut(render_texture) else {
                continue;
            };

            let size = Extent3d {
                width: size.x as u32,
                height: size.y as u32,
                ..default()
            };

            texture.resize(size);
        }
    }
}
