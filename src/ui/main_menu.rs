use bevy::app::AppExit;
use bevy::prelude::*;
use crate::{AppState, constants, ui};
use crate::ui::{Clicked, create};

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

            .add_systems(Update, (
                on_quit_button_clicked,
                on_play_button_clicked,
            ).run_if(in_state(AppState::MainMenu)))

            .add_systems(OnExit(AppState::MainMenu), (
                destroy_main_menu,
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

pub fn destroy_main_menu(
    main_menus: Query<Entity, With<MainMenu>>,
    mut commands: Commands,
) {
    for main_menu in main_menus.iter() {
        commands.entity(main_menu).despawn_recursive();
    }
}

pub fn on_play_button_clicked(
    buttons: Query<Entity, With<PlayButton>>,
    mut event_reader: EventReader<Clicked>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for event in event_reader.read() {
        if let Ok(_) = buttons.get(event.0) {
            next_state.set(AppState::Gameplay);
        }
    }
}

pub fn on_quit_button_clicked(
    buttons: Query<Entity, With<QuitButton>>,
    mut event_reader: EventReader<Clicked>,
    mut app_exit_event: EventWriter<AppExit>,
) {
    for event in event_reader.read() {
        if let Ok(_) = buttons.get(event.0) {
            app_exit_event.send(AppExit);
        }
    }
}
