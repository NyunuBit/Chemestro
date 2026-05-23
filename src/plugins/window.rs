use bevy::{prelude::*, window::WindowTheme};

pub struct WindowHandling;

impl Plugin for WindowHandling {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Chemestro".to_owned(),
                name: Some("com.nyunu.chemestro".to_owned()),
                window_theme: Some(WindowTheme::Dark),
                ..Default::default()
            }),
            ..Default::default()
        }));
    }
}
