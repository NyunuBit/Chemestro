mod plugins;
mod prelude;

use plugins::window;
use prelude::*;

fn main() {
    App::new().add_plugins(window::WindowHandling).run();
}
