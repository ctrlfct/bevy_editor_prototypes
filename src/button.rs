use bevy::prelude::*;
use crate::editor_settings::EditorSettings;

pub enum ButtonOrientation {
    Horizontal,
    Vertical,
}

pub fn create_button<T: Component>(
    parent: &mut ChildBuilder,
    settings: &Res<EditorSettings>,
    text: &str,
    component: T,
    orientation: ButtonOrientation,
) {
    parent.spawn((
        ButtonBundle {
            style: button_style(orientation),
            background_color: settings.button_background.into(),
            border_radius: BorderRadius::MAX,
            ..default()
        },
        component,
    )).with_children(|parent| {
        spawn_button_text(parent, settings, text);
    });
}

fn button_style(orientation: ButtonOrientation) -> Style {
    match orientation {
        ButtonOrientation::Horizontal => Style {
            width: Val::Px(80.0),
            height: Val::Px(30.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::horizontal(Val::Px(5.0)),
            ..default()
        },
        ButtonOrientation::Vertical => Style {
            width: Val::Px(100.0),
            height: Val::Px(30.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::vertical(Val::Px(5.0)),
            // flex_direction: FlexDirection::Column,
            ..default()
        },
    }
}

fn spawn_button_text(parent: &mut ChildBuilder, settings: &Res<EditorSettings>, text: &str) {
    parent.spawn(TextBundle::from_section(
        text,
        TextStyle {
            font: settings.font.clone(),
            font_size: 16.0,
            color: settings.button_text_color,
        },
    ));
}