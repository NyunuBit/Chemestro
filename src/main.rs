mod plugins;

use bevy::prelude::*;
use plugins::{entry, window};

fn main() {
    App::new()
        .add_plugins(entry::BasePlugin)
        .add_plugins((window::WindowHandlingPlugin))
        .run();
}
