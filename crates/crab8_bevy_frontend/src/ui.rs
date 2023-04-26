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

fn setup_ui(mut commands: Commands, crab8: Res<Crab8>, mut images: ResMut<Assets<Image>>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Start,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ui_screen(crab8, &mut images))
                .insert(Screen)
                .insert(Name::new("Screen"));
        });
}

fn ui_screen(crab8: Res<Crab8>, images: &mut ResMut<Assets<Image>>) -> ImageBundle {
    ImageBundle {
        image: UiImage::new(images.add(screen::render_framebuffer(&crab8.screen))),
        style: Style {
            flex_grow: 1.0,
            aspect_ratio: Some(128.0 / 64.0),
            ..default()
        },
        ..default()
    }
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
