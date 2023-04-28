use bevy::prelude::*;
use crab8::Crab8;

use crate::{screen, Screen};

/// Debugger Plugin for CRAB-8's debug UI
pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui)
            .add_system(update_ui_screen);
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
    asset_server: ResMut<AssetServer>,
) {
    parent
        .spawn(NodeBundle {
            background_color: Color::RED.into(),
            style: Style {
                flex_direction: FlexDirection::Column,
                flex_grow: 1.0,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| ui_screen(parent, crab8, images))
        .with_children(|parent| ui_button_bar(parent, asset_server));
}

fn ui_screen(parent: &mut ChildBuilder, crab8: &Crab8, mut images: ResMut<Assets<Image>>) {
    parent
        .spawn(ImageBundle {
            background_color: Color::ANTIQUE_WHITE.into(),
            image: UiImage::new(images.add(screen::render_framebuffer(&crab8.screen))),
            style: Style {
                flex_grow: 1.0,
                max_size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                aspect_ratio: Some(128.0 / 64.0),
                ..default()
            },
            ..default()
        })
        .insert(Screen)
        .insert(Name::new("Screen"));
}

fn ui_button_bar(parent: &mut ChildBuilder, mut asset_server: ResMut<AssetServer>) {
    parent
        .spawn(NodeBundle {
            background_color: Color::DARK_GRAY.into(),
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                size: Size::height(Val::Px(48.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| ui_icon_button(parent, "Play", &mut asset_server))
        .with_children(|parent| ui_icon_button(parent, "Pause", &mut asset_server));
}

fn ui_icon_button(parent: &mut ChildBuilder, name: &str, asset_server: &mut ResMut<AssetServer>) {
    let icon: Handle<Image> = asset_server.load(format!("buttons/{}.png", name.to_lowercase()));

    parent
        .spawn(ButtonBundle {
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::horizontal(Val::Px(3.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    image: UiImage::new(icon),
                    background_color: Color::ANTIQUE_WHITE.into(),
                    style: Style {
                        size: Size::all(Val::Px(48.0)),
                        ..default()
                    },
                    ..default()
                })
                .insert(Name::new(format!("{name} Button")));
        });
}

fn update_ui_screen(
    mut commands: Commands,
    query: Query<(Entity, &UiImage), With<Screen>>,
    crab8: Res<Crab8>,
    mut images: ResMut<Assets<Image>>,
) {
    if let Ok((entity, previous_frame)) = query.get_single() {
        let previous_texture = previous_frame.texture.clone();

        commands
            .entity(entity)
            .remove::<UiImage>()
            .insert(UiImage::new(
                images.add(screen::render_framebuffer(&crab8.screen)),
            ));

        images.remove(previous_texture);
    }
}
