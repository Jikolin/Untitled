use godot::prelude::*;

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
		let mut player = self.player.clone();
		let mut map = self.map.clone();

		self.base_mut().add_child(&player);
		self.base_mut().add_child(&map.bind().build_grid_map());

		player.set_position(map.bind().get_start_position());
	}
}