mod plugins;
mod prelude;

use plugins::{game_state, window};
use prelude::*;

fn main() {
    App::new()
        .add_plugins((window::WindowHandlingPlugin, game_state::GameStatePlugin))
        .run();
}
