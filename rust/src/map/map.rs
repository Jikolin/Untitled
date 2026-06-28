use godot::classes::GridMap;
use godot::prelude::*;

use crate::map::{DoorNode, Floor, RoomData};
use crate::utils::load_scene_as;
use crate::{Item, Player};

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
        let mut room_node = load_scene_as::<Node3D>("res://scenes/room.tscn");
        self.mut_current_floor().prepare_enter_room(coords, player);
        let cell_data = self.current_floor().get_cell_data(coords).unwrap();

        let mut room_data = RoomData {
            coords,
            is_cleared: false,
            node: room_node.clone(),
            enemies: vec![],
            doors: vec![],
            items: vec![],
        };
        for enemy_data in cell_data.enemies.iter() {
            let enemy = enemy_data.turn_to_life();
            room_data.enemies.push(enemy.clone());
            room_node.add_child(&enemy);
        }
        for door_data in cell_data.doors.iter() {
            let door_node = DoorNode::new(door_data);
            room_data.doors.push(door_node.clone());
            room_node.add_child(&door_node);
        }
        for item_data in cell_data.items.iter() {
            let item_node = Item::new(
                item_data.kind.clone(),
                Vector3::new(coords.x as f32 - 4.0, 0.0, coords.y as f32),
            );
            room_data.items.push(item_node.clone());
            room_node.add_child(&item_node);
        }

        self.curr_room = Some(room_data);
        room_node
    }
    pub fn room_is_cleared(&self) -> bool {
        if let Some(roomdata) = &self.curr_room {
            return roomdata.is_cleared;
        } else {
            return false;
        }
    }
    pub fn exit_room(&mut self) {
        let mut room_data = self.curr_room.clone().unwrap();
        self.mut_current_floor().prepare_exit_room(&mut room_data);
        room_data.node.queue_free();
    }

    pub fn try_exit_room(&self) -> bool {
        if let Some(curr_room) = &self.curr_room {
            for door in curr_room.doors.iter() {
                if door.bind().player_is_colliding() {
                    return true;
                }
            }
        }
        false
    }
}
