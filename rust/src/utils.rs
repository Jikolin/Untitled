use godot::prelude::*;



#[warn(nonstandard_style)]
pub mod assets {
	pub const PLAYER_MESH: &str =  	"res://assets/player_mesh.tscn";
	pub const PLAYER_SHAPE: &str = 	"res://assets/player_shape.tscn";
	pub const DOOR_MESH: &str =    	"res://assets/door_mesh.tscn";
	pub const DOOR_SHAPE: &str =    "res://assets/door_shape.tscn";
	pub const ROOM: &str = 			"res://scenes/room.tscn";
}


pub fn load_scene(path: &str) -> Gd<PackedScene> {
	load::<PackedScene>(path)
}


pub fn load_inst_as<T>(path: &str) -> Gd<T>
where
    T: Inherits<Node>,
{
    load::<PackedScene>(path).instantiate_as::<T>()
}