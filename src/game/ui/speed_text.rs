use crate::{game::player::components::*, prelude::*};

#[derive(Component)]
pub struct SpeedText;

pub fn setup(mut commands: Commands) {
    commands
        .spawn(TextBundle::default().with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Percent(55.0),
            left: Val::Percent(50.0),
            ..default()
        }))
        .insert(SpeedText);
}

pub fn track_speed(
    player: Query<&Velocity, With<Player>>,
    mut text: Query<&mut Text, With<SpeedText>>,
) {
    for Velocity(vel) in &player {
        let vel = vel.xz().length() * 10.0;
        *text.single_mut() = Text::from_section(
            format!("{vel:.0}"),
            TextStyle {
                font_size: 20.0,
                ..default()
            },
        )
    }
}
