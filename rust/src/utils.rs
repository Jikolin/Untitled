use godot::prelude::*;


pub mod assets {
	pub const PLAYER_MESH:		  &str = "res://assets/player_mesh.tscn";
	pub const PLAYER_SHAPE: 	  &str = "res://assets/player_shape.tscn";
	pub const DOOR_MESH: 		  &str = "res://assets/door_mesh.tscn";
	pub const DOOR_SHAPE:   	  &str = "res://assets/door_shape.tscn";
	pub const ROOM: 			  &str = "res://scenes/room.tscn";
	pub const LABYRINTH_MESH_LIB: &str = "res://assets/labyrinth.tres";
}


pub fn load_resource<T>(path: &str) -> Gd<T> 
where 
	T: Inherits<Resource>,
{
	load::<T>(path)
}


pub fn load_scene(path: &str) -> Gd<PackedScene> {
	load::<PackedScene>(path)
}


pub fn load_scene_as<T>(path: &str) -> Gd<T>
where
    T: Inherits<Node>,
{
    load::<PackedScene>(path).instantiate_as::<T>()
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dir2i;
impl Dir2i {
    pub const UP:    Vector2i = Vector2i { x: 0,  y: -1 };
    pub const RIGHT: Vector2i = Vector2i { x: 1,  y: 0  };
    pub const DOWN:  Vector2i = Vector2i { x: 0,  y: 1  };
    pub const LEFT:  Vector2i = Vector2i { x: -1, y: 0  };
    pub const NULL:  Vector2i = Vector2i { x: 0,  y: 0  };

    pub fn all() -> [Vector2i; 4] {
        [Self::UP, Self::RIGHT, Self::DOWN, Self::LEFT]
    }
}

#[derive(Clone, Copy)]
pub struct Dir3;
impl Dir3 {
    pub const UP:    Vector3 = Vector3 { x: 0.0, y: 0.0, z: -1.0 };
    pub const RIGHT: Vector3 = Vector3 { x: 1.0, y: 0.0, z: 0.0  };
    pub const DOWN:  Vector3 = Vector3 { x: 0.0, y: 0.0, z: 1.0  };
    pub const LEFT:  Vector3 = Vector3 { x: -1.0, y: 0.0, z: 0.0 };

    pub fn all() -> [Vector3; 4] {
        [Self::UP, Self::RIGHT, Self::DOWN, Self::LEFT]
    } 
}
