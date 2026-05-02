use godot::prelude::*;
use godot::classes::{ GridMap, Camera3D };

use crate::player::Player;
use crate::map::MapLayer;
use crate::door::Door;



#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct MainScene {
	base: Base<Node3D>,

	player: Gd<Player>,
	map: Gd<MapLayer>,
	map_grid: Gd<GridMap>,
}


#[godot_api]
impl INode3D for MainScene {
	fn init(base: Base<Node3D>) -> Self {
		let map = MapLayer::new(10, 10);
		let map_grid = map.bind().build_grid_map();
		let player = Player::new(map.clone());

		Self {
			base,
			player,
			map,
			map_grid,
		}
	}

	fn ready(&mut self) {
		let mut player = self.player.clone();
		let grid = self.map_grid.clone();

		self.base_mut().add_child(&player);
		player.connect("enter_room", &self.to_gd().callable("enter_room"));
		self.base_mut().add_child(&grid);
	}

	fn physics_process(&mut self, delta: f32) {
		// Smooth camera movement
		let mut camera = self.base().try_get_node_as::<Camera3D>("Camera3D").unwrap();
		let player_pos = self.player.get_position();
		let cam_pos = camera.get_position();
		let target = Vector3::new(player_pos.x, 2.5, player_pos.z + 3.0);
		let new_pos = cam_pos.lerp(target, 3.5 * delta);
		camera.set_position(new_pos);
	}
}


#[godot_api]
impl MainScene {
	#[func]
	pub fn enter_room(&mut self, coords: Vector2i) {
		let room = self.map.bind_mut().build_room(coords);
		for child in room.get_children().iter_shared() {
	        if let Ok(mut door) = child.try_cast::<Door>() {
	            door.connect("exit_room", &self.to_gd().callable("exit_room"));
	        }
	    }
		self.map_grid.set_visible(false);
		self.base_mut().add_child(&room);
	}

	#[func]
	pub fn exit_room(&mut self) {
		self.player.bind_mut().exit_room();
		self.map_grid.set_visible(true);
		if let Some(mut room) = self.base().try_get_node_as::<Node3D>("Room") {
		    room.queue_free();
		}
	}
}