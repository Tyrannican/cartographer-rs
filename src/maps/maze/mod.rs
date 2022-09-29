use crate::maps::{utils::*, Architect};

const TOP : usize = 0;
const RIGHT : usize = 1;
const BOTTOM : usize = 2;
const LEFT : usize = 3;

#[derive(Copy, Clone)]
struct Cell {
    row: i32,
    column: i32,
    walls: [bool; 4],
    visited: bool,
}

impl Cell {
    fn new(row: i32, column: i32) -> Self {
        Self {
            row,
            column,
            walls: [true, true, true, true],
            visited: false
        }
    }

    fn remove_walls(&mut self, next : &mut Cell) {
        let x = self.column - next.column;
        let y = self.row - next.row;

        if x == 1 {
            self.walls[LEFT] = false;
            next.walls[RIGHT] = false;
        }
        else if x == -1 {
            self.walls[RIGHT] = false;
            next.walls[LEFT] = false;
        }
        else if y == 1 {
            self.walls[TOP] = false;
            next.walls[BOTTOM] = false;
        }
        else if y == -1 {
            self.walls[BOTTOM] = false;
            next.walls[TOP] = false;
        }
    }
}

struct Grid<'a> {
    width: i32,
    height: i32,
    cells: Vec<Cell>,
    backtrace: Vec<usize>,
    current: usize,
    rng : &'a mut RandomNumberGenerator
}

impl<'a> Grid<'a> {
    fn new(width: i32, height:i32, rng: &'a mut RandomNumberGenerator) -> Self {
        let mut grid = Self {
            width,
            height,
            cells: Vec::new(),
            backtrace: Vec::new(),
            current: 0,
            rng
        };

        for row in 0..height {
            for column in 0..width {
                grid.cells.push(Cell::new(row, column));
            }
        }

        grid
    }

    fn calculate_index(&self, row: i32, column: i32) -> i32 {
        if row < 0 || column < 0 || column > self.width-1 || row > self.height-1 {
            -1
        } else {
            column + (row * self.width)
        }
    }

    fn get_available_neighbors(&self) -> Vec<usize> {
        let mut neighbors : Vec<usize> = Vec::new();
    
        let current_row = self.cells[self.current].row;
        let current_column = self.cells[self.current].column;
    
        let neighbor_indices : [i32; 4] = [
            self.calculate_index(current_row -1, current_column),
            self.calculate_index(current_row, current_column + 1),
            self.calculate_index(current_row + 1, current_column),
            self.calculate_index(current_row, current_column - 1)
        ];
    
        for i in neighbor_indices.iter() {
            if *i != -1 && !self.cells[*i as usize].visited {
                neighbors.push(*i as usize);
            }
        }
    
        neighbors
    }

    fn find_next_cell(&mut self) -> Option<usize> {
        let neighbors = self.get_available_neighbors();
        if !neighbors.is_empty() {
            if neighbors.len() == 1 {
                return Some(neighbors[0]);
            } else {
                return Some(neighbors[(self.rng.roll_dice(1, neighbors.len() as i32) - 1) as usize]);
            }
        }
        None
    }

    fn generate_maze(&mut self, generator : &mut MazeMap) {
        loop {
            self.cells[self.current].visited = true;
            let next = self.find_next_cell();
    
            match next {
                Some(next) => {
                    self.cells[next].visited = true;
                    self.backtrace.push(self.current);
                    let (lower_part, higher_part) =
                        self.cells.split_at_mut(std::cmp::max(self.current, next));
                    let cell1 = &mut lower_part[std::cmp::min(self.current, next)];
                    let cell2 = &mut higher_part[0];
                    cell1.remove_walls(cell2);
                    self.current = next;
                }
                None => {
                    if !self.backtrace.is_empty() {
                        self.current = self.backtrace[0];
                        self.backtrace.remove(0);
                    } else {
                        break;
                    }
                }
            }
    
            self.copy_to_map(&mut generator.map);   
        }
    }

    fn copy_to_map(&self, map : &mut Map) {
        // Clear the map
        for i in map.tiles.iter_mut() { *i = TileType::Wall; }
    
        for cell in self.cells.iter() {
            let x = cell.column + 1;
            let y = cell.row + 1;
            let idx = map.xy_idx(x * 2, y * 2);
            
            map.set_tile_at_idx(idx, TileType::Floor);
            if !cell.walls[TOP] { map.set_tile_at_idx(idx - map.width as usize, TileType::Floor); }
            if !cell.walls[RIGHT] { map.set_tile_at_idx(idx + 1, TileType::Floor); }
            if !cell.walls[BOTTOM] { map.set_tile_at_idx(idx + map.width as usize, TileType::Floor); }
            if !cell.walls[LEFT] { map.set_tile_at_idx(idx - 1, TileType::Floor); }
        }
    }
}

pub struct MazeMap {
    pub map: Map,
    pub width: i32,
    pub height: i32,
}

impl MazeMap {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            map: Map::new(width, height),
            width,
            height
        }
    }
}

impl Architect for MazeMap {
    fn build(&mut self) {
        let mut rng = RandomNumberGenerator::new();        

        let mut maze = Grid::new((self.width / 2) - 2, (self.height / 2) - 2, &mut rng);
        maze.generate_maze(self);

        // Find a starting point; start at the middle and walk left until we find an open tile
        let start_position = Position::new(2, 2);
        self.map.start_position = start_position;
        let start_idx = self.map.xy_idx(start_position.x, start_position.y);

        // Find all tiles we can reach from the starting point
        let exit_tile = remove_unreachable_areas_returning_most_distant(&mut self.map, start_idx);

        // Place the stairs
        self.map.set_tile_at_idx(exit_tile, TileType::Exit);
    }

    fn get_map(&self) -> &Map {
        &self.map
    }
}