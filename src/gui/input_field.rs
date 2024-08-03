use bevy::prelude::*;
use crate::editor_settings::EditorSettings;
use bevy::input::common_conditions::*;
use bevy::input::keyboard::KeyboardInput;

#[derive(Component)]
pub struct TextInput {
    value: String,
}

#[derive(Debug, Clone, Event)]
pub struct TextInputEvent {
    pub value: String,
}

pub fn create_input_field(
    parent: &mut ChildBuilder,
    settings: &Res<EditorSettings>,
    text: &str,
    component: impl Component,
) {
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(200.0),
                height: Val::Px(30.0),
                padding: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            border_radius: BorderRadius::new (
                Val::Px(8.0),
                Val::Px(8.0),
                Val::Px(8.0),
                Val::Px(8.0),
            ),
            background_color: settings.input_panel_background.into(),
            ..default()
        },

        component,
    )).with_children(|parent| {
        spawn_input_field_text(parent, settings, text);
    });
}

fn spawn_input_field_text(
    parent: &mut ChildBuilder,
    settings: &Res<EditorSettings>,
    text: &str,
) {
    parent.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font: settings.font.clone(),
                font_size: 16.0,
                color: settings.sub_panel_background,
            },
        ));
}

fn text_input_system(
    mut submit_events: EventReader<TextInputEvent>,
    mut query: Query<(&mut Text, &mut TextInput)>,
) {
    for event in submit_events.read() {
        for (mut text, mut text_input) in query.iter_mut() {
            text_input.value = event.value.clone();
            text.sections[0].value = text_input.value.clone();
        }
    }
}
