use bevy::{prelude::*, window::WindowTheme};

pub struct WindowHandlingPlugin;

impl Plugin for WindowHandlingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Chemestro - v{}", env!("CARGO_PKG_VERSION")).to_owned(),
                name: Some("com.nyunu.chemestro".to_owned()),
                window_theme: Some(WindowTheme::Dark),
                ..Default::default()
            }),
            ..Default::default()
        }));
    }
}
