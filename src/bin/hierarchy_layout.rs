//! This example illustrates hierarchy pattern.
//! 
use bevy::app::App;
use bevy::color::palettes::css::{BLUE, GREEN, GREY};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let text_font = (
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 33.0,
            ..Default::default()
        },
        TextColor(Color::BLACK),
    );

    // hierarchical structure
    // multiple children
    commands
        .spawn((  // PARENT CONTAINER (root)
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::WHITE),
        ))
        .with_children(
            // Child text node
            |parent| {  
            parent.spawn((
                Node {
                    height: Val::Percent(80.),
                    width: Val::Percent(80.),
                    ..default()
                },
                BackgroundColor(GREY.into()),
                Text::new("Inside container 1"),
                text_font.clone()
            ));
        })
        .with_children(|parent| {   // CHILD CONTAINER
            parent.spawn((
                Node {
                    height: Val::Percent(80.),
                    width: Val::Percent(80.),
                    display: Display::Flex,
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::End,
                    ..default()
                },
                BackgroundColor(GREEN.into()),
                Text::new("Inside container 2"),
                text_font.clone()
            ))
            .with_child((
                Node {
                    height: Val::Percent(20.),
                    width: Val::Percent(40.),
                    align_content: AlignContent::Center,
                    ..default()
                },
                BackgroundColor(BLUE.into()),
                Text::new("SINGLE CHILD"),
                text_font.clone()
            ));
        });

}
