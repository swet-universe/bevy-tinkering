// this example uses bevy_simple_text_input crate for text input support, it has two inputs with submit button
use bevy::{
    color::palettes::{css::BLUE, tailwind::BLUE_400},
    prelude::*,
    ui::FocusPolicy,
};
use bevy_simple_text_input::{
    TextInput, TextInputInactive, TextInputPlaceholder, TextInputPlugin, TextInputSettings,
    TextInputSystem, TextInputTextColor, TextInputTextFont, TextInputValue,
};

const BORDER_COLOR_ACTIVE: Color = Color::srgb(0.75, 0.52, 0.99);
const BORDER_COLOR_INACTIVE: Color = Color::srgb(0.25, 0.25, 0.25);
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const BACKGROUND_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);

#[derive(Component, Clone, Copy)]
struct Username;

#[derive(Component, Clone, Copy)]
struct Password;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TextInputPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, focus.before(TextInputSystem))
        .add_systems(Update, handle_submit)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.),
                ..default()
            },
            // Make this container node interactive so that clicking on it removes
            // focus from the text input.
            Interaction::None,
        ))
        .with_children(|parent| {
            parent.spawn(text_input());
            parent.spawn((
                Node {
                    width: Val::Px(200.0),
                    border: UiRect::all(Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                BorderColor(BORDER_COLOR_ACTIVE),
                BackgroundColor(BACKGROUND_COLOR),
                TextInput,
                TextInputValue("".to_string()),
                TextInputTextFont(TextFont {
                    font_size: 34.,
                    ..default()
                }),
                TextInputTextColor(TextColor(TEXT_COLOR)),
                TextInputPlaceholder {
                    value: "Password".to_string(),
                    ..default()
                },
                TextInputSettings {
                    mask_character: Some('*'),
                    retain_on_submit: true,
                },
                Password,
            ));
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(100.0),
                        height: Val::Px(40.0),
                        margin: UiRect::all(Val::Px(20.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(BLUE.into()),
                    BackgroundColor(BLUE_400.into()),
                ))
                .with_child((
                    Text::new("Submit"),
                    TextFont {
                        font_size: 17.,
                        ..default()
                    },
                    TextColor(Color::srgb(255., 255., 255.)),
                ));
        });
}

fn text_input() -> impl Bundle {
    (
        Node {
            width: Val::Px(200.0),
            border: UiRect::all(Val::Px(5.0)),
            padding: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        BorderColor(BORDER_COLOR_INACTIVE),
        BackgroundColor(BACKGROUND_COLOR),
        // Prevent clicks on the input from also bubbling down to the container
        // behind it
        FocusPolicy::Block,
        TextInput,
        TextInputTextFont(TextFont {
            font_size: 34.,
            ..default()
        }),
        TextInputTextColor(TextColor(TEXT_COLOR)),
        TextInputPlaceholder {
            value: "Name".to_string(),
            ..default()
        },
        TextInputInactive(true),
        Username,
    )
}


fn focus(
    query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut text_input_query: Query<(Entity, &mut TextInputInactive, &mut BorderColor)>,
) {
    for (interaction_entity, interaction) in &query {
        if *interaction == Interaction::Pressed {
            for (entity, mut inactive, mut border_color) in &mut text_input_query {
                // let value = value.clone_value();
                // println!("CHECK VALUE : {:?} ", value);
                if entity == interaction_entity {
                    inactive.0 = false;
                    *border_color = BORDER_COLOR_ACTIVE.into();
                } else {
                    inactive.0 = true;
                    *border_color = BORDER_COLOR_INACTIVE.into();
                }
            }
        }
    }
}

fn handle_submit(
    query: Query<(Entity, &Interaction), (Changed<Interaction>, With<Button>)>,
    mut input_queries: ParamSet<(
        Query<&mut TextInputValue, With<Username>>,
        Query<&mut TextInputValue, With<Password>>,
    )>,
    // mut username_query: Query<&mut TextInputValue, With<Username>>,
    // mut password_query: Query<&mut TextInputValue, With<Password>>,
) {
    for (_interaction_entity, interaction) in &query {
        if *interaction == Interaction::Pressed {
            if let Ok(username_value) = input_queries.p0().get_single_mut() {
                println!("Uname:: {}", username_value.0);
            }

            if let Ok(password_value) = input_queries.p1().get_single_mut() {
                println!("pwd:: {}", password_value.0);
            }
        }
    }
}
