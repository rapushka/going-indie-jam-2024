use bevy::prelude::*;
use bevy::asset::AssetServer;
use bevy::hierarchy::ChildBuilder;
use bevy::core::Name;
use crate::constants;

pub fn title(
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
    title_text: &str,
) {
    parent.spawn(NodeBundle { style: constants::styles::TITLE, ..default() })
        .with_children(|parent| {
            text(asset_server, title_text, parent, 64.0);
        });
}

pub fn button<C>(
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
    string: &str,
    component: C,
)
    where C: Component {
    button_internal(asset_server, parent, string, component, constants::styles::BUTTON);
}

pub fn small_button<C>(
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
    string: &str,
    component: C,
)
    where C: Component {
    button_internal(asset_server, parent, string, component, constants::styles::SMALL_BUTTON);
}

fn button_internal<C>(
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
    string: &str,
    component: C,
    style: Style,
) where C: Component {
    parent.spawn((
        component,
        ButtonBundle {
            style,
            background_color: constants::color::DEFAULT_BUTTON.into(),
            ..default()
        },
    ))
        .with_children(|parent| {
            text(asset_server, string, parent, 32.0);
        });
}

pub fn horizontal_layout(
    parent: &mut ChildBuilder,
    spawn_children: impl FnOnce(&mut ChildBuilder),
) {
    parent.spawn((
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(10.0),
                margin: UiRect::all(Val::Px(25.0)),
                ..default()
            },
            ..default()
        }
    ))
        .with_children(spawn_children);
}

pub fn text(
    asset_server: &Res<AssetServer>,
    text: &str,
    parent: &mut ChildBuilder,
    font_size: f32,
) {
    parent.spawn((
        Name::new(format!("text: {text}")),
        TextBundle {
            text: Text {
                sections: vec![
                    TextSection::new(
                        text,
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size,
                            color: constants::color::DEFAULT_TEXT,
                        },
                    )],
                justify: JustifyText::Center,
                ..default()
            },
            ..default()
        },
    ));
}
