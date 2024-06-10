use sickle_ui::{
    animated_interaction::{AnimatedInteraction, AnimationConfig},
    interactions::InteractiveBackground,
    math::ease::Ease,
    ui_builder::UiBuilder,
    widgets::{
        container::UiContainerExt,
        label::{LabelConfig, UiLabelExt},
    },
    TrackedInteraction,
};

use crate::{common::debug, prelude::*};

pub trait UiButtonExt<'w, 's> {
    fn button<'a>(&'a mut self, label: impl Into<String>) -> UiBuilder<'w, 's, 'a, Entity>;
}

impl<'w, 's> UiButtonExt<'w, 's> for UiBuilder<'w, 's, '_, Entity> {
    fn button<'a>(&'a mut self, label: impl Into<String>) -> UiBuilder<'w, 's, 'a, Entity> {
        let label = label.into();
        self.container(
            (
                Name::new(label.clone()),
                debug::DebugComponent,
                ButtonBundle {
                    style: Style {
                        padding: UiRect::axes(val!(10. px), val!(5. px)),
                        border: UiRect::horizontal(val!(1. px)),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::NONE.into(),
                    border_color: Color::NONE.into(),
                    ..default()
                },
                TrackedInteraction::default(),
                InteractiveBackground {
                    highlight: Color::DARK_GRAY.into(),
                    ..default()
                },
                AnimatedInteraction::<InteractiveBackground> {
                    tween: AnimationConfig {
                        duration: 0.1,
                        easing: Ease::OutExpo,
                        ..default()
                    },
                    ..default()
                },
            ),
            |button| {
                button.label(LabelConfig {
                    label,
                    color: Color::ANTIQUE_WHITE,
                    ..default()
                });
            },
        )
    }
}
