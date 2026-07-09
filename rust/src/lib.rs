use godot::prelude::*;

struct MyExtension;

mod entities;
pub use entities::{enemy, item::Item, player::Player};

mod map;

mod utils;
pub use utils::*;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
