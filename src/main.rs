use bevy::prelude::*;

use crate::plugins::window;

mod plugins;

fn main() {
    App::new().add_plugins((window::WindowHandling)).run();
}
