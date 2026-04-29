use godot::prelude::*;
use godot::classes::{ GridMap };

use crate::player::Player;
use crate::map::MapLayer;



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
		let player = self.player.clone();
		let grid = self.map_grid.clone();

		self.base_mut().add_child(&player);
		self.base_mut().add_child(&grid);
	}
}


#[godot_api]
impl MainScene {
	#[func]
	pub fn enter_room(&mut self, coords: Vector2i) {
		let room = self.map.bind_mut().build_room(coords);
		self.map_grid.set_visible(false);
		self.base_mut().add_child(&room);
		self.player.bind_mut().enter_room();
	}
}