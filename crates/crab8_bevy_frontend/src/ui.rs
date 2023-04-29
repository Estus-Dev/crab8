use bevy::prelude::*;
use crab8::{registers::Register, Crab8};

use crate::{screen, update_crab8, PlaybackState, Screen};

/// Debugger Plugin for CRAB-8's debug UI
pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui)
            .add_system(update_ui_screen)
            .add_system(update_ui_registers)
            .add_system(update_ui_stack)
            .add_system(
                handle_debug_click
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .after(update_crab8),
            );
    }
}

#[derive(Component)]
pub struct UiStack;

#[derive(Component, PartialEq, Eq)]
pub enum DebugButton {
    Play,
    Pause,
    Stop,
    StepFrame,
    StepInstruction,
}

impl ToString for DebugButton {
    fn to_string(&self) -> String {
        match self {
            Self::Play => "Play",
            Self::Pause => "Pause",
            Self::Stop => "Stop",
            Self::StepFrame => "Step Frame",
            Self::StepInstruction => "Step Instruction",
        }
        .to_string()
    }
}

fn setup_ui(
    mut commands: Commands,
    crab8: Res<Crab8>,
    images: ResMut<Assets<Image>>,
    asset_server: ResMut<AssetServer>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                max_size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                align_self: AlignSelf::Start,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| ui_main_display(parent, &crab8, images, asset_server));
}

fn ui_main_display(
    parent: &mut ChildBuilder,
    crab8: &Crab8,
    images: ResMut<Assets<Image>>,
    mut asset_server: ResMut<AssetServer>,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                flex_grow: 1.0,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| ui_screen(parent, crab8, images))
                        .with_children(|parent| {
                            parent
                                .spawn(NodeBundle {
                                    background_color: Color::BLUE.into(),
                                    style: Style {
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::SpaceBetween,
                                        flex_wrap: FlexWrap::WrapReverse,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    ui_button_bar(parent, &mut asset_server);
                                    ui_register_bar(parent, crab8, &asset_server);
                                });
                        });
                })
                .with_children(|parent| {
                    ui_stack(parent, crab8, &asset_server);
                });
        });
}

fn ui_screen(parent: &mut ChildBuilder, crab8: &Crab8, mut images: ResMut<Assets<Image>>) {
    parent
        .spawn(ImageBundle {
            background_color: Color::WHITE.into(),
            image: UiImage::new(images.add(screen::render_framebuffer(&crab8.screen))),
            style: Style {
                flex_grow: 1.0,
                aspect_ratio: Some(128.0 / 64.0),
                ..default()
            },
            ..default()
        })
        .insert(Screen)
        .insert(Name::new("Screen"));
}

fn update_ui_screen(
    mut commands: Commands,
    query: Query<(Entity, &UiImage), With<Screen>>,
    asset_server: Res<AssetServer>,
    crab8: Res<Crab8>,
    state: Res<State<PlaybackState>>,
    mut images: ResMut<Assets<Image>>,
) {
    use PlaybackState::*;

    let texture = match state.0 {
        Unloaded | Downloading | Stopped => asset_server.load("textures/stopped.png"),
        _ => images.add(screen::render_framebuffer(&crab8.screen)),
    };

    if let Ok((entity, previous_frame)) = query.get_single() {
        let previous_texture = previous_frame.texture.clone();

        commands
            .entity(entity)
            .remove::<UiImage>()
            .insert(UiImage::new(texture.clone()));

        if previous_texture != texture {
            images.remove(previous_texture);
        }
    }
}

fn ui_button_bar(parent: &mut ChildBuilder, asset_server: &mut ResMut<AssetServer>) {
    use DebugButton::*;

    parent
        .spawn(NodeBundle {
            background_color: Color::BLUE.into(),
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Start,
                flex_direction: FlexDirection::Row,
                size: Size::height(Val::Px(48.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| ui_debug_button(parent, Play, asset_server))
        .with_children(|parent| ui_debug_button(parent, Pause, asset_server))
        .with_children(|parent| ui_debug_button(parent, Stop, asset_server))
        .with_children(|parent| ui_debug_button(parent, StepInstruction, asset_server))
        .with_children(|parent| ui_debug_button(parent, StepFrame, asset_server));
}

fn ui_debug_button(
    parent: &mut ChildBuilder,
    button_type: DebugButton,
    asset_server: &mut ResMut<AssetServer>,
) {
    let name = button_type.to_string();
    let icon: Handle<Image> = asset_server.load(format!("buttons/{}.png", name.to_lowercase()));

    parent
        .spawn(ButtonBundle {
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
            image: UiImage::new(icon),
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::horizontal(Val::Px(3.0)),
                size: Size::all(Val::Px(48.0)),
                ..default()
            },
            ..default()
        })
        .insert(button_type)
        .insert(Name::new(format!("{name} Button")));
}

#[allow(clippy::type_complexity)]
pub fn handle_debug_click(
    mut query: Query<
        (&Interaction, &DebugButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<PlaybackState>>,
) {
    use DebugButton::*;
    use Interaction::*;

    for (action, button_type, mut color) in &mut query {
        match *action {
            None => *color = Color::WHITE.into(),
            Hovered => *color = Color::GOLD.into(),
            Clicked => {
                *color = Color::DARK_GRAY.into();
                next_state.set(match *button_type {
                    Pause => PlaybackState::Paused,
                    Play => PlaybackState::Playing,
                    Stop => PlaybackState::Stopped,
                    StepFrame => PlaybackState::StepFrame,
                    StepInstruction => PlaybackState::StepInstruction,
                });
            }
        };
    }
}

fn ui_register_bar(parent: &mut ChildBuilder, crab8: &Crab8, asset_server: &ResMut<AssetServer>) {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            for i in 0x0..=0xF {
                let register = Register::try_from(i as u16).expect("A nibble is a valid register");
                let name = register.name();
                let value = format!("{:#04X}", crab8.registers.get(register));

                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            margin: UiRect::horizontal(Val::Px(3.0)),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        let text_style = TextStyle {
                            color: Color::GRAY,
                            font: asset_server.load("fonts/pixeloid-font/PixeloidMono-VGj6x.ttf"),
                            font_size: 18.0,
                        };
                        let header_style = TextStyle {
                            color: Color::WHITE,
                            ..text_style.clone()
                        };

                        parent.spawn(TextBundle::from_section(name, header_style));
                        parent
                            .spawn(TextBundle::from_section(value, text_style))
                            .insert(register);
                    });
            }
        });
}

fn update_ui_registers(
    mut query: Query<(&mut Text, &Register)>,
    crab8: Res<Crab8>,
    asset_server: ResMut<AssetServer>,
) {
    let text_style = TextStyle {
        color: Color::GRAY,
        font: asset_server.load("fonts/pixeloid-font/PixeloidMono-VGj6x.ttf"),
        font_size: 18.0,
    };

    for (mut text, &register) in &mut query {
        let value = crab8.registers.get(register);
        let value = format!("{value:#04X}");

        text.sections[0] = TextSection::new(value, text_style.clone());
    }
}

fn ui_stack(parent: &mut ChildBuilder, crab8: &Crab8, asset_server: &ResMut<AssetServer>) {
    let font = &asset_server.load("fonts/pixeloid-font/PixeloidMono-VGj6x.ttf");

    parent
        .spawn(NodeBundle {
            background_color: Color::BLUE.into(),
            style: Style {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Stack:",
                TextStyle {
                    font: font.clone(),
                    font_size: 18.0,
                    color: Color::WHITE,
                },
            ));

            parent
                .spawn(TextBundle::from_section(
                    format!("0: {:#05X?}", crab8.program_counter.get()),
                    TextStyle {
                        font: font.clone(),
                        font_size: 18.0,
                        color: Color::WHITE,
                    },
                ))
                .insert(UiStack);
        });
}

fn update_ui_stack(
    mut query: Query<&mut Text, With<UiStack>>,
    crab8: Res<Crab8>,
    asset_server: ResMut<AssetServer>,
) {
    let font = &asset_server.load("fonts/pixeloid-font/PixeloidMono-VGj6x.ttf");

    if let Ok(mut text) = query.get_single_mut() {
        let mut sections = vec![];

        for (i, frame) in crab8.stack.clone().into_iter().enumerate() {
            sections.push(TextSection::new(
                format!("{:#02}: {:#05X?}\n", crab8.stack.len() - i, frame.get()),
                TextStyle {
                    font: font.clone(),
                    font_size: 18.0,
                    color: Color::GRAY,
                },
            ));
        }

        sections.push(TextSection::new(
            format!("PC: {:#05X?}", crab8.program_counter.get()),
            TextStyle {
                font: font.clone(),
                font_size: 18.0,
                color: Color::WHITE,
            },
        ));

        text.sections = sections;
    }
}
