use godot::prelude::*;
use godot::classes::{ Area3D, IArea3D, Input, MeshInstance3D, CollisionShape3D};

use crate::player::Player;


#[derive(GodotClass)]
#[class(base=Area3D, no_init)]
pub struct Door {
	base: Base<Area3D>,
	player: Gd<Player>,
}


#[godot_api]
impl IArea3D for Door {
	fn ready(&mut self) {
		let mesh = load::<PackedScene>("res://assets/door_mesh.tscn")
			.instantiate_as::<MeshInstance3D>();
		let shape = load::<PackedScene>("res://assets/door_shape.tscn")
			.instantiate_as::<CollisionShape3D>();

		self.base_mut().add_child(&mesh);
		self.base_mut().add_child(&shape);
	}

	fn process(&mut self, _delta: f32) {
		if Input::singleton().is_action_just_pressed("interact") {
			let mut player = self.player.clone();
			if self.base().overlaps_body(&player) {
				player.bind_mut().exit_room();
			}
		}
	}
}


#[godot_api]
impl Door {
	pub fn new(player: Gd<Player>, position: Vector3) -> Gd<Self> {
		let mut door = Gd::from_init_fn(|base| {
			Self {
				base,
				player
			}
		});
		door.set_position(position);
		door
	}
}