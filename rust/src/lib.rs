use godot::prelude::*;


struct MyExtension;


mod main_scene;
mod map;
mod player;
mod utils;



#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}