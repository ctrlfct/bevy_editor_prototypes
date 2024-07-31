use bevy::prelude::*;
use crate::editor_settings::EditorSettings;

pub fn create_button<T: Component>(parent: &mut ChildBuilder, settings: &Res<EditorSettings>, text: &str, component: T) {
    parent.spawn((
        ButtonBundle {
            style: button_style(),
            background_color: settings.button_background.into(),
            border_radius: BorderRadius::MAX,
            ..default()
        },
        component,
    )).with_children(|parent| {
        spawn_button_text(parent, settings, text);
    });
}

fn button_style() -> Style {
    Style {
        width: Val::Px(80.0),
        height: Val::Px(30.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::horizontal(Val::Px(5.0)),
        ..default()
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