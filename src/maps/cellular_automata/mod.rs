use bracket_pathfinding::prelude::*;
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

        // Find a starting point; start at the middle and walk left until we find an open tile
        let mut start = Position::new(self.width / 2, self.height / 2);
        while self.map.get_tile(start.x, start.y) != TileType::Floor {
            start.x -= 1;
        }

        let (start_x, start_y) = (start.x, start.y);
        self.map.start_position = start;

        // Find all tiles we can reach from the starting point
        let start_idx = self.map.xy_idx(start_x, start_y);
        let map_starts : Vec<usize> = vec![start_idx];
        let dijkstra_map = DijkstraMap::new(self.width, self.height, &map_starts , &self.map, 200.0);
        let mut exit_tile = (0, 0.0f32);
        for (i, tile) in self.map.tiles.iter_mut().enumerate() {
            if *tile == TileType::Floor {
                let distance_to_start = dijkstra_map.map[i];
                println!("{:?}", distance_to_start);
                // We can't get to this tile - so we'll make it a wall
                if distance_to_start == std::f32::MAX {
                    *tile = TileType::Wall;
                } else {
                    // If it is further away than our current exit candidate, move the exit
                    if distance_to_start > exit_tile.1 {
                        exit_tile.0 = i;
                        exit_tile.1 = distance_to_start;
                    }
                }
            }
        }

        self.map.tiles[exit_tile.0] = TileType::Exit;
    }
}