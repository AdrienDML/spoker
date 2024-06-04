use crate::{camera, prelude::*};

use common::{debug, ui::{interactions::InteractiveBackground, FluxInteraction}, val };

mod hierarchy_view;
mod map_view;

use hierarchy_view::HierarchyView;
pub use map_view::MouseOnMap;
use map_view::{update_map_focus, MapView};

pub struct UiPlugin;

#[derive(SystemSet, Clone, Hash, Debug, Eq, PartialEq)]
pub struct UiStartupSet;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SickleUiPlugin)
            .add_plugins(debug::DebugComponentPlugin::<
                (&BackgroundColor, &FluxInteraction),
                (Or<(Changed<BackgroundColor>, Changed<FluxInteraction>)>, With<debug::DebugComponent>),
            >::default())
            .init_resource::<IconCache>()
            .init_resource::<MouseOnMap>()
            .add_systems(
                Startup,
                (setup, (map_view::setup, hierarchy_view::setup))
                    .chain()
                    .in_set(UiStartupSet),
            )
            .add_systems(PreUpdate, (update_map_focus, exit_app_on_menu_item))
            .add_systems(Update, map_view::set_map_view_cam_viewport);
    }
}

#[derive(Component)]
pub struct UiMainRootNode;

#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
#[reflect(Component)]
struct ExitAppButton;

#[derive(Resource, Debug, Default, Reflect)]
#[reflect(Resource)]
pub struct IconCache(Vec<Handle<Image>>);

pub fn setup(
    asset_server: Res<AssetServer>,
    mut icon_cache: ResMut<IconCache>,
    mut commands: Commands,
) {
    // Workaround for disappearing icons when they are despawned and spawned back in during the same frame
    // Should be fixed in Bevy > 0.13
    let icons_to_cache: Vec<&str> = vec![
        "embedded://sickle_ui/icons/checkmark.png",
        "embedded://sickle_ui/icons/chevron_down.png",
        "embedded://sickle_ui/icons/chevron_left.png",
        "embedded://sickle_ui/icons/chevron_right.png",
        "embedded://sickle_ui/icons/chevron_up.png",
        "embedded://sickle_ui/icons/close.png",
        "embedded://sickle_ui/icons/exit_white.png",
        "embedded://sickle_ui/icons/popout_white.png",
        "embedded://sickle_ui/icons/redo_white.png",
        "embedded://sickle_ui/icons/submenu_white.png",
    ];

    for icon in icons_to_cache.iter() {
        icon_cache.0.push(asset_server.load(*icon));
    }

    // The camera which will render the ui
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                order: 1,
                clear_color: Color::BLACK.into(),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 30., 0.))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        camera::UiCamera,
    ));

    // Use the UI builder with plain bundles and direct setting of bundle props
    commands.ui_builder(UiRoot).column(|col| {
        col.style()
            .width(val!(100.0 %))
            .row_gap(val!(3.0 px))
            .padding(UiRect::all(val!(3.0 px)))
            .background_color(Color::BLACK);

        // The menu bar.
        col.row(|row| {
            row.style()
                .height(val!(30.0 px))
                .padding(UiRect::all(val!(3.0 px)))
                .background_color(Color::DARK_GRAY);

            // Todo Spawn button actions.
            row.menu(
                MenuConfig {
                    name: "File".into(),
                    alt_code: KeyCode::KeyF.into(),
                },
                |menu| {
                    menu.style()
                        .background_color(Color::DARK_GRAY);
                    menu.menu_item(MenuItemConfig {
                        name: "Open".into(),
                        shortcut: vec![KeyCode::ControlLeft, KeyCode::KeyO].into(),
                        ..default()
                    });
                    menu.menu_item(MenuItemConfig {
                        name: "Save".into(),
                        shortcut: vec![KeyCode::ControlLeft, KeyCode::KeyS].into(),
                        ..default()
                    });
                    menu.menu_item(MenuItemConfig {
                        name: "Save as".into(),
                        shortcut: vec![KeyCode::ControlLeft, KeyCode::ShiftLeft, KeyCode::KeyS]
                            .into(),
                        ..default()
                    });
                    menu.menu_item(MenuItemConfig {
                        name: "Export".into(),
                        shortcut: vec![KeyCode::ControlLeft, KeyCode::KeyE].into(),
                        ..default()
                    });
                    menu.menu_item_separator();
                    menu.menu_item(MenuItemConfig {
                        name: "Exit".into(),
                        shortcut: vec![KeyCode::ControlLeft, KeyCode::ShiftLeft, KeyCode::KeyQ]
                            .into(),
                        ..default()
                    })
                    .insert(ExitAppButton);
                },
            ).insert();
        });

        // The Editor space.
        col.row(|row| {
            row.style().width(val!(100.0 %)).height(val!(100.0 %));

            // Hierarchy.
            row.sized_zone(
                SizedZoneConfig {
                    size: 30.0,
                    ..default()
                },
                |panel| {
                    panel
                        .insert(HierarchyView)
                        .style()
                        .height(val!(100.0 %))
                        .width(val!(30.0 %));
                },
            );

            // Map View + Toolbar
            row.sized_zone(
                SizedZoneConfig {
                    size: 70.0,
                    ..default()
                },
                |panel| {
                    panel
                        .insert(MapView)
                        .style()
                        .height(val!(100.0 %))
                        .background_color(Color::WHITE);
                },
            );
        });
    });
}

fn exit_app_on_menu_item(
    q_menu_items: Query<&MenuItem, (With<ExitAppButton>, Changed<MenuItem>)>,
    mut exit_app: EventWriter<bevy::app::AppExit>,
) {
    let Ok(item) = q_menu_items.get_single() else {
        return;
    };

    if item.interacted() {
        exit_app.send(bevy::app::AppExit);
    }
}
