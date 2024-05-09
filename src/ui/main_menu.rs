use bevy::prelude::*;
use crate::{AppState, constants, ui};
use crate::ui::create;

pub struct MainMenuPlugin;

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct QuitButton;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::MainMenu), (
                build_main_menu,
            ))
        ;
    }
}

pub fn build_main_menu(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.spawn((
        MainMenu {},
        NodeBundle {
            style: constants::styles::MAIN_MENU,
            z_index: ui::order::MAIN_MENU,
            ..default()
        },
    ))
        .with_children(|parent| {
            create::title(&asset_server, parent);
            create::button(&asset_server, parent, "Play", PlayButton {});
            create::button(&asset_server, parent, "Quit", QuitButton {});
        });
}
