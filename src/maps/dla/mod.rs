/// Diffusion-Limited Aggregation Maps

use super::{utils::*, Architect};
use bracket_pathfinding::prelude::*;

#[derive(PartialEq, Copy, Clone)]
pub(crate) enum DlaAlgorithm {
    WalkInwards,
    WalkOutwards,
    CentralAttractor
}

pub struct DlaMap {
    pub map: Map,
    pub width: i32,
    pub height: i32,
    algorithm: DlaAlgorithm,
    symmetry: Symmetry,
    brush_size: i32,
    floor_percent: f32
}

impl DlaMap {
    pub fn walk_inwards(width: i32, height: i32) -> Self {
        Self {
            map: Map::new(width, height),
            width,
            height,
            algorithm: DlaAlgorithm::WalkInwards,
            brush_size: 1,
            symmetry: Symmetry::None,
            floor_percent: 0.45
        }
    }

    pub fn walk_outwards(width: i32, height: i32) -> Self {
        Self {
            map: Map::new(width, height),
            width,
            height,
            algorithm: DlaAlgorithm::WalkOutwards,
            brush_size: 2,
            symmetry: Symmetry::None,
            floor_percent: 0.45
        }
    }

    pub fn central_attractor(width: i32, height: i32) -> Self {
        Self {
            map: Map::new(width, height),
            width,
            height,
            algorithm: DlaAlgorithm::CentralAttractor,
            brush_size: 2,
            symmetry: Symmetry::None,
            floor_percent: 0.45
        }
    }

    pub fn insectoid(width: i32, height: i32) -> Self {
        Self {
            map: Map::new(width, height),
            width,
            height,
            algorithm: DlaAlgorithm::CentralAttractor,
            brush_size: 2,
            symmetry: Symmetry::Horizontal,
            floor_percent: 0.25
        }
    }
}

impl Architect for DlaMap {
    fn build(&mut self) {
        let mut rng = RandomNumberGenerator::new();
    
        // Carve a starting seed
        let start_position = Position::new(self.width / 2, self.height / 2);
        let start_idx = self.map.xy_idx(start_position.x, start_position.y);
        self.map.start_position = start_position;

        self.map.set_tile_at_idx(start_idx, TileType::Floor);
        self.map.set_tile_at_idx(start_idx - 1, TileType::Floor);
        self.map.set_tile_at_idx(start_idx + 1, TileType::Floor);
        self.map.set_tile_at_idx(start_idx - self.width as usize, TileType::Floor);
        self.map.set_tile_at_idx(start_idx + self.width as usize, TileType::Floor);
    
        // Random walker
        let total_tiles = self.width * self.height;
        let desired_floor_tiles = (self.floor_percent * total_tiles as f32) as usize;
        let mut floor_tile_count = self.map.count_tile_type(TileType::Floor);

        while floor_tile_count < desired_floor_tiles {    
            match self.algorithm {
                DlaAlgorithm::WalkInwards => {
                    let mut digger_x = rng.roll_dice(1, self.width - 3) + 1;
                    let mut digger_y = rng.roll_dice(1, self.height - 3) + 1;
                    let mut prev_x = digger_x;
                    let mut prev_y = digger_y;

                    while self.map.get_tile(digger_x, digger_y) == TileType::Wall {
                        prev_x = digger_x;
                        prev_y = digger_y;
                        let stagger_direction = rng.roll_dice(1, 4);

                        match stagger_direction {
                            1 => { if digger_x > 2 { digger_x -= 1; } }
                            2 => { if digger_x < self.width - 2 { digger_x += 1; } }
                            3 => { if digger_y > 2 { digger_y -= 1; } }
                            _ => { if digger_y < self.height - 2 { digger_y += 1; } }
                        }
                    }

                    paint(&mut self.map, self.symmetry, self.brush_size, prev_x, prev_y);
                },
                DlaAlgorithm::WalkOutwards => {
                    let mut digger_x = start_position.x;
                    let mut digger_y = start_position.y;

                    while self.map.get_tile(digger_x, digger_y) == TileType::Floor {
                        let stagger_direction = rng.roll_dice(1, 4);
                        match stagger_direction {
                            1 => { if digger_x > 2 { digger_x -= 1; } }
                            2 => { if digger_x < self.width - 2 { digger_x += 1; } }
                            3 => { if digger_y > 2 { digger_y -= 1; } }
                            _ => { if digger_y < self.height - 2 { digger_y += 1; } }
                        }
                    }
                    paint(&mut self.map, self.symmetry, self.brush_size, digger_x, digger_y);
                },
                DlaAlgorithm::CentralAttractor => {
                    let mut digger_x = rng.roll_dice(1, self.width - 3) + 1;
                    let mut digger_y = rng.roll_dice(1, self.height - 3) + 1;
                    let mut prev_x = digger_x;
                    let mut prev_y = digger_y;
                
                    let mut path = line2d(
                        LineAlg::Bresenham, 
                        Point::new( digger_x, digger_y ), 
                        Point::new( start_position.x, start_position.y )
                    );
                
                    while self.map.get_tile(digger_x, digger_y) == TileType::Wall && !path.is_empty() {
                        prev_x = digger_x;
                        prev_y = digger_y;
                        digger_x = path[0].x;
                        digger_y = path[0].y;
                        path.remove(0);
                    }

                    paint(&mut self.map, self.symmetry, self.brush_size, prev_x, prev_y);
                }
            }

            floor_tile_count = self.map.count_tile_type(TileType::Floor);
        }

        // Find all tiles we can reach from the starting point
        let exit_tile = remove_unreachable_areas_returning_most_distant(&mut self.map, start_idx);

        // Place the stairs
        self.map.set_tile_at_idx(exit_tile, TileType::Exit);
    }

    fn get_map(&self) -> &Map {
        &self.map
    }

    fn start_position(&self) -> (i32, i32) {
        (self.map.start_position.x, self.map.start_position.y)
    }
}