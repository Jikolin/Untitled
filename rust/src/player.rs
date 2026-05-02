use godot::prelude::*;
use godot::classes::{ 
	CharacterBody3D, ICharacterBody3D, 
 	CollisionShape3D, MeshInstance3D, Input
};

use crate::map::MapLayer;
use crate::utils::{assets, load_scene_as, Dir3};


#[derive(GodotClass)]
#[class(base=CharacterBody3D, no_init)]
pub struct Player {
	base: Base<CharacterBody3D>,
	mesh: Gd<MeshInstance3D>,
	shape: Gd<CollisionShape3D>,
	map: Gd<MapLayer>,

	is_moving: 		bool,
	pub is_in_the_room: bool,

	step_speed: f32,
	step_size:  f32,
	walk_speed: f32,

	target_velocity: Vector3,
	target_pos: 	 Vector3,
	buff_target_pos: Vector3,
	// target_rot: 	 Basis,	
	// target_rot_grid: Basis,
	grid_position:   Vector3,
}


#[godot_api]
impl ICharacterBody3D for Player {
	fn ready(&mut self) {
		let mesh = self.mesh.clone();
		let shape = self.shape.clone();

		let start_position = self.map.bind().get_start_position();
		self.base_mut().set_position(start_position);
		self.target_pos = self.base().get_position();

		self.base_mut().add_child(&mesh);
		self.base_mut().add_child(&shape);
	}


	fn physics_process(&mut self, delta: f32) {
		self.check_input();

		if self.is_in_the_room {
			let velocity = self.target_velocity * self.walk_speed;
			self.base_mut().set_velocity(velocity);
			self.base_mut().move_and_slide();
		} else if self.is_moving {
	            let cur_pos = self.base_mut().get_position();
	            let targ_pos = self.target_pos;

	            let new_pos = cur_pos.move_toward(targ_pos, &self.step_speed * delta);
	            self.base_mut().set_position(new_pos);

	            if cur_pos.distance_to(targ_pos) < 0.01 {
	                self.base_mut().set_position(targ_pos);
	                if self.buff_target_pos != Vector3::ZERO {
	                	self.target_pos = self.buff_target_pos;
	                	self.buff_target_pos = Vector3::ZERO;
	                } else {
	                	self.is_moving = false;
	                }
	            }
		} 
	}
}


#[godot_api]
impl Player {
	pub fn new(map: Gd<MapLayer>) -> Gd<Self> {
		Gd::from_init_fn(|base| {
			Self {
				base,
				mesh:  load_scene_as::<MeshInstance3D>(assets::PLAYER_MESH),
				shape: load_scene_as::<CollisionShape3D>(assets::PLAYER_SHAPE),
				map,

				is_moving: 		false,
				is_in_the_room: false,

				step_speed: 5.0,
				step_size: 	2.0,
				walk_speed: 5.5,

				target_velocity: Vector3::ZERO,
				target_pos: 	 Vector3::ZERO,
				buff_target_pos: Vector3::ZERO,
				// target_rot: 	 Basis::default(),
				// target_rot_grid: Basis::default(),
				grid_position:   Vector3::ZERO,
			}
		})
	}

	#[signal]
	fn enter_room(coords: Vector2i);


	fn check_input(&mut self) {
        let input = Input::singleton();

        if self.is_in_the_room {
        	if input.is_action_pressed("ui_up").into() {
	        	self.target_velocity.z -= 1.0;
	        } else if input.is_action_pressed("ui_down").into() {
	        	self.target_velocity.z += 1.0;
	        } else {
	        	self.target_velocity.z = 0.0;
	        }

	        if input.is_action_pressed("ui_right").into() {
	        	self.target_velocity.x += 1.0;
	        } else if input.is_action_pressed("ui_left").into() {
	        	self.target_velocity.x -= 1.0;
	        } else {
	        	self.target_velocity.x = 0.0;
	        }

			if self.target_velocity != Vector3::ZERO{
				if self.target_velocity.x > 0.0 && self.target_velocity.z > 0.0 {
					// Moderate formula
					self.target_velocity.x = self.target_velocity.x / 2.0;
					self.target_velocity.z = self.target_velocity.z / 2.0;
				}
				self.target_velocity = self.target_velocity.normalized();
				// self.target_rot = Basis::looking_at(self.target_velocity);
			}

        } else {
        	if input.is_action_just_pressed("ui_up").into() {
	        	self.try_to_move(Dir3::UP);
	        } else if input.is_action_just_pressed("ui_right").into() {
	        	self.try_to_move(Dir3::RIGHT);
	        } else if input.is_action_just_pressed("ui_down").into() {
	        	self.try_to_move(Dir3::DOWN);
	        } else if input.is_action_just_pressed("ui_left").into() {
	        	self.try_to_move(Dir3::LEFT);
	        }

	        if input.is_action_just_pressed("interact").into() {
	        	let pos = self.base().get_position();
	        	let coords = Vector2i::new((pos.x - 0.5) as i32, (pos.z - 0.5) as i32);
	        	self.enter_room();
	        	self.base_mut().emit_signal("enter_room", &[coords.to_variant()]);
	        }	
        }
    }

    #[func]
    pub fn enter_room(&mut self) {
		self.is_in_the_room = true;
		let position = self.base().get_position();
		let position = Vector3::new(position.x, 1.0, position.z);
		self.grid_position = position;
		self.target_pos = position;
		self.base_mut().set_position(Vector3::new(0.0, 1.0, 0.0));
	}

	#[func]
	pub fn exit_room(&mut self) {
		self.is_in_the_room = false;
		let grid_position = self.grid_position;
		self.base_mut().set_position(grid_position);
	}


    fn try_to_move(&mut self, direction: Vector3) {
    	let curr_position = self.base().get_position();
    	if !self.is_moving && self.move_is_possible(curr_position, direction) {
    		self.is_moving = true;
    		self.target_pos += direction * self.step_size;
    	} else if self.move_is_possible(self.target_pos, direction) {
    		self.buff_target_pos = self.target_pos + direction * self.step_size;
    	}
    }


    fn move_is_possible(&self, position: Vector3, direction: Vector3) -> bool {
    	let new_position = Vector2i::new(
    		(position.x - 0.5 + direction.x) as i32,
			(position.z - 0.5 + direction.z) as i32 );

    	self.map.bind().is_walkable(new_position)
    }
}
