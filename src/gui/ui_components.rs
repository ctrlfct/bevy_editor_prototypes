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
pub struct CommandPaletteAction;


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

#[derive(Component)]
pub struct ProjectNameInputFieldAction;

#[derive(Component)]
pub struct ProjectPathInputFieldAction;

#[derive(Component)]
pub enum NewProjectButtonsAction {
    Cancel, 
    Create,
}

#[derive(Component)]
pub enum ProjectManagerAction {
    Run,
    Rename,
    Remove,
}