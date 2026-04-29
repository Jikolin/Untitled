use godot::prelude::*;
use godot::classes::{ Area3D, IArea3D, CollisionShape3D, MeshInstance3D };

use crate::utils::{assets, load_scene_as};


#[derive(GodotClass)]
#[class(base=Area3D, no_init)]
pub struct Door {
	base: Base<Area3D>,
}


#[godot_api]
impl Door {
	#[func]
	pub fn new(position: Vector3, rotation: Basis) -> Gd<Self> {
		let mut door = Gd::from_init_fn(|base| {
			Self { base }
		});
		let mesh = load_scene_as::<MeshInstance3D>(assets::DOOR_MESH);
		let shape = load_scene_as::<CollisionShape3D>(assets::DOOR_SHAPE);
		door.add_child(&mesh);
		door.add_child(&shape);

		door.set_position(position);
		// door.set_rotation(rotation);
		door.set_basis(rotation);

		door
	}
}