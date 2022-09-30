use crate::maps::{utils::*, Architect};

pub struct CellularAutomataMap {
    pub map: Map,
    pub width: i32,
    pub height: i32
}

impl CellularAutomataMap {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            map: Map::new(width, height),
            width,
            height
        }
    }
}

impl Architect for CellularAutomataMap {
    fn build(&mut self) {
        let mut rng = RandomNumberGenerator::new();

        // First we completely randomize the map, setting 55% of it to be floor.
        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                let roll = rng.roll_dice(1, 100);
                if roll > 55 { self.map.set_tile(x, y, TileType::Floor) } 
                else { self.map.set_tile(x, y, TileType::Wall) }
            }
        }

        // Now we iteratively apply cellular automata rules
        for _i in 0..15 {
            let mut newtiles = self.map.tiles.clone();

            for y in 1..self.height - 1 {
                for x in 1..self.width - 1 {
                    let idx = self.map.xy_idx(x, y);
                    let mut neighbors = 0;
                    if self.map.get_tile_at_idx(idx - 1) == TileType::Wall { neighbors += 1; }
                    if self.map.get_tile_at_idx(idx + 1) == TileType::Wall { neighbors += 1; }
                    if self.map.get_tile_at_idx(idx - self.width as usize) == TileType::Wall { neighbors += 1; }
                    if self.map.get_tile_at_idx(idx + self.width as usize) == TileType::Wall { neighbors += 1; }
                    if self.map.get_tile_at_idx(idx - (self.width as usize - 1)) == TileType::Wall { neighbors += 1; }
                    if self.map.get_tile_at_idx(idx - (self.width as usize + 1)) == TileType::Wall { neighbors += 1; }
                    if self.map.get_tile_at_idx(idx + (self.width as usize - 1)) == TileType::Wall { neighbors += 1; }
                    if self.map.get_tile_at_idx(idx + (self.width as usize + 1)) == TileType::Wall { neighbors += 1; }

                    if neighbors > 4 || neighbors == 0 {
                        newtiles[idx] = TileType::Wall;
                    }
                    else {
                        newtiles[idx] = TileType::Floor;
                    }
                }
            }

            self.map.tiles = newtiles.clone();
        }

        // Find a starting point
        let mut start = Position::new(self.width / 2, self.height / 2);
        while self.map.get_tile(start.x, start.y) != TileType::Floor {
            start.x -= 1;
        }

        let (start_x, start_y) = (start.x, start.y);
        let start_idx = self.map.xy_idx(start_x, start_y);
        self.map.start_position = start;

        let exit_idx = remove_unreachable_areas_returning_most_distant(&mut self.map, start_idx);
        self.map.set_tile_at_idx(exit_idx, TileType::Exit);
    }

    fn get_map(&self) -> &Map {
        &self.map
    }

    fn start_position(&self) -> (i32, i32) {
        (self.map.start_position.x, self.map.start_position.y)
    }
}