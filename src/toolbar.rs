use bevy::prelude::*;

#[derive(Component)]
pub struct FileMenu;

#[derive(Component)]
pub struct FileButton;

pub fn setup_toolbar(mut commands: Commands, asset_server: Res<AssetServer>) {
    
    commands
        .spawn(toolbar_root())
        .with_children(|parent| {
            spawn_logo(parent, &asset_server);
            spawn_menu_buttons(parent, &asset_server);
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

fn spawn_menu_buttons(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    spawn_file_button(parent, asset_server);
    spawn_regular_button(parent, asset_server, "Edit");
    spawn_regular_button(parent, asset_server, "View");
    spawn_regular_button(parent, asset_server, "Window");
    spawn_regular_button(parent, asset_server, "Help");
}

fn spawn_file_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent.spawn((
        ButtonBundle {
            style: button_style(),
            background_color: Color::srgb(0.4, 0.4, 0.4).into(),
            ..default()
        },
        FileButton,
    ))
    .with_children(|parent| {
        spawn_button_text(parent, asset_server, "File");
        spawn_file_menu(parent, asset_server);
    });
}

fn spawn_regular_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, text: &str) {
    parent.spawn(ButtonBundle {
        style: button_style(),
        background_color: Color::srgb(0.4, 0.4, 0.4).into(),
        ..default()
    })
    .with_children(|parent| {
        spawn_button_text(parent, asset_server, text);
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

fn spawn_button_text(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, text: &str) {
    parent.spawn(TextBundle::from_section(
        text,
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 16.0,
            color: Color::WHITE,
        },
    ));
}

fn spawn_file_menu(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent.spawn((
        NodeBundle {
            style: Style {
                display: Display::None,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                top: Val::Px(35.0),
                left: Val::Px(0.0),
                ..default()
            },
            background_color: Color::srgb(0.2, 0.2, 0.2).into(),
            ..default()
        },
        FileMenu,
    ))
    .with_children(|parent| {
        spawn_file_menu_button(parent, asset_server, "New");
        spawn_file_menu_button(parent, asset_server, "Open");
    });
}

fn spawn_file_menu_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>,text: &str) {
    parent.spawn(ButtonBundle {
        style: button_style(),
        background_color: Color::srgb(0.4, 0.4, 0.4).into(),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 16.0,
                color: Color::WHITE,
            },
        ));
    });
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
            // ... rest of the pressed logic
        }
    }
}
