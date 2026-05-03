use godot::prelude::*;
use godot::classes::{ Area3D, IArea3D, CollisionShape3D, MeshInstance3D, Input };

use crate::utils::{assets, load_scene_as};
use crate::player::Player;


#[derive(GodotClass)]
#[class(base=Area3D, no_init)]
pub struct Door {
	base: Base<Area3D>,
	is_colliding_player: bool
}


#[godot_api]
impl IArea3D for Door {
	fn ready(&mut self) {
		let callable = self.base().callable("on_body_entered");
        self.base_mut().connect("body_entered", &callable);
        self.base_mut().connect("body_exited", &callable);
	}

	fn physics_process(&mut self, _delta: f32) {
		let input = Input::singleton();
		if input.is_action_pressed("interact").into() 
		&& self.is_colliding_player {
			self.base_mut().emit_signal("exit_room", &[]);
		}
	}
}



#[godot_api]
impl Door {
	#[func]
	pub fn new(position: Vector3, rotation: Basis) -> Gd<Self> {
		let mut door = Gd::from_init_fn(|base| {
			Self { base, is_colliding_player: false }
		});
		let mesh = load_scene_as::<MeshInstance3D>(assets::DOOR_MESH);
		let shape = load_scene_as::<CollisionShape3D>(assets::DOOR_SHAPE);
		door.add_child(&mesh);
		door.add_child(&shape);
		door.set_position(position);
		door.set_basis(rotation);

		door
	}

	#[signal]
	fn exit_room();

	#[func]
	fn on_body_entered(&mut self, body: Gd<Node3D>) {
	    if body.try_cast::<Player>().is_ok() {
	        self.is_colliding_player = true;
	    }
	}

	#[func]
	fn on_body_exited(&mut self, body: Gd<Node3D>) {
	    if body.try_cast::<Player>().is_ok() {
	        self.is_colliding_player = false;
	    }
	}
}