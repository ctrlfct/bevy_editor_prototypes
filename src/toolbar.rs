use bevy::prelude::*;

use crate::button::create_button;
use crate::editor_settings::EditorSettings;

#[derive(Component)]
pub struct FileMenu;

#[derive(Component)]
pub struct FileButton;

pub fn setup_toolbar(mut commands: Commands, asset_server: Res<AssetServer>, settings: Res<EditorSettings>) {
    
    commands
        .spawn(toolbar_root())
        .with_children(|parent| {
            spawn_logo(parent, &asset_server);
            spawn_menu_buttons(parent, &settings);
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


pub fn toggle_file_menu(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<FileButton>)
    >,
    mut text_query: Query<&mut Text>,
    mut menu_query: Query<&mut Style, With<FileMenu>>,
) {
    for (interaction, mut background_color, children) in interaction_query.iter_mut() {
        println!("Interaction detected: {:?}", interaction);
        if let Interaction::Pressed = *interaction {
            if let Some(child) = children.first() {
                if let Ok(mut text) = text_query.get_mut(*child) {
                    text.sections[0].value = "Pressed!".to_string();
                }
            }
        }
    }
}
