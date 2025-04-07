// this example uses bevy_cosmic_edit crate for text input support, it has two inputs with submit button
// TODO: solve issue: CosmicEditBuffer is not being updated when text is changed for password input
use bevy::{color::palettes::css::BLUE, prelude::*};
use bevy_cosmic_edit::{
    cosmic_text::{Attrs, AttrsOwned, Family, Metrics},
    placeholder::Placeholder,
    prelude::*,
};

#[derive(Component)]
struct NameInput;

#[derive(Component)]
struct PasswordInput;

#[derive(Component)]
struct NameDisplay;

#[derive(Component)]
struct PasswordDisplay;

#[derive(Component)]
struct SubmitButton;

fn main() {
    let font_bytes: &[u8] = include_bytes!("../../assets/fonts/FiraMono-Medium.ttf");
    let font_config = CosmicFontConfig {
        fonts_dir_path: None,
        font_bytes: Some(vec![font_bytes]),
        load_system_fonts: true,
    };

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CosmicEditPlugin { font_config })
        .add_systems(Startup, setup)
        .add_systems(Update, submit_inputs)
        .run();
}

fn setup(mut commands: Commands, mut font_system: ResMut<CosmicFontSystem>) {
    let camera_bundle = (
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(bevy::color::palettes::css::PINK.into()),
            ..default()
        },
    );
    commands.spawn(camera_bundle);

    let mut attrs = Attrs::new();
    attrs = attrs.family(Family::Name("Victor Mono"));
    attrs = attrs.color(CosmicColor::rgb(0x94, 0x00, 0xD3));

    let main_container = commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::BLACK),
        ))
        .id();

    commands.entity(main_container).with_children(|parent| {
        parent
            .spawn((
                TextEdit,
                CosmicEditBuffer::new(&mut font_system, Metrics::new(20., 20.)).with_rich_text(
                    &mut font_system,
                    vec![("", attrs)],
                    attrs,
                ),
                Placeholder::new(
                    "Enter name",
                    attrs.color(bevy::color::palettes::basic::GRAY.to_cosmic()),
                ),
                Node {
                    width: Val::Px(200.),
                    height: Val::Px(30.),
                    margin: UiRect::all(Val::Px(20.)),
                    ..default()
                },
                NameInput,
            ))
            .observe(focus_on_click);
    });

    commands.entity(main_container).with_children(|parent| {
        parent
            .spawn((
                TextEdit,
                CosmicEditBuffer::new(&mut font_system, Metrics::new(20., 20.)).with_rich_text(
                    &mut font_system,
                    vec![("", attrs)],
                    attrs,
                ),
                Placeholder::new(
                    "Enter password",
                    attrs.color(bevy::color::palettes::basic::GRAY.to_cosmic()),
                ),
                Node {
                    width: Val::Px(200.),
                    height: Val::Px(30.),
                    margin: UiRect::all(Val::Px(20.)),
                    ..default()
                },
                PasswordInput,
            ))
            .observe(focus_on_click);
    });

    let button_node = Node {
        width: Val::Px(100.0),
        height: Val::Px(40.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    commands.entity(main_container).with_children(|parent| {
        parent
            .spawn((
                Button,
                button_node.clone(),
                BackgroundColor(BLUE.into()),
                SubmitButton,
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

fn submit_inputs(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<SubmitButton>),
    >,
    name_query: Query<&CosmicEditBuffer, With<NameInput>>,
    password_query: Query<&CosmicEditBuffer, With<PasswordInput>>,
) {

    for (interaction, mut background_color) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {

            // *background_color = bevy::color::palettes::css::GREY.into();

            // Handle name input
            if let Ok(name_buffer) = name_query.get_single() {
                let name = name_buffer.get_text_spans(AttrsOwned::new(Attrs::new()))
                    .iter()
                    .flat_map(|line| line.iter().map(|(text, _)| text.as_str()))
                    .collect::<String>();
                if name.starts_with("Enter name") {
                    println!("Name: No input provided");
                } else {
                    println!("Name: {}", name);
                }
            }

            // Handle password input
            if let Ok(password_buffer) = password_query.get_single() {
                let password = password_buffer.get_text_spans(AttrsOwned::new(Attrs::new()))
                    .iter()
                    .flat_map(|line| line.iter().map(|(text, _)| text.as_str()))
                    .collect::<String>();
                if password.starts_with("Enter password") {
                    println!("Password: No input provided");
                } else {
                    println!("Password: {}", password);
                }
            }
        }
    }
}
