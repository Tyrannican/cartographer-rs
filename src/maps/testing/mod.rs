use std::cmp::{min, max};

use super::utils::*;

use super::Architect;

pub struct TestingMap {
    pub map: Map,
    pub width: i32,
    pub height: i32
}

impl TestingMap {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            map: Map::new(width, height),
            width,
            height
        }
    }

    fn apply_room_to_map(&mut self, room : &Room) {
        for y in room.y1 +1 ..= room.y2 {
            for x in room.x1 + 1 ..= room.x2 {
                self.map.set_tile(x, y, TileType::Floor);
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1:i32, x2:i32, y:i32) {
        for x in min(x1,x2) ..= max(x1,x2) {
            let idx = self.map.xy_idx(x, y);
            if idx > 0 && idx < (self.width * self.height) as usize {
                self.map.set_tile(x, y, TileType::Floor);
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1:i32, y2:i32, x:i32) {
        for y in min(y1,y2) ..= max(y1,y2) {
            let idx = self.map.xy_idx(x, y);
            if idx > 0 && idx < (self.width * self.height) as usize {
                self.map.set_tile(x, y, TileType::Floor);
            }
        }
    }
}

impl Architect for TestingMap {
    fn build(&mut self) {
        let mut rooms : Vec<Room> = Vec::new();
        let max_rooms : i32 = 30;
        let min_size : i32 = 6;
        let max_size : i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..max_rooms {
            let w = rng.range(min_size, max_size);
            let h = rng.range(min_size, max_size);
            let x = rng.roll_dice(1, (self.width + 1) - w - 1) - 1;
            let y = rng.roll_dice(1, (self.height + 1) - h - 1) - 1;
            let new_room = Room::new(x, y, w, h);
            let mut ok = true;
            for other_room in rooms.iter() {
                if new_room.intersect(other_room) { ok = false }
            }
            if ok {
                self.apply_room_to_map(&new_room);

                if !rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = rooms[rooms.len()-1].center();
                    if rng.range(0, 2) == 1 {
                        self.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        self.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        self.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        self.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }

                rooms.push(new_room);
            }
        }

        let (start_x, start_y) = rooms[0].center();
        self.map.start_position = Position::new(start_x, start_y);

        let (exit_x, exit_y) = rooms[rooms.len() - 1].center();
        self.map.set_tile(exit_x, exit_y, TileType::Exit);
    }
}