//! This example illustrates very first root node

use bevy::app::App;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // root node
    commands
        .spawn(( 
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.0, 255., 0.)),
        ));

    
}
