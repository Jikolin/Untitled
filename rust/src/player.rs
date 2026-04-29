use godot::prelude::*;
use godot::classes::{ 
	CharacterBody3D, ICharacterBody3D, 
 	CollisionShape3D, MeshInstance3D, Input
};

use crate::map::MapLayer;
use crate::main_scene::MainScene;
use crate::utils::{assets, load_scene, load_scene_as, Dir3};


#[derive(GodotClass)]
#[class(base=CharacterBody3D, no_init)]
pub struct Player {
	base: Base<CharacterBody3D>,
	mesh: Gd<MeshInstance3D>,
	shape: Gd<CollisionShape3D>,
	map: Gd<MapLayer>,

	is_moving: 		bool,
	is_in_the_room: bool,

	step_speed: f32,
	step_size:  f32,
	walk_speed: f32,

	target_velocity: Vector3,
	target_pos: 	 Vector3,
	target_rot: 	 Basis,	
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
			let input = Input::singleton();
		    let mut velocity = Vector3::ZERO;

		    if input.is_action_pressed("ui_up") {
		        velocity.z -= 1.0;
		    }
		    if input.is_action_pressed("ui_down") {
		        velocity.z += 1.0;
		    }
		    if input.is_action_pressed("ui_left") {
		        velocity.x -= 1.0;
		    }
		    if input.is_action_pressed("ui_right") {
		        velocity.x += 1.0;
		    }

		    if velocity != Vector3::ZERO {
		        velocity = velocity.normalized() * self.walk_speed * delta;
		    }

		    self.base_mut().set_velocity(velocity);
		    self.base_mut().move_and_slide();

		} else {
			if self.is_moving{
	            let cur_pos = self.base_mut().get_position();
	            let targ_pos = self.target_pos;

	            let new_pos = cur_pos.move_toward(targ_pos, &self.step_speed * delta);
	            self.base_mut().set_position(new_pos);

	            if cur_pos.distance_to(targ_pos) < 0.01 {
	                self.base_mut().set_position(targ_pos);
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
				walk_speed: 3.5,

				target_velocity: Vector3::ZERO,
				target_pos: 	 Vector3::ZERO,
				target_rot: 	 Basis::default(),
			}
		})
	}


	fn check_input(&mut self) {
        let input = Input::singleton();

        if self.is_in_the_room {


        } else if !self.is_moving {
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
	        	let mut main = self.base()
				    .get_tree()
				    .get_root().unwrap()
				    .get_node_as::<MainScene>("MyNode");
				let coords = self.base().get_position();
				let coords = Vector2i::new((coords.x - 0.5) as i32, (coords.z - 0.5) as i32);
	        	main.bind_mut().enter_room(coords);
	        }	
        }
    }

    #[func]
    pub fn enter_room(&mut self) {
		self.is_in_the_room = true;
		// self.base_mut().set_position(Vector3::new(0.0, 1.0, 0.0));
	}


    fn try_to_move(&mut self, direction: Vector3) {
    	if self.move_is_possible(direction) && !self.is_moving {
    		self.is_moving = true;
    		self.target_pos += direction * self.step_size;
            // self.target_rot = Basis::looking_at(dir, Vector3::UP, true);
    	}
    }


    fn move_is_possible(&self, direction: Vector3) -> bool {
    	let curr_position = self.base().get_position();
    	let new_position = Vector2i::new(
    		(curr_position.x - 0.5 + direction.x) as i32,
			(curr_position.z - 0.5 + direction.z) as i32 );

    	self.map.bind().is_walkable(new_position)
    }
}
