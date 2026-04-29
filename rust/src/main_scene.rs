use godot::prelude::*;
use godot::classes::{ Area3D };

use crate::player::Player;
use crate::map::MapLayer;



#[derive(GodotClass)]
#[class(base=Node3D)]
struct MainScene {
	base: Base<Node3D>,

	player: Gd<Player>,
	map: Gd<MapLayer>,
}


#[godot_api]
impl INode3D for MainScene {
	fn init(base: Base<Node3D>) -> Self {
		let map = MapLayer::new(10, 10);
		let player = Player::new(map.clone());

		Self {
			base,
			player,
			map,
		}
	}

	fn ready(&mut self) {
		let player = self.player.clone();
		let grid = self.map.bind_mut().build_grid_map();

		self.base_mut().add_child(&player);
		self.base_mut().add_child(&grid);

		// let door = Door::new(player.clone(), map.bind().get_start_position());
		// self.base_mut().add_child(&door);
	}

	// fn physics_process(&mut self, _delta: f32) {

	// }
}


// impl MainScene {
// 	fn enter_room(&self, coords: Vector2i) {

// 	}
// }