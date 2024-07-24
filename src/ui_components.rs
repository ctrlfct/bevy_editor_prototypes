use bevy::prelude::*;

#[derive(Component)]
pub enum MenuButtonsAction {
    File,
    Edit,
    View,
    Window,
    Help,
}

#[derive(Component)]
pub enum PlayerButtonsAction {
    Play,
    Pause,
    Stop,
}

#[derive(Component)]
pub enum FileButtonsAction {
    New,
    Open,
    Save,
    SaveAs,
    Close
}

#[derive(Component)]
pub enum EditButtonsAction {
    Undo,
    Redo,
}