use bevy::prelude::*;
pub use main_menu::*;
use crate::constants;
use crate::ui::gameplay_hud::GameplayHudPlugin;
use crate::ui::level_selection::LevelSelectionPlugin;

mod main_menu;
mod level_selection;
mod gameplay_hud;
mod create;

pub mod order {
    use bevy::prelude::ZIndex;

    pub const LOADING_CURTAIN: ZIndex = ZIndex::Global(1_000);
    pub const MAIN_MENU: ZIndex = ZIndex::Global(100);
    pub const LEVEL_SELECTION: ZIndex = ZIndex::Global(100);
    pub const PAUSE_MENU: ZIndex = ZIndex::Global(200);
    pub const GAMEPLAY_HUD: ZIndex = ZIndex::Global(10);
}

#[derive(Event)]
pub struct Clicked(Entity);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Clicked>()

            .add_plugins((
                MainMenuPlugin,
                LevelSelectionPlugin,
                GameplayHudPlugin,
            ))

            .add_systems(Update, (
                visualise_interaction_with_buttons,
                click_on_pressed_button,
                gameplay_hud::pause::on_back_button_clicked,
            ))
        ;
    }
}

pub fn visualise_interaction_with_buttons(
    mut buttons: Query<(&Interaction, &mut BackgroundColor), Changed<Interaction>>,
) {
    for (interaction, mut background_color) in buttons.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = constants::color::PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *background_color = constants::color::HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *background_color = constants::color::DEFAULT_BUTTON.into();
            }
        };
    }
}

pub fn click_on_pressed_button(
    mut buttons: Query<(&Interaction, Entity), Changed<Interaction>>,
    mut clicked_event: EventWriter<Clicked>,
) {
    for (interaction, entity) in buttons.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                clicked_event.send(Clicked(entity));
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        };
    }
}
