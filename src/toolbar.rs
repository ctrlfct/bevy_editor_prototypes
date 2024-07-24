use bevy::prelude::*;

use bevy::color::palettes::basic::*;

use crate::button::create_button;
use crate::editor_settings::EditorSettings;

#[derive(Component)]
pub struct FileMenu;

#[derive(Component)]
pub struct FileButton;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn setup_toolbar(mut commands: Commands, asset_server: Res<AssetServer>, settings: Res<EditorSettings>) {
    commands
        .spawn(toolbar_root())
        .with_children(|parent| {
            spawn_logo(parent, &asset_server);
            spawn_menu_buttons(parent, &settings);
            spawn_player_buttons(parent, &settings);
        });
}

fn toolbar_root() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Px(50.0),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        background_color: Color::srgb(0.2, 0.2, 0.2).into(),
        
        border_radius: BorderRadius::new (
            Val::Px(4.8),
            Val::Px(4.8),
            Val::Px(4.8),
            Val::Px(4.8),
        ),
        ..default()
    }
}

fn spawn_logo(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent.spawn(ImageBundle {
        style: Style {
            width: Val::Px(40.0),
            height: Val::Px(40.0),
            margin: UiRect::right(Val::Px(10.0)),
            ..default()
        },
        image: UiImage::new(asset_server.load("branding/bevy_bird_dark.png")),
        ..default()
    });
}

fn spawn_menu_buttons(parent: &mut ChildBuilder, editor_settings: &Res<EditorSettings>) {
    create_button(parent, editor_settings, "File");
    create_button(parent, editor_settings, "Edit");
    create_button(parent, editor_settings, "View");
    create_button(parent, editor_settings, "Window");
    create_button(parent, editor_settings, "Help");
}

fn spawn_file_buttons(parent: &mut ChildBuilder, editor_settings: &Res<EditorSettings>) {
    create_button(parent, editor_settings, "New");
    create_button(parent, editor_settings, "Open");
    create_button(parent, editor_settings, "Save");
    create_button(parent, editor_settings, "Save As");
    create_button(parent, editor_settings, "Close");
}

fn spawn_player_buttons(parent: &mut ChildBuilder, editor_settings: &Res<EditorSettings>) {
    create_button(parent, editor_settings, "Play");
    create_button(parent, editor_settings, "Pause");
    create_button(parent, editor_settings, "Stop");
}

pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}