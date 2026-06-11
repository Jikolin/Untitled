use godot::classes::{GridMap, MeshLibrary};
use godot::prelude::*;
use rand::RngExt;

use crate::enemy::Goblin;
use crate::utils::{Dir2i, Dir3, assets, load_resource};

use crate::enemy::EnemyData;
use crate::enemy::{EnemyClass, enemy};
use crate::map::door::DoorData;
use crate::player::Player;

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
pub struct CellData {
    pub doors: Vec<DoorData>,
    pub enemies: Vec<EnemyData>,
}

#[derive(Default, Clone)]
pub struct Cell {
    pub c_type: CellType,
    pub is_visited: bool,
    pub data: CellData,
}

pub struct RoomData {
    pub coords: Vector2i,
    pub is_cleared: bool,
    pub enemies: Vec<Gd<Node3D>>,
    // pub scene: Gd<Node3D>,
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
        let mesh_lib = load_resource::<MeshLibrary>(assets::LABYRINTH_MESH_LIB);
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
        let mut doors: Vec<DoorData> = vec![];
        for dir in Dir3::all() {
            let dir2i = Vector2i::new(dir.x as i32, dir.z as i32);
            if let Some(CellType::Bridge { .. }) = self.get_cell_type(coords + dir2i) {
                doors.push(DoorData {
                    position: Vector3::new(dir.x * 4.9, 0.3, dir.z * 4.9),
                    rotation: match dir {
                        Dir3::UP | Dir3::DOWN => Basis::looking_at(Vector3::new(1.0, 0.0, 0.0)),
                        _ => Basis::default(),
                    },
                });
            }
        }

        let cell = match self.get_mut_cell(coords) {
            Some(cell) if !cell.is_visited => cell,
            _ => return,
        };
        cell.is_visited = true;
        cell.data = CellData {
            doors,
            enemies: vec![EnemyData {
                class: EnemyClass::Goblin,
                position: Vector3::default(),
                player,
                is_alive: true,
            }],
        };
    }

    pub fn prepare_leave_room(&mut self, room_data: RoomData) {
        if !room_data.is_cleared {
            let mut data = self.get_cell_mut_data(room_data.coords).unwrap();
            data.enemies.clear();
            for enemy in room_data.enemies.iter() {
                data.enemies.push(enemy.bind_mut().to_data());
            }
        } else {
            return;
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

    fn get_mut_cell(&mut self, coords: Vector2i) -> Option<&mut Cell> {
        if self.coords_is_ok(coords) {
            Some(&mut self.grid[coords.y as usize][coords.x as usize])
        } else {
            None
        }
    }

    pub fn get_cell_data(&self, coords: Vector2i) -> Option<&CellData> {
        if self.coords_is_ok(coords) {
            Some(&self.grid[coords.y as usize][coords.x as usize].data)
        } else {
            None
        }
    }

    fn get_cell_mut_data(&mut self, coords: Vector2i) -> Option<&mut CellData> {
        if self.coords_is_ok(coords) {
            Some(&mut self.grid[coords.y as usize][coords.x as usize].data)
        } else {
            None
        }
    }

    fn set_cell(&mut self, coords: Vector2i, c_type: CellType) {
        if self.coords_is_ok(coords) {
            self.grid[coords.y as usize][coords.x as usize].c_type = c_type;
        }
    }

    fn get_cell_state(&self, coords: Vector2i) -> bool {
        matches!(self.get_cell_type(coords), Some(CellType::Wall))
    }

    fn check_direction(&self, coords: Vector2i, dir: Vector2i) -> Option<Vector2i> {
        let new_coords = Vector2i {
            x: coords.x + dir.x,
            y: coords.y + dir.y,
        };
        if self.get_cell_state(new_coords) {
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
