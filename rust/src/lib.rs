use godot::prelude::*;


struct MyExtension;


mod main_scene;
mod map;
mod player;
mod utils;
mod door;



#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}