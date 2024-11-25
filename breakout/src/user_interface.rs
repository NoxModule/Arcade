use bevy::{
    prelude::NodeBundle,
    ui::{AlignItems, FlexDirection, JustifyContent, Style, Val},
    utils::default,
};

pub struct UserInterface;
impl UserInterface {
    pub fn centered_container() -> NodeBundle {
        NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                ..default()
            },
            ..default()
        }
    }
}
