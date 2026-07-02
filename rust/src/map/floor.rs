use godot::classes::{GridMap, MeshLibrary};
use godot::prelude::*;
use rand::RngExt;

use crate::utils::{Dir2i, Dir3, load_resource};

use crate::Player;
use crate::enemy::{Enemy, EnemyClass, EnemyData};
use crate::entities::item::{Item, ItemData, ItemKind};
use crate::map::door::{DoorData, DoorNode};

#[derive(Default, Clone, Copy)]
pub enum CellType {
    #[default]
    Wall,
    Start,
    Room,
    Bridge {
        direction: Vector2i,
    },
}

#[derive(Default, Clone)]
pub struct Cell {
    pub c_type: CellType,
    pub is_visited: bool,
    pub items: Vec<ItemData>,
    pub enemies: Vec<EnemyData>,
    pub doors: Vec<DoorData>,
}

impl Cell {
    fn new(c_type: CellType, is_visited: bool) -> Cell {
        Cell {
            c_type,
            is_visited,
            items: vec![],
            enemies: vec![],
            doors: vec![],
        }
    }
}

#[derive(Clone)]
pub struct RoomData {
    pub coords: Vector2i,
    pub is_cleared: bool,
    pub node: Gd<Node3D>,
    pub enemies: Vec<Gd<Enemy>>,
    pub doors: Vec<Gd<DoorNode>>,
    pub items: Vec<Gd<Item>>,
}

#[derive(Clone)]
struct PossCoords {
    bridge: Vector2i,
    room: Vector2i,
    direction: Vector2i,
}

pub struct Floor {
    pub width: usize,
    pub height: usize,
    s_coords: Vector2i,
    grid: Vec<Vec<Cell>>,
    pub grid_map: Gd<GridMap>,
}

impl Floor {
    pub fn new(size: (usize, usize)) -> Self {
        let s_coords = Vector2i {
            x: rand::rng().random_range(0..size.0 as i32 - 1),
            y: rand::rng().random_range(0..size.1 as i32 - 1),
        };
        let grid = vec![vec![Cell::default(); size.1]; size.0];
        let mut floor = Floor {
            width: size.0,
            height: size.1,
            s_coords,
            grid,
            grid_map: GridMap::new_alloc(),
        };

        floor.generate_labyrinth();
        // floor.build_grid_map();
        floor
    }

    pub fn build_grid_map(&mut self) -> Gd<GridMap> {
        let mut grid_map = GridMap::new_alloc();
        grid_map.set_cell_size(Vector3::new(1.0, 0.4, 1.0));
        let mesh_lib = load_resource::<MeshLibrary>("res://assets/map/labyrinth.tres");
        grid_map.set_mesh_library(&mesh_lib);

        for y in 0..self.height {
            for x in 0..self.width {
                match self.get_cell_type(Vector2i::new(x as i32, y as i32)) {
                    Some(CellType::Room) | Some(CellType::Start) => {
                        grid_map.set_cell_item(Vector3i::new(x as i32, 0, y as i32), 0);
                    }
                    Some(CellType::Bridge { direction }) => {
                        let orientation = match direction {
                            Dir2i::UP | Dir2i::DOWN => 16,
                            Dir2i::RIGHT | Dir2i::LEFT => 10,
                            _ => 0,
                        };
                        grid_map
                            .set_cell_item_ex(Vector3i::new(x as i32, 1, y as i32), 1)
                            .orientation(orientation)
                            .done();
                    }
                    _ => {}
                }
            }
        }
        self.grid_map = grid_map.clone();
        grid_map
    }

    pub fn prepare_enter_room(&mut self, coords: Vector2i, player: Gd<Player>) {
        let doors: Vec<DoorData> = Dir3::all()
            .iter()
            .filter_map(|&dir3| {
                let dir2i = Vector2i::new(dir3.x as i32, dir3.z as i32);
                match self.get_cell_type(coords + dir2i) {
                    Some(CellType::Bridge { .. }) => Some(DoorData {
                        position: Vector3::new(dir3.x * 4.9, 0.3, dir3.z * 4.9),
                        rotation: match dir3 {
                            Dir3::UP | Dir3::DOWN => Basis::looking_at(Vector3::new(1.0, 0.0, 0.0)),
                            _ => Basis::default(),
                        },
                    }),
                    _ => None,
                }
            })
            .collect();

        let Some(cell) = self.get_mut_cell(coords) else {
            return;
        };
        if cell.is_visited {
            return;
        }
        cell.is_visited = true;
        cell.doors = doors;

        let enemy = EnemyData::new(EnemyClass::Goblin, Vector3::new(5.0, 0.0, 3.0), player);
        cell.enemies.push(enemy);

        let item = ItemData {
            slot_cost: 1,
            kind: ItemKind::Potion,
        };
        cell.items.push(item);
    }

    pub fn prepare_exit_room(&mut self, room_data: &mut RoomData) {
        let cell_data = self.get_mut_cell(room_data.coords).unwrap();
        cell_data.enemies.clear();
        for enemy in room_data.enemies.iter_mut() {
            cell_data.enemies.push(enemy.bind_mut().to_data());
        }
    }

    pub fn is_walkable(&self, coords: Vector2i) -> bool {
        if !self.coords_is_ok(coords) {
            return false;
        }
        matches!(self.get_cell_type(coords), Some(CellType::Bridge { .. }))
    }

    pub fn get_start_coords(&self) -> Vector2i {
        self.s_coords
    }
}

impl Floor {
    fn coords_is_ok(&self, coords: Vector2i) -> bool {
        if coords.x >= 0
            && coords.y >= 0
            && coords.x < self.width as i32
            && coords.y < self.height as i32
        {
            true
        } else {
            false
        }
    }

    fn get_cell_type(&self, coords: Vector2i) -> Option<CellType> {
        if self.coords_is_ok(coords) {
            Some(
                self.grid[coords.y as usize][coords.x as usize]
                    .c_type
                    .clone(),
            )
        } else {
            None
        }
    }

    pub fn get_mut_cell(&mut self, coords: Vector2i) -> Option<&mut Cell> {
        if self.coords_is_ok(coords) {
            Some(&mut self.grid[coords.y as usize][coords.x as usize])
        } else {
            None
        }
    }

    fn set_cell(&mut self, coords: Vector2i, c_type: CellType) {
        if self.coords_is_ok(coords) {
            self.grid[coords.y as usize][coords.x as usize].c_type = c_type;
        }
    }

    fn cell_is_free(&self, coords: Vector2i) -> bool {
        matches!(self.get_cell_type(coords), Some(CellType::Wall))
    }

    fn check_direction(&self, coords: Vector2i, dir: Vector2i) -> Option<Vector2i> {
        let new_coords = Vector2i {
            x: coords.x + dir.x,
            y: coords.y + dir.y,
        };
        if self.cell_is_free(new_coords) {
            Some(new_coords)
        } else {
            None
        }
    }

    fn get_poss_coords(&self, coords: Vector2i) -> Vec<PossCoords> {
        let mut poss_coords = vec![];
        for dir in Dir2i::all() {
            if let Some(bridge) = self.check_direction(coords, dir) {
                if let Some(room) = self.check_direction(bridge, dir) {
                    poss_coords.push(PossCoords {
                        bridge,
                        room,
                        direction: dir,
                    });
                }
            }
        }
        poss_coords
    }
}

impl Floor {
    fn generate_labyrinth(&mut self) {
        let mut curr_coords = self.s_coords;
        self.set_cell(curr_coords, CellType::Start);
        let mut created_cells: Vec<Vector2i> = vec![curr_coords];

        for _ in 0..30 {
            let way_length: usize = rand::rng().random_range(1..5);
            for _ in 0..way_length {
                let poss_coords = self.get_poss_coords(curr_coords);
                if poss_coords.is_empty() {
                    let indx = rand::rng().random_range(0..created_cells.len());
                    curr_coords = created_cells[indx];
                    continue;
                }
                let indx = rand::rng().random_range(0..poss_coords.len());
                let next = &poss_coords[indx];
                self.set_cell(
                    next.bridge,
                    CellType::Bridge {
                        direction: next.direction,
                    },
                );
                self.set_cell(next.room, CellType::Room);
                curr_coords = next.room;
                created_cells.push(curr_coords);
            }
        }
    }
}
