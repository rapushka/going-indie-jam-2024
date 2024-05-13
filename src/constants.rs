pub const PLAYER_MOVEMENT_SPEED: f32 = 3.0;
pub const PLAYER_MASS: f32 = 12.5;
pub const GRAVITY_SCALE: f32 = 3.0;
pub const JUMP_FORCE: f32 = 150.0;

pub const DESPAWN_HEIGHT: f32 = -5.0;

pub const STAR_ROTATION_SPEED: f32 = 0.05;

pub const DELAY_BEFORE_GAME_OVER_SCREEN_SHOW: f32 = 0.5;

pub mod controls {
    use bevy::prelude::KeyCode;

    pub const PAUSE_KEY: KeyCode = KeyCode::Escape;
    pub const TOGGLE_DEBUG_VIEW: KeyCode = KeyCode::KeyI;
}

pub mod color {
    use bevy::prelude::*;

    pub const DEFAULT_BUTTON: Color = Color::rgb(0.93, 0.71, 0.32);
    pub const HOVERED_BUTTON: Color = Color::rgb(0.9, 0.81, 0.54);
    pub const PRESSED_BUTTON: Color = Color::rgb(0.89, 0.58, 0.28);

    pub const DEFAULT_TEXT: Color = Color::rgb(0.30, 0.25, 0.22);
}

pub mod styles {
    use bevy::prelude::*;

    pub const LOADING_CURTAIN: Style = {
        let mut style = Style::DEFAULT;
        style.width = Val::Percent(100.0);
        style.height = Val::Percent(100.0);
        style.padding = UiRect::all(Val::Px(10.0));
        style.flex_direction = FlexDirection::ColumnReverse;
        style
    };

    pub const MAIN_MENU: Style = {
        let mut style = Style::DEFAULT;
        style.width = Val::Percent(100.0);
        style.height = Val::Percent(100.0);
        style.flex_direction = FlexDirection::Column;
        style.justify_content = JustifyContent::Center;
        style.align_items = AlignItems::Center;
        style.row_gap = Val::Px(8.0);
        style
    };
    pub const LEVEL_SELECTION: Style = {
        let mut style = Style::DEFAULT;
        style.width = Val::Percent(100.0);
        style.height = Val::Percent(100.0);
        style.flex_direction = FlexDirection::Column;
        style.justify_content = JustifyContent::Center;
        style.align_items = AlignItems::Center;
        style.row_gap = Val::Px(8.0);
        style
    };

    pub const GAMEPLAY_HUD: Style = {
        let mut style = Style::DEFAULT;
        style.width = Val::Percent(100.0);
        style.height = Val::Percent(100.0);
        style.padding = UiRect::all(Val::Px(10.0));
        style
    };

    pub const BUTTON: Style = {
        let mut style = Style::DEFAULT;
        style.justify_content = JustifyContent::Center;
        style.align_items = AlignItems::Center;
        style.width = Val::Px(200.0);
        style.height = Val::Px(80.0);
        style
    };

    pub const SMALL_BUTTON: Style = {
        let mut style = Style::DEFAULT;
        style.justify_content = JustifyContent::Center;
        style.align_items = AlignItems::Center;
        style.width = Val::Px(100.0);
        style.height = Val::Px(80.0);
        style
    };

    pub const TITLE: Style = {
        let mut style = Style::DEFAULT;
        style.flex_direction = FlexDirection::Row;
        style.justify_content = JustifyContent::Center;
        style.align_items = AlignItems::Center;
        style.width = Val::Px(300.0);
        style.height = Val::Px(300.0);
        style
    };
}

