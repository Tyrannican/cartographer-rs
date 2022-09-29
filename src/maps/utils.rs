use rand::Rng;
use std::fs;
use std::io::prelude::*;

use bracket_pathfinding::prelude::*;

pub struct Position {
    pub x: i32,
    pub y: i32
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    Exit,
    Void
}

#[derive(Copy, Clone)]
pub struct Room {
    pub x1 : i32,
    pub x2 : i32,
    pub y1 : i32,
    pub y2 : i32
}

impl Room {
    pub fn new(x:i32, y: i32, w:i32, h:i32) -> Self {
        Room{x1:x, y1:y, x2:x+w, y2:y+h}
    }

    // Returns true if this overlaps with other
    pub fn intersect(&self, other: &Room) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2)/2, (self.y1 + self.y2)/2)
    }
}

pub struct Map {
    pub tiles: Vec<TileType>,
    width: i32,
    height: i32,
    pub start_position: Position,
    blocked: Vec<bool>
}

impl Map {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            tiles: vec![TileType::Wall; (width * height) as usize],
            width,
            height,
            start_position: Position::new(0, 0),
            blocked: vec![false; (width * height) as usize] 
        }
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        ((y * self.width) + x) as usize
    }

    pub fn get_tile(&self, x: i32, y: i32) -> TileType {
        let idx = self.xy_idx(x, y);
        self.tiles[idx]
    }

    pub fn get_tile_at_idx(&self, idx: usize) -> TileType {
        self.tiles[idx]
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile: TileType) {
        let idx = self.xy_idx(x, y);
        self.tiles[idx] = tile;
    }

    pub fn set_tile_at_idx(&mut self, idx: usize, tile: TileType) {
        self.tiles[idx] = tile;
    }

    pub fn output_map(&self, name: &str) {
        let mut map = Vec::new();
        
        for x in 0..self.width {
            let mut inner = Vec::new();
            for y in 0..self.height {
                let tile = self.get_tile(x, y);
                match tile {
                    TileType::Floor => inner.push(' '),
                    TileType::Wall => inner.push('#'),
                    TileType::Exit => inner.push('E'),
                    _ => {}
                }
            }
            map.push(inner);
        }
        
        let mut output = fs::File::create(name).unwrap();
        for x in 0..self.width {
            for y in 0..self.height {
                write!(output, "{}", map[x as usize][y as usize]).unwrap()
            }
            write!(output, "\n").unwrap();
        }
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width-1 || y < 1 || y > self.height-1 { return false; }
        self.get_tile(x, y) != TileType::Wall
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.get_tile_at_idx(idx) == TileType::Wall
    }

    fn get_available_exits(&self, idx:usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        let w = self.width as usize;
    
        // Cardinal directions
        if self.is_exit_valid(x - 1, y) { exits.push((idx - 1, 1.0)) };
        if self.is_exit_valid(x + 1, y) { exits.push((idx + 1, 1.0)) };
        if self.is_exit_valid(x, y - 1) { exits.push((idx - w, 1.0)) };
        if self.is_exit_valid(x, y + 1) { exits.push((idx + w, 1.0)) };
    
        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}

pub struct RandomNumberGenerator {
    rng: rand::rngs::ThreadRng
}

impl RandomNumberGenerator {
    pub fn new() -> Self {
        Self { rng: rand::thread_rng() }
    }

    pub fn range(&mut self, min: i32, max: i32) -> i32 {
        self.rng.gen_range(min..max)
    }

    pub fn roll_dice(&mut self, start: i32, end: i32) -> i32 {
        self.rng.gen_range(start..end+1)
    }
}