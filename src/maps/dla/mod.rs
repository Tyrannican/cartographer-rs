/// Diffusion-Limited Aggregation Maps

use super::{utils::*, Architect};
use bracket_pathfinding::prelude::*;

#[derive(PartialEq, Copy, Clone)]
pub enum DlaAlgorithm {
    WalkInwards,
    WalkOutwards,
    CentralAttractor
}

#[derive(PartialEq, Copy, Clone)]
pub enum DlaSymmetry {
    None,
    Horizontal,
    Vertical,
    Both
}

pub struct DlaMap {
    pub map: Map,
    pub width: i32,
    pub height: i32,
    algorithm: DlaAlgorithm,
    symmetry: DlaSymmetry,
    brush_size: i32,
    floor_percent: f32
}

impl DlaMap {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            map: Map::new(width, height),
            width,
            height,
            algorithm: DlaAlgorithm::WalkInwards,
            brush_size: 1,
            symmetry: DlaSymmetry::None,
            floor_percent: 0.45
        }
    }

    pub fn walk_inwards(width: i32, height: i32) -> Self {
        Self {
            map: Map::new(width, height),
            width,
            height,
            algorithm: DlaAlgorithm::WalkInwards,
            brush_size: 1,
            symmetry: DlaSymmetry::None,
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
            symmetry: DlaSymmetry::None,
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
            symmetry: DlaSymmetry::None,
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
            symmetry: DlaSymmetry::Horizontal,
            floor_percent: 0.25
        }
    }

    fn paint(&mut self, x: i32, y: i32) {
        match self.symmetry {
            DlaSymmetry::None => self.apply_paint(x, y),
            DlaSymmetry::Horizontal => {
                let center_x = self.map.width / 2;
                if x == center_x {
                    self.apply_paint(x, y);                    
                } else {
                    let dist_x = i32::abs(center_x - x);
                    self.apply_paint(center_x + dist_x, y);
                    self.apply_paint(center_x - dist_x, y);
                }
            }
            DlaSymmetry::Vertical => {
                let center_y = self.map.height / 2;
                if y == center_y {
                    self.apply_paint(x, y);
                } else {
                    let dist_y = i32::abs(center_y - y);
                    self.apply_paint(x, center_y + dist_y);
                    self.apply_paint(x, center_y - dist_y);
                }
            }
            DlaSymmetry::Both => {
                let center_x = self.map.width / 2;
                let center_y = self.map.height / 2;
                if x == center_x && y == center_y {
                    self.apply_paint(x, y);
                } else {
                    let dist_x = i32::abs(center_x - x);
                    self.apply_paint(center_x + dist_x, y);
                    self.apply_paint(center_x - dist_x, y);
                    let dist_y = i32::abs(center_y - y);
                    self.apply_paint(x, center_y + dist_y);
                    self.apply_paint(x, center_y - dist_y);
                }
            }
        }
    }

    fn apply_paint(&mut self, x: i32, y: i32) {
        match self.brush_size {
            1 => self.map.set_tile(x, y, TileType::Floor),
            _ => {
                let half_brush_size = self.brush_size / 2;
                for brush_y in y - half_brush_size..y + half_brush_size {
                    for brush_x in x - half_brush_size..x + half_brush_size {
                        if brush_x > 1 && brush_x < self.width-1 && brush_y > 1 && brush_y < self.height-1 {
                            self.map.set_tile(brush_x, brush_y, TileType::Floor);
                        }
                    }
                }
            }
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

                    self.paint(prev_x, prev_y);
                },
                DlaAlgorithm::WalkOutwards => {
                    let mut digger_x = start_position.x;
                    let mut digger_y = start_position.y;

                    while self.map.get_tile(digger_x, digger_y) == TileType::Floor {
                        let stagger_direction = rng.roll_dice(1, 4);
                        match stagger_direction {
                            1 => { if digger_x > 2 { digger_x -= 1; } }
                            2 => { if digger_x < self.map.width - 2 { digger_x += 1; } }
                            3 => { if digger_y > 2 { digger_y -= 1; } }
                            _ => { if digger_y < self.map.height - 2 { digger_y += 1; } }
                        }
                    }
                    self.paint(digger_x, digger_y);
                },
                DlaAlgorithm::CentralAttractor => {
                    let mut digger_x = rng.roll_dice(1, self.map.width - 3) + 1;
                    let mut digger_y = rng.roll_dice(1, self.map.height - 3) + 1;
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
                    self.paint(prev_x, prev_y);
                }
                _ => {}
            }

            floor_tile_count = self.map.count_tile_type(TileType::Floor);
        }

        // Find all tiles we can reach from the starting point
        let exit_tile = remove_unreachable_areas_returning_most_distant(&mut self.map, start_idx);

        // Place the stairs
        self.map.set_tile_at_idx(exit_tile, TileType::Exit);
    }
}