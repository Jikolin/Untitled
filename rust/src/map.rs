use godot::prelude::*;
use godot::classes::{ GridMap, MeshLibrary };

use rand::RngExt;

use crate::utils::{assets, load_resource, load_scene_as, Dir2i, Dir3};
use crate::door::Door;


#[derive(Clone, Debug)]
enum CellType { 
    Wall, 
    Room, 
    Bridge { direction: Vector2i }, 
    Start 
}

#[derive(Clone)]
struct CellData {
    enter: Vector3,
    doors: [Vector3; 4],
}

#[derive(Clone)]
struct Cell {
    c_type: CellType,
    data: Option<CellData>
}

#[derive(Clone)]
struct PossCoords {
    bridge: Vector2i,
    room:   Vector2i,
    direction: Vector2i
}


#[derive(GodotClass)]
#[class(base=Node, no_init)]
pub struct MapLayer {
    base:     Base<Node>,
    width:    usize,
    height:   usize,
    s_coords: Vector2i,
    grid:     Vec<Vec<Cell>>,
}


#[godot_api]
impl MapLayer {
    pub fn new(width: usize, height: usize) -> Gd<Self> {
        let mut map = Gd::from_init_fn(|base| {
            Self {
                base,
                width,
                height,
                s_coords: Vector2i {
                    x: rand::rng().random_range(1..width  as i32 - 1),
                    y: rand::rng().random_range(1..height as i32 - 1),
                },
                grid: vec![vec![Cell { c_type: CellType::Wall, data: None }; width]; height],
            }
        });
        map.bind_mut().generate();
        map
    }

    #[func]
    pub fn print_grid(&self) {
        for row in &self.grid {
            for cell in row {
                match cell.c_type {
                    CellType::Wall           => print!("#"),
                    CellType::Room           => print!("@"),
                    CellType::Bridge { .. }  => print!("-"),
                    CellType::Start          => print!("S"),
                }
            }
            println!();
        }
    }

    #[func]
    pub fn generate(&mut self) {
        self.generate_labyrinth();
    }

    #[func]
    pub fn build_grid_map(&self) -> Gd<GridMap> {
        let mut grid_map = GridMap::new_alloc();
        let mesh_lib = load_resource::<MeshLibrary>(assets::LABYRINTH_MESH_LIB);
        grid_map.set_cell_size(Vector3::new(1.0, 0.4, 1.0));
        grid_map.set_mesh_library(&mesh_lib);

        for y in 0..self.height {
            for x in 0..self.width {
                match self.get_cell_type(Vector2i::new(x as i32, y as i32)) {
                    Some(CellType::Room) | Some(CellType::Start) => {
                        grid_map.set_cell_item(Vector3i::new(x as i32, 0, y as i32), 0);
                    },
                    Some(CellType::Bridge { direction }) => {
                        let orientation = match direction {
                            Dir2i::UP | Dir2i::DOWN   => 16,
                            Dir2i::RIGHT | Dir2i::LEFT => 10,
                            _ => 0
                        };
                        grid_map.set_cell_item_ex(Vector3i::new(x as i32, 1, y as i32), 1)
                            .orientation(orientation)
                            .done();
                    },
                    _ => {}
                }
            }
        }
        grid_map
    }

    #[func]
    pub fn build_room(&mut self, coords: Vector2i) -> Gd<Node3D> {
        let mut room = load_scene_as::<Node3D>(assets::ROOM);
        self.place_doors(coords, &mut room);
        room
    }

    #[func]
    pub fn is_walkable(&self, coords: Vector2i) -> bool {
        if !self.coords_is_ok(coords) {
            return false;
        }
        // matches!(self.get_cell_type(coords), Some(CellType::Room) | Some(CellType::Start) | Some(CellType::Bridge { .. }))
        matches!(self.get_cell_type(coords), Some(CellType::Bridge { .. }))

    }

	// Player-oriented
    #[func]
    pub fn get_start_position(&self) -> Vector3 {
        Vector3::new(self.s_coords.x as f32 + 0.5, 1.2, self.s_coords.y as f32 + 0.5)
    }

    // For entering start room. Not sure
    // #[func]
    // pub fn get_start_coords(&self) -> Vector3 {
    //     Vector3::new(self.s_coords.x as f32, 1.0, self.s_coords.y as f32)
    // }

    fn place_doors(&self, coords: Vector2i, room: &mut Gd<Node3D>){
        for dir in Dir3::all() {
            let dir2i = Vector2i::new(dir.x as i32, dir.z as i32);
            if let Some(CellType::Bridge { .. }) = self.get_cell_type(coords + dir2i) {
                let position = Vector3::new(dir.x * 4.9, 0.3, dir.z * 4.9);
                let rotation = match dir {
                    Dir3::UP | Dir3::DOWN => Basis::looking_at(Vector3::new(1.0, 0.0, 0.0)),
                    _ => Basis::default()
                };
                let door = Door::new(position, rotation);
                room.add_child(&door);
            }
        }
    }
}


impl MapLayer {
    fn coords_is_ok(&self, coords: Vector2i) -> bool {
        coords.x >= 0 && coords.y >= 0
        && coords.x < self.width  as i32
        && coords.y < self.height as i32
    }

    fn get_cell_type(&self, coords: Vector2i) -> Option<CellType> {
        if self.coords_is_ok(coords) {
            Some(self.grid[coords.y as usize][coords.x as usize].c_type.clone())
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

    fn get_mut_cell_data(&mut self, coords: Vector2i) -> Option<&mut CellData> {
        if self.coords_is_ok(coords) {
            self.grid[coords.y as usize][coords.x as usize].data.as_mut()
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
                    poss_coords.push(PossCoords { bridge, room, direction: dir });
                }
            }
        }
        poss_coords
    }

    // REMAKE
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
                self.set_cell(next.bridge, CellType::Bridge { direction: next.direction });
                self.set_cell(next.room, CellType::Room);
                curr_coords = next.room;
                created_cells.push(curr_coords);
            }
        }
    }
}