use bevy::prelude::*;

use bevy::color::palettes::basic::*;

use crate::button::create_button;
use crate::editor_settings::EditorSettings;
use crate::ui_components::MenuButtonsAction;
use crate::ui_components::FileButtonsAction;
use crate::ui_components::PlayerButtonsAction;

#[derive(Component)]
pub struct FileMenu;

#[derive(Component)]
pub struct FileButton;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn setup_toolbar(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, settings: &Res<EditorSettings>) {
    parent.spawn(toolbar_root(settings))
        .with_children(|toolbar_parent| {
            spawn_logo(toolbar_parent, asset_server);
            
            toolbar_parent.spawn(menu_buttons_container())
                .with_children(|menu_parent| {
                    spawn_menu_buttons(menu_parent, settings);
                });
            
            toolbar_parent.spawn(player_buttons_container())
                .with_children(|player_parent| {
                    spawn_player_buttons(player_parent, settings);
                });
        });
}

fn toolbar_root(settings: &EditorSettings) -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(99.0),
            height: Val::Px(50.0),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(10.0)),
            margin: UiRect::all(Val::Percent(0.5)),
            ..default()
        },
        
        border_radius: BorderRadius::new (
            Val::Px(4.8),
            Val::Px(4.8),
            Val::Px(4.8),
            Val::Px(4.8),
        ),
        background_color: settings.sub_panel_background.into(),
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

fn menu_buttons_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    }
}

fn player_buttons_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            align_items: AlignItems::Center,
            margin: UiRect::left(Val::Px(20.0)),
            ..default()
        },
        ..default()
    }
}

fn spawn_menu_buttons(parent: &mut ChildBuilder, editor_settings: &Res<EditorSettings>) {
    create_button(parent, editor_settings, "File", MenuButtonsAction::File);
    create_button(parent, editor_settings, "Edit", MenuButtonsAction::Edit);
    create_button(parent, editor_settings, "View", MenuButtonsAction::View);
    create_button(parent, editor_settings, "Window", MenuButtonsAction::Window);
    create_button(parent, editor_settings, "Help", MenuButtonsAction::Help);
}

fn spawn_file_buttons(parent: &mut ChildBuilder, editor_settings: &Res<EditorSettings>) {
    create_button(parent, editor_settings, "New", FileButtonsAction::New);
    create_button(parent, editor_settings, "Open", FileButtonsAction::Open);
    create_button(parent, editor_settings, "Save", FileButtonsAction::Save);
    create_button(parent, editor_settings, "Save as", FileButtonsAction::SaveAs);
    create_button(parent, editor_settings, "Close", FileButtonsAction::Close);
}

fn spawn_player_buttons(parent: &mut ChildBuilder, editor_settings: &Res<EditorSettings>) {
    create_button(parent, editor_settings, "Play", PlayerButtonsAction::Play);
    create_button(parent, editor_settings, "Pause", PlayerButtonsAction::Pause);
    create_button(parent, editor_settings, "Stop", PlayerButtonsAction::Stop);
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

pub fn file_button_system(
    interaction_query: Query<(&Interaction, &MenuButtonsAction), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, action) in interaction_query.iter() {
        if *interaction == Interaction::Pressed && matches!(action, MenuButtonsAction::File) {
            println!("File menu button pressed");
        }
    }
}