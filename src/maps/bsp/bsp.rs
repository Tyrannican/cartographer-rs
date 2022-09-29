use crate::maps::{utils::*, Architect};

pub struct BspMap {
    pub map: Map,
    pub width: i32,
    pub height: i32,
    pub rooms: Vec<Room>,
    pub rects: Vec<Room>,
}

impl BspMap {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            map: Map::new(width, height),
            width,
            height,
            rooms: Vec::new(),
            rects: Vec::new()
        }
    }

    fn add_subrects(&mut self, rect: Room) {
        let width = i32::abs((rect.x1 - rect.x2) as i32);
        let height = i32::abs((rect.y1 - rect.y2) as i32);
        let half_width = i32::max(width / 2, 1) as i32;
        let half_height = i32::max(height / 2, 1) as i32;

        self.rects.push(Room::new( rect.x1, rect.y1, half_width, half_height ));
        self.rects.push(Room::new( rect.x1, rect.y1 + half_height, half_width, half_height ));
        self.rects.push(Room::new( rect.x1 + half_width, rect.y1, half_width, half_height ));
        self.rects.push(Room::new( rect.x1 + half_width, rect.y1 + half_height, half_width, half_height ));
    }

    fn get_random_rect(&mut self, rng : &mut RandomNumberGenerator) -> Room {
        if self.rects.len() == 1 { return self.rects[0]; }
        let idx = (rng.roll_dice(1, self.rects.len() as i32)-1) as usize;
        self.rects[idx]
    }

    fn get_random_sub_rect(&self, rect : Room, rng : &mut RandomNumberGenerator) -> Room {
        let mut result = rect;
        let rect_width = i32::abs((rect.x1 - rect.x2) as i32);
        let rect_height = i32::abs((rect.y1 - rect.y2) as i32);
    
        let w = i32::max(3, rng.roll_dice(1, i32::min(rect_width, 10))-1) + 1;
        let h = i32::max(3, rng.roll_dice(1, i32::min(rect_height, 10))-1) + 1;
    
        result.x1 += rng.roll_dice(1, 6)-1;
        result.y1 += rng.roll_dice(1, 6)-1;
        result.x2 = result.x1 + w;
        result.y2 = result.y1 + h;
    
        result
    }

    fn is_possible(&self, rect : Room) -> bool {
        let mut expanded = rect;
        expanded.x1 -= 2;
        expanded.x2 += 2;
        expanded.y1 -= 2;
        expanded.y2 += 2;
    
        let mut can_build = true;
    
        for y in expanded.y1 ..= expanded.y2 {
            for x in expanded.x1 ..= expanded.x2 {
                if x > self.width-2 { can_build = false; }
                if y > self.height-2 { can_build = false; }
                if x < 1 { can_build = false; }
                if y < 1 { can_build = false; }
                if can_build {
                    if self.map.get_tile(x, y) != TileType::Wall {
                        can_build = false;
                    }
                }
            }
        }
    
        can_build
    }

    fn apply_room_to_map(&mut self, room : &Room) {
        for y in room.y1 + 1 ..= room.y2 {
            for x in room.x1 + 1 ..= room.x2 {
                self.map.set_tile(x, y, TileType::Floor);
            }
        }
    }

    fn draw_corridor(&mut self, x1:i32, y1:i32, x2:i32, y2:i32) {
        let mut x = x1;
        let mut y = y1;
    
        while x != x2 || y != y2 {
            if x < x2 {
                x += 1;
            } else if x > x2 {
                x -= 1;
            } else if y < y2 {
                y += 1;
            } else if y > y2 {
                y -= 1;
            }
    
            self.map.set_tile(x, y, TileType::Floor);
        }
    }
}

impl Architect for BspMap {
    fn build(&mut self) {
        let mut rng = RandomNumberGenerator::new();

        self.rects.clear();
        self.rects.push( Room::new(2, 2, self.width-5, self.height-5) ); // Start with a single map-sized rectangle
        let first_room = self.rects[0];
        self.add_subrects(first_room); // Divide the first room

        // Up to 240 times, we get a random rectangle and divide it. If its possible to squeeze a
        // room in there, we place it and add it to the rooms list.
        let mut n_rooms = 0;
        while n_rooms < 240 {
            let rect = self.get_random_rect(&mut rng);
            let candidate = self.get_random_sub_rect(rect, &mut rng);

            if self.is_possible(candidate) {
                self.apply_room_to_map(&candidate);
                self.rooms.push(candidate);
                self.add_subrects(rect);
            }

            n_rooms += 1;
        }

        // Now we want corridors
        for i in 0..self.rooms.len()-1 {
            let room = self.rooms[i];
            let next_room = self.rooms[i+1];
            let start_x = room.x1 + (rng.roll_dice(1, i32::abs(room.x1 - room.x2))-1);
            let start_y = room.y1 + (rng.roll_dice(1, i32::abs(room.y1 - room.y2))-1);
            let end_x = next_room.x1 + (rng.roll_dice(1, i32::abs(next_room.x1 - next_room.x2))-1);
            let end_y = next_room.y1 + (rng.roll_dice(1, i32::abs(next_room.y1 - next_room.y2))-1);
            self.draw_corridor(start_x, start_y, end_x, end_y);
        }

        let (start_x, start_y) = self.rooms[0].center();
        self.map.start_position = Position::new(start_x, start_y);

        let (exit_x, exit_y) = self.rooms[self.rooms.len() - 1].center();
        self.map.set_tile(exit_x, exit_y, TileType::Exit);
    }
}