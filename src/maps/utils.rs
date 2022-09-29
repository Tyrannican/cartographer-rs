use rand::Rng;
use std::fs;
use std::io::prelude::*;

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
    tiles: Vec<TileType>,
    width: i32,
    height: i32,
    pub start_position: Position
}

impl Map {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            tiles: vec![TileType::Wall; (width * height) as usize],
            width,
            height,
            start_position: Position::new(0, 0),
        }
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        ((y * self.width) + x) as usize
    }

    pub fn get_tile(&self, x: i32, y: i32) -> TileType {
        let idx = self.xy_idx(x, y);
        self.tiles[idx]
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile: TileType) {
        let idx = self.xy_idx(x, y);
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