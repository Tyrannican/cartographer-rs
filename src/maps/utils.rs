use rand::Rng;
use std::fs;
use std::io::prelude::*;

use bracket_pathfinding::prelude::*;

#[derive(Copy, Clone)]
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
    pub width: i32,
    pub height: i32,
    pub start_position: Position,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            tiles: vec![TileType::Wall; (width * height) as usize],
            width,
            height,
            start_position: Position::new(0, 0)
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

    pub fn count_tile_type(&self, tile: TileType) -> usize {
        self.tiles.iter().filter(|a| **a == tile).count()
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width-1 || y < 1 || y > self.height-1 { return false; }
        self.get_tile(x, y) != TileType::Wall
    }
}

/// Required for bracket-lib pathfinding
impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

/// Required for bracket-lib pathfinding
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

pub fn output_map(map: &Map, name: &str) {
    let mut outer = Vec::new();
    
    for y in 0..map.height {
        let mut inner = Vec::new();
        for x in 0..map.width {
            let tile = map.get_tile(x, y);
            match tile {
                TileType::Floor => inner.push(' '),
                TileType::Wall => inner.push('#'),
                TileType::Exit => inner.push('E'),
                _ => {}
            }
        }

        outer.push(inner);
    }
    
    let filename = format!("test_maps_output/{}", name);
    let mut output = fs::File::create(filename).unwrap();
    
    for inner in outer.iter() {
        for c in inner.iter() {
            write!(output, "{}", c).unwrap();
        }
        write!(output, "\n").unwrap();
    }
}

/// Searches a map, removes unreachable areas and returns the most distant tile.
pub fn remove_unreachable_areas_returning_most_distant(map : &mut Map, start_idx : usize) -> usize {
    let map_starts : Vec<usize> = vec![start_idx];
    let dijkstra_map = DijkstraMap::new(map.width as usize, map.height as usize, &map_starts , map, 200.0);
    let mut exit_tile = (0, 0.0f32);
    for (i, tile) in map.tiles.iter_mut().enumerate() {
        if *tile == TileType::Floor {
            let distance_to_start = dijkstra_map.map[i];
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

    exit_tile.0
}

#[derive(PartialEq, Copy, Clone)]
pub enum Symmetry { None, Horizontal, Vertical, Both }

pub fn paint(map: &mut Map, mode: Symmetry, brush_size: i32, x: i32, y:i32) {
    match mode {
        Symmetry::None => apply_paint(map, brush_size, x, y),
        Symmetry::Horizontal => {
            let center_x = map.width / 2;
            if x == center_x {
                apply_paint(map, brush_size, x, y);                    
            } else {
                let dist_x = i32::abs(center_x - x);
                apply_paint(map, brush_size, center_x + dist_x, y);
                apply_paint(map, brush_size, center_x - dist_x, y);
            }
        }
        Symmetry::Vertical => {
            let center_y = map.height / 2;
            if y == center_y {
                apply_paint(map, brush_size, x, y);
            } else {
                let dist_y = i32::abs(center_y - y);
                apply_paint(map, brush_size, x, center_y + dist_y);
                apply_paint(map, brush_size, x, center_y - dist_y);
            }
        }
        Symmetry::Both => {
            let center_x = map.width / 2;
            let center_y = map.height / 2;
            if x == center_x && y == center_y {
                apply_paint(map, brush_size, x, y);
            } else {
                let dist_x = i32::abs(center_x - x);
                apply_paint(map, brush_size, center_x + dist_x, y);
                apply_paint(map, brush_size, center_x - dist_x, y);
                let dist_y = i32::abs(center_y - y);
                apply_paint(map, brush_size, x, center_y + dist_y);
                apply_paint(map, brush_size, x, center_y - dist_y);
            }
        }
    }
}

fn apply_paint(map: &mut Map, brush_size: i32, x: i32, y: i32) {
    match brush_size {
        1 => map.set_tile(x, y, TileType::Floor),
        _ => {
            let half_brush_size = brush_size / 2;
            for brush_y in y - half_brush_size .. y + half_brush_size {
                for brush_x in x-half_brush_size .. x+half_brush_size {
                    if brush_x > 1 && brush_x < map.width-1 && brush_y > 1 && brush_y < map.height-1 {
                        map.set_tile(brush_x, brush_y, TileType::Floor);
                    }
                }
            }
        }
    }
}
