use godot::prelude::*;


// #[derive(GodotClass)]
// #[class(base=Node3D, init)]
// struct MyNode {
//     base: Base<Node3D>
// }



struct MyExtension;


mod main_scene;
mod player;
mod map;


#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}