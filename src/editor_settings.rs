use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct EditorSettings {
    pub font: Handle<Font>,
    pub text_color: Color,
    pub panel_background: Color,
    pub sub_panel_background: Color,
    pub button_background: Color,
    pub button_hovered_background: Color,
    pub button_pressed_background: Color,
    pub border_color: Color,
    pub border_radius: f32,
}

impl EditorSettings {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        Self {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            text_color: Color::srgb(1.0, 1.0, 1.0),
            panel_background: Color::srgb(0.137, 0.137, 0.149),
            sub_panel_background: Color::srgb(0.22, 0.22, 0.24),
            button_background: Color::srgb(0.2, 0.2, 0.2),
            button_hovered_background: Color::srgb(0.25, 0.25, 0.25),
            button_pressed_background: Color::srgb(0.3, 0.3, 0.3),
            border_color: Color::srgb(0.3, 0.3, 0.3),
            border_radius: 4.0,
        }
    }
}