use godot::prelude::*;
use godot::classes::{ 
	CharacterBody3D, ICharacterBody3D, 
 	CollisionShape3D, MeshInstance3D, Input
};


use crate::map::MapLayer;
use crate::utils::{assets, load_scene, load_inst_as};


struct Direction;
impl Direction {
    const UP: Vector3    = Vector3 { x: 0.0, y: 0.0, z: -1.0 };
    const RIGHT: Vector3 = Vector3 { x: 1.0, y: 0.0, z: 0.0 };
    const DOWN: Vector3  = Vector3 { x: 0.0, y: 0.0, z: 1.0 };
    const LEFT: Vector3  = Vector3 { x: -1.0, y: 0.0, z: 0.0 };
}


#[derive(GodotClass)]
#[class(base=CharacterBody3D, no_init)]
pub struct Player {
	base: Base<CharacterBody3D>,
	map: Gd<MapLayer>,
	// mesh: Gd<MeshInstance3D>,
	// collision: Gd<CollisionShape3D>,

	is_moving: bool,
	is_in_the_room: bool,

	step_speed: f32,
	step_size: f32,
	walk_speed: f32,

	target_velocity: Vector3,
	target_pos: Vector3,
	target_rot: Basis,	
}


#[godot_api]
impl ICharacterBody3D for Player {
	fn ready(&mut self) {
		let mesh = load_inst_as::<MeshInstance3D>(assets::PLAYER_MESH);
		let shape = load_inst_as::<CollisionShape3D>(assets::PLAYER_SHAPE);

		let map = self.map.clone();

		self.base_mut().set_position(map.bind().get_start_position());
		self.target_pos = self.base().get_position();

		self.base_mut().add_child(&mesh);
		self.base_mut().add_child(&shape);
	}


	fn physics_process(&mut self, delta: f32) {
		self.check_input();

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


#[godot_api]
impl Player {
	pub fn new(map: Gd<MapLayer>) -> Gd<Self> {
		// let model = load::<PackedScene>("res://scenes/player.tscn")
		// 	.instantiate_as::<Node3D>();

		Gd::from_init_fn(|base| {
			Self {
				base,
				map,
				// model,

				is_moving: false,
				is_in_the_room: false,

				step_speed: 5.0,
				step_size: 2.0,
				walk_speed: 3.5,

				target_velocity: Vector3::ZERO,
				target_pos: Vector3::ZERO,
				target_rot: Basis::default(),
			}
		})
	}


	fn check_input(&mut self) {
        let input = Input::singleton();

        if self.is_in_the_room {

        } else {
        	if input.is_action_just_pressed("ui_up").into() {
	        	self.try_to_move(Direction::UP);
	        } else if input.is_action_just_pressed("ui_right").into() {
	        	self.try_to_move(Direction::RIGHT);
	        } else if input.is_action_just_pressed("ui_down").into() {
	        	self.try_to_move(Direction::DOWN);
	        } else if input.is_action_just_pressed("ui_left").into() {
	        	self.try_to_move(Direction::LEFT);
	        }

	        if input.is_action_just_pressed("interact").into() {

	        }	
        }
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
