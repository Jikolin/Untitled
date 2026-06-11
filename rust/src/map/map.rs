use godot::classes::GridMap;
use godot::prelude::*;

use crate::map::{DoorNode, Floor, RoomData};
use crate::player::Player;
use crate::utils::load_scene_as;

#[derive(GodotClass)]
#[class(base = Node, no_init)]
pub struct Map {
    base: Base<Node>,
    floors: Vec<Floor>,
    curr_floor: usize,
    curr_room: Option<RoomData>,
}

impl Map {
    pub fn new(floor_size: (usize, usize), flour_count: usize) -> Gd<Self> {
        let mut floors = Vec::with_capacity(flour_count);
        for _ in 0..flour_count {
            floors.push(Floor::new(floor_size));
        }

        Gd::from_init_fn(|base| Self {
            base,
            floors,
            curr_floor: 0,
            curr_room: None,
        })
    }

    pub fn current_floor(&self) -> &Floor {
        &self.floors[self.curr_floor]
    }

    fn mut_current_floor(&mut self) -> &mut Floor {
        &mut self.floors[self.curr_floor]
    }

    pub fn build_grid_map(&mut self) -> Gd<GridMap> {
        self.mut_current_floor().build_grid_map()
    }

    pub fn is_walkable(&self, coords: Vector2i) -> bool {
        self.current_floor().is_walkable(coords)
    }

    pub fn get_start_position(&self) -> Vector3 {
        let coords = self.current_floor().get_start_coords();
        Vector3::new(coords.x as f32 + 0.5, 1.2, coords.y as f32 + 0.5)
    }

    pub fn build_room(&mut self, coords: Vector2i, player: Gd<Player>) -> Gd<Node3D> {
        let mut room = load_scene_as::<Node3D>("res://scenes/room.tscn");
        let mut room_data = RoomData {
            coords,
            is_cleared: false,
            enemies: Vec::new(),
        };
        self.mut_current_floor().prepare_enter_room(coords, player);
        let data = self.current_floor().get_cell_data(coords).unwrap().clone();
        if !data.enemies.is_empty() {
            for enemy_data in &data.enemies {
                let enemy = enemy_data.turn_to_life();
                room.add_child(&enemy);
                room_data.enemies.push(enemy.clone());
            }
        }
        for door_data in &data.doors {
            let door = DoorNode::new(door_data);
            room.add_child(&door);
        }

        self.curr_room = Some(room_data);
        room
    }

    pub fn leave_room(&mut self) {
        let mut room_data = self.curr_room.clone().unwrap();
        self.mut_current_floor().prepare_leave_room(&mut room_data);
        self.curr_room = None;
    }
}
