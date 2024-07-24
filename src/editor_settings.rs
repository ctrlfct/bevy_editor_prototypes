use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct EditorSettings {
    pub font: Handle<Font>,
    pub text_color: Color,
    pub button_text_color: Color,
    pub panel_background: Color,
    pub sub_panel_background: Color,
    pub button_background: Color,
    pub button_hovered_background: Color,
    pub button_pressed_background: Color,
}

impl EditorSettings {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        Self {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            text_color: Color::WHITE,
            button_text_color: Color::WHITE,
            panel_background: Color::srgb(0.2, 0.2, 0.2),
            sub_panel_background: Color::srgb(0.3, 0.3, 0.3),
            button_background: Color::srgb(0.4, 0.4, 0.4),
            button_hovered_background: Color::srgb(0.5, 0.5, 0.5),
            button_pressed_background: Color::srgb(0.3, 0.3, 0.3),
        }
    }
}