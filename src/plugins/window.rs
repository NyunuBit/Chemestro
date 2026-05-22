use bevy::prelude::*;

pub struct WindowHandling;

impl Plugin for WindowHandling {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Chemestro".to_owned(),
                ..Default::default()
            }),
            ..Default::default()
        }));
    }
}
