//! This example illustrates with container approach, how to create confirm & cancel buttons that updates selected button value
//!
use bevy::app::App;
use bevy::color::palettes::css::{BLUE, GREY, WHITE};
use bevy::color::palettes::tailwind::CYAN_400;
use bevy::prelude::*;

#[derive(Component)]
struct ConfirmButton;

#[derive(Component)]
struct CancelButton;

#[derive(Resource, Debug)]
enum SelectedButton {
    Confirm,
    Cancel,
    None,
}

impl Default for SelectedButton {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Component, Clone, Copy)]
struct StatusText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(SelectedButton::None)
        .add_systems(Startup, setup_ui)
        .add_systems(Update, button_systems)
        .run();
}

fn setup_ui(mut commands: Commands, selected_button: Res<SelectedButton>) {
    commands.spawn(Camera2d);

    // Define colors
    let root_color = Color::srgb(0.1, 0.1, 0.1);
    let box_color = Color::srgb(0., 0., 0.);

    // Root node (acts like a full-screen container)
    let main_container = commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(root_color.into()),
        ))
        .id();

    // TODO: make round border
    let body_container = commands
        .spawn((
            Node {
                width: Val::Px(500.),
                height: Val::Px(200.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Start,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(box_color.into()),
        ))
        .id();

    commands.entity(main_container).add_child(body_container);

    commands.entity(body_container).with_children(|parent| {
        parent.spawn((
            Text::new("Confirm action!"),
            TextFont {
                font_size: 30.,
                ..default()
            },
            TextColor(WHITE.into()),
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(60.),
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(GREY.into()),
        ));
    });

    let button_node = Node {
        width: Val::Px(100.0),
        height: Val::Px(40.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    commands.entity(body_container).with_children(|parent| {
        parent
            .spawn((
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(40.),
                    align_items: AlignItems::End,
                    justify_content: JustifyContent::SpaceAround,
                    justify_items: JustifyItems::End,
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                BackgroundColor(WHITE.into()),
            ))
            .with_children(|parent| {
                parent
                    .spawn((
                        Button,
                        button_node.clone(),
                        BackgroundColor(BLUE.into()),
                        ConfirmButton,
                    ))
                    .with_child((
                        Text::new("Confirm"),
                        TextFont {
                            font_size: 17.,
                            ..default()
                        },
                        TextColor(Color::srgb(255., 255., 255.)),
                    ));

                parent
                    .spawn((
                        Button,
                        button_node.clone(),
                        BackgroundColor(GREY.into()),
                        CancelButton, // BackgroundColor(NORMAL_BUTTON),
                    ))
                    .with_child((
                        Text::new("Cancel"),
                        TextFont {
                            font_size: 17.,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
            });
    });

    selected_button_text_view(&mut commands, main_container, &selected_button);
}

fn selected_button_text_view(
    commands: &mut Commands,
    main_container: Entity,
    selected_button: &SelectedButton,
) {
    commands.entity(main_container).with_child((
        Text::new(get_text_view(selected_button)),
        TextFont {
            font_size: 30.,
            ..default()
        },
        TextColor(CYAN_400.into()),
        StatusText,
    ));
}

fn get_text_view(currently_selected_option: &SelectedButton) -> String {
    format!("selected action is ...  {:?}", currently_selected_option)
}

// changing component - with
fn button_systems(
    interaction_query: Query<
        (
            &Interaction,
            &Children,
            Option<&ConfirmButton>,
            Option<&CancelButton>,
        ),
        Changed<Interaction>,
    >,
    mut text_query: Query<&mut Text, With<StatusText>>,
    mut selected_button: ResMut<SelectedButton>,
) {
    let mut status_text = text_query.get_single_mut().unwrap();

    for (interaction, _, confirm_button, cancel_button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            if confirm_button.is_some() {
                println!("CONFIRM CLICKED!");
                *selected_button = SelectedButton::Confirm;
            } else if cancel_button.is_some() {
                println!("CANCEL CLICKED!");
                *selected_button = SelectedButton::Cancel;
            }
            status_text.0 = get_text_view(&selected_button);
        }
    }
}
