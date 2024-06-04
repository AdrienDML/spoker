use common::input;

use crate::csg::brush_mesh::BrushMesh;
use crate::prelude::*;
use crate::select::{SelectMode, Selection};

#[derive(Component)]
pub struct Moving;

#[derive(Component, Deref, DerefMut)]
pub struct MoveAxis(Direction3d);

pub fn handle_move(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut selected: Query<(Entity, Option<&mut MoveAxis>), With<Selection>>,
    mut local: Local<bool>, // Moving or not.
) {
    if keys.just_pressed(KeyCode::KeyG) && !*local {
        for (entity, _) in &selected {
            commands.entity(entity).insert(input::Mouse::default());
            *local = true; // Only set to true if there is a selection.
        }
    } else {
        if mouse_button.just_pressed(MouseButton::Left) {
            for (entity, _) in &selected {
                commands.entity(entity).remove::<input::Mouse>();
            }
            *local = false;
            return;
        }

        // Set the movement axis.
        let mov_axis = if keys.just_pressed(KeyCode::KeyX) {
            Direction3d::X
        } else if keys.just_pressed(KeyCode::KeyY) {
            Direction3d::Y
        } else if keys.just_pressed(KeyCode::KeyZ) {
            Direction3d::Z
        } else {
            return;
        };

        for (entity, axis) in &mut selected {
            let Some(mut axis) = axis else {
                commands.entity(entity).insert(MoveAxis(mov_axis));
                return;
            };
            axis.0 = mov_axis;
        }
    }
}

pub fn move_selection(
    select_mode: Res<SelectMode>,
    mut selected: Query<(
        &mut Transform,
        &Handle<BrushMesh>,
        &Selection,
        &input::Mouse,
        Option<&MoveAxis>,
    )>,
) {
    for (mut transform, _brush, _selection, mouse, axis) in &mut selected {
        match *select_mode {
            SelectMode::Object => {
                let dir = axis
                    .map(|a| *a.0)
                    .unwrap_or(transform.up() * mouse.delta.y + transform.left() * mouse.delta.x);
                transform.translation += dir;
            }
            _ => println!("Unimplemented"),
        }
    }
}
