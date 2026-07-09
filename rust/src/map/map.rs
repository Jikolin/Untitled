use godot::classes::GridMap;
use godot::prelude::*;

use crate::Player;
use crate::map::{DoorNode, Floor, RoomData};
use crate::utils::{AssetMap, get};

#[derive(GodotClass)]
#[class(base = Node, no_init)]
pub struct Map {
    base: Base<Node>,
    floors: Vec<Floor>,
    curr_floor: usize,
    curr_room: Option<RoomData>,
}

#[godot_api]
impl Map {
    #[func]
    pub fn create(width: i32, height: i32, floor_count: i32) -> Gd<Self> {
        let floor_size = (width as usize, height as usize);
        let mut floors = Vec::with_capacity(floor_count as usize);
        for _ in 0..floor_count {
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

    pub fn build_room(
        &mut self,
        assets: &AssetMap,
        coords: Vector2i,
        player: Gd<Player>,
    ) -> Gd<Node3D> {
        let mut room_node = get::<Node3D>(&assets, "room");
        self.mut_current_floor().prepare_enter_room(coords, player);
        let cell = self.mut_current_floor().get_mut_cell(coords).unwrap();

        let mut room_data = RoomData {
            coords,
            is_cleared: false,
            node: room_node.clone(),
            enemies: vec![],
            doors: vec![],
            items: vec![],
        };
        for enemy_data in cell.enemies.iter() {
            let enemy = enemy_data.turn_to_life(&assets);
            room_data.enemies.push(enemy.clone());
            room_node.add_child(&enemy);
        }
        for door_data in cell.doors.iter() {
            let door_node = DoorNode::new(&assets, door_data);
            room_data.doors.push(door_node.clone());
            room_node.add_child(&door_node);
        }
        for item_data in cell.items.iter() {
            let item_node = item_data.to_node(&assets, Vector3::new(3.0, 0.0, 2.0));
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
