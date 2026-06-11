use godot::prelude::*;

struct MyExtension;

mod entities;
pub use entities::{enemy, player};
pub use player::Player;
mod main_scene;
mod map;
mod utils;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
