use godot::prelude::*;
use godot::classes::{ GridMap, MeshLibrary, Area3D };

use rand::RngExt;


#[derive(Clone, Debug)]
enum CellType { 
	Wall, 
	Room, 
	Bridge { direction: Direction }, 
	Start 
}

enum CellState { Unused, Used, OutOfGrid }


#[derive(Clone)]
struct CellData {
	enter: Direction,
	doors: [Direction; 4],
}


#[derive(Clone, Debug)]
struct Coords {
	y: usize,
	x: usize
}


#[derive(Clone, Copy, Debug, PartialEq)]
struct Direction {
	y: isize,
	x: isize,
}
impl Direction {
	const UP: Direction  = Direction { y: -1, x: 0 };
	const RIGHT: Direction = Direction { y: 0, x: 1 };
	const DOWN: Direction  = Direction { y: 1, x: 0 };
	const LEFT: Direction  = Direction { y: 0, x: -1 };
	const NULL: Direction = Direction { y: 0, x: 0};

	fn all() -> [Self; 4] {
		[Self::UP, Self::RIGHT, Self::DOWN, Self::LEFT]
	}
}

#[derive(Clone)]
struct Cell {
	c_type: CellType,
	data: Option<CellData>
}

#[derive(Clone)]
struct PossCoords {
	bridge: Coords,
	room: Coords,
	direction: Direction
}


#[derive(GodotClass)]
#[class(base=Node, no_init)]
pub struct MapLayer {
	base: Base<Node>,
	width: usize,
	height: usize,
	s_coords: Coords,
	grid: Vec<Vec<Cell>>,
}


#[godot_api]
impl MapLayer {
	pub fn new(width: usize, height: usize) -> Gd<Self> {
		let mut map = Gd::from_init_fn(|base| {
			Self {
				base,
				width,
				height,
				s_coords: Coords{
					y: rand::rng().random_range(0..height),
					x: rand::rng().random_range(0..width) 
				},
				grid: vec![vec![Cell{c_type: CellType::Wall, data: None}; width]; height],
			}
		});
		map.bind_mut().generate();
		map
	}


	#[func]
    fn print_grid(&self) {
        for row in &self.grid {
            for cell in row {
                match cell.c_type {
                    CellType::Wall => print!("#"),
                    CellType::Room => print!("@"),
                    CellType::Bridge { direction: _ } => print!("-"),
                    CellType::Start => print!("S"),
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
		grid_map.set_cell_size(Vector3::new(1.0, 0.4, 1.0));
		// grid_map.set_position(Vector3::new(0.0, 0.0, 0.0));
		let mesh_lib: Gd<MeshLibrary> = load("res://assets/labyrinth.tres");
		grid_map.set_mesh_library(&mesh_lib);
		for y in 0..self.height {
			for x in 0..self.width {
				match self.get_cell_type(&Coords{y, x}) {
					CellType::Room => grid_map.set_cell_item(Vector3i::new(x as i32, 0, y as i32), 0),
					CellType::Bridge { direction } => {
						let orientation = match direction {
						    // (0, _) => 16,  // Up or Down
						    // (_, 0) => 10,  // Left or Right
						    // _ => 0
						    Direction::UP | Direction::DOWN => 16,
						    Direction::RIGHT | Direction::LEFT => 10,
						    _ => 0
						};
						// Setting default parameters
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
	pub fn is_walkable(&self, coords: Vector2i) -> bool {
		if coords.x < 0 || coords.y < 0 {
			return false;
		}
		let coords = Coords{ y: coords.y as usize, x: coords.x as usize };
		matches!(self.get_cell_state(&coords), CellState::Used)
	}

	#[func]
	pub fn get_start_position(&self) -> Vector3 {
		Vector3::new( self.s_coords.x as f32 + 0.5, 1.0, self.s_coords.y as f32 + 0.5 )
	}
}


// Map Generating logic
impl MapLayer {

	fn get_cell_type(&self, coords: &Coords) -> CellType {
		self.grid[coords.y][coords.x].c_type.clone()
	}

	fn get_mut_cell(&mut self, coords: &Coords) -> &mut Cell {
		&mut self.grid[coords.y][coords.x]
	}

	fn get_mut_cell_data(&mut self, coords: &Coords) -> &mut Option<CellData> {
		&mut self.grid[coords.y][coords.x].data
	}

	fn set_cell(&mut self, coords: &Coords, c_type: CellType) {
		self.grid[coords.y][coords.x].c_type = c_type;
	}


	fn get_cell_state(&self, coords: &Coords) -> CellState {
		if coords.y >= self.height || coords.x >= self.width {
			return CellState::OutOfGrid;
		}
		match self.get_cell_type(&coords) {
			CellType::Wall => return CellState::Unused,
			_ => return CellState::Used
		}
	}

	fn check_direction(&self, coords: &Coords, dir: &Direction) -> Option<Coords> {
		let new_coords = Direction{y: coords.y as isize + dir.y,
								   x: coords.x as isize + dir.x};
		if new_coords.y < 0 || new_coords.x < 0 {
			return None;
		}

		let new_coords = Coords{y: new_coords.y as usize, 
								x: new_coords.x as usize};
		match self.get_cell_state(&new_coords) {
			CellState::Unused => return Some(new_coords),
			_ => return None
		}
	}

	fn get_poss_coords(&self, coords: &Coords) -> Vec<PossCoords> {
		let mut poss_coords: Vec<PossCoords> = vec![];
		for dir in Direction::all() {
			if let Some(new_coords_1) = self.check_direction(&coords, &dir) {
				if let Some(new_coords_2) = self.check_direction(&new_coords_1, &dir) {
					poss_coords.push(PossCoords{
						bridge: new_coords_1,
						room: new_coords_2,
						direction: dir
					});
				}
			}
		}
		poss_coords
	}

	fn get_near_bridges(&self, coords: &Coords) -> [Direction; 4] {
		let mut bridges = [Direction::NULL; 4];
		let mut count = 0;
		for dir in Direction::all() {
			if let Some(new_coords) = self.check_direction(coords, &dir) {
				bridges[count] = dir;
				count += 1;
			}
		}
		bridges
	}

	fn generate_labyrinth(&mut self) {
		let mut curr_coords = self.s_coords.clone();
		self.set_cell(&curr_coords, CellType::Start);
		let mut created_cells: Vec<Coords> = vec![];

		for _i in 0..30 {
			let way_lenght: usize = rand::rng().random_range(1..5);
			for _i in 0..way_lenght {
				let poss_coords = self.get_poss_coords(&curr_coords);
				if poss_coords.is_empty() {
					let indx = rand::rng().random_range(0..created_cells.len());
					curr_coords = created_cells[indx].clone();
					continue;
				}
				let indx = rand::rng().random_range(0..poss_coords.len());
				let new_coords = poss_coords[indx].clone();
				self.set_cell(&new_coords.bridge, CellType::Bridge{ direction: new_coords.direction });
				self.set_cell(&new_coords.room, CellType::Room);
				curr_coords = new_coords.room.clone();
				created_cells.push(curr_coords.clone());
			}
		}
	}
}