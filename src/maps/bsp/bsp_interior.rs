use crate::maps::{utils::*, Architect};

pub struct BspInteriorMap {
    pub map: Map,
    pub width: i32,
    pub height: i32,
    pub rooms: Vec<Room>,
    pub(crate) rects: Vec<Room>
}

impl BspInteriorMap {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            map: Map::new(width, height),
            width,
            height,
            rooms: Vec::new(),
            rects: Vec::new()
        }
    }

    fn add_subrects(&mut self, rect : Room, rng : &mut RandomNumberGenerator) {
        const MIN_ROOM_SIZE: i32 = 6;
        
        // Remove the last rect from the list
        if !self.rects.is_empty() {
            self.rects.remove(self.rects.len() - 1);
        }
    
        // Calculate boundaries
        let width  = rect.x2 - rect.x1;
        let height = rect.y2 - rect.y1;
        let half_width = width / 2;
        let half_height = height / 2;
    
        let split = rng.roll_dice(1, 4);
    
        if split <= 2 {
            // Horizontal split
            let h1 = Room::new( rect.x1, rect.y1, half_width - 1, height );
            self.rects.push( h1 );
            if half_width > MIN_ROOM_SIZE { self.add_subrects(h1, rng); }
            let h2 = Room::new( rect.x1 + half_width, rect.y1, half_width, height );
            self.rects.push( h2 );
            if half_width > MIN_ROOM_SIZE { self.add_subrects(h2, rng); }
        } else {
            // Vertical split
            let v1 = Room::new( rect.x1, rect.y1, width, half_height - 1 );
            self.rects.push(v1);
            if half_height > MIN_ROOM_SIZE { self.add_subrects(v1, rng); }
            let v2 = Room::new( rect.x1, rect.y1 + half_height, width, half_height );
            self.rects.push(v2);
            if half_height > MIN_ROOM_SIZE { self.add_subrects(v2, rng); }
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

impl Architect for BspInteriorMap {
    fn build(&mut self) {
        let mut rng = RandomNumberGenerator::new();

        self.rects.clear();
        self.rects.push( Room::new(1, 1, self.width - 2, self.height -  2) ); // Start with a single map-sized rectangle
        let first_room = self.rects[0];
        self.add_subrects(first_room, &mut rng); // Divide the first room

        let rooms = self.rects.clone();
        for r in rooms.iter() {
            let room = *r;
            self.rooms.push(room);
            for y in room.y1 .. room.y2 {
                for x in room.x1 .. room.x2 {
                    let idx = self.map.xy_idx(x, y);
                    if idx > 0 && idx < ((self.width * self.height)-1) as usize {
                        self.map.set_tile(x, y, TileType::Floor);
                    }
                }
            }
        }

        // Corridors
        for i in 0..self.rooms.len()-1 {
            let room = self.rooms[i];
            let next_room = self.rooms[i + 1];
            let start_x = room.x1 + (rng.roll_dice(1, i32::abs(room.x1 - room.x2)) - 1);
            let start_y = room.y1 + (rng.roll_dice(1, i32::abs(room.y1 - room.y2)) - 1);
            let end_x = next_room.x1 + (rng.roll_dice(1, i32::abs(next_room.x1 - next_room.x2)) - 1);
            let end_y = next_room.y1 + (rng.roll_dice(1, i32::abs(next_room.y1 - next_room.y2)) - 1);
            self.draw_corridor(start_x, start_y, end_x, end_y);
        }

        let (start_x, start_y) = self.rooms[0].center();
        self.map.start_position = Position::new(start_x, start_y);

        let (exit_x, exit_y) = self.rooms[self.rooms.len() - 1].center();
        self.map.set_tile(exit_x, exit_y, TileType::Exit);
    }

    fn get_map(&self) -> &Map {
        &self.map
    }

    fn start_position(&self) -> (i32, i32) {
        (self.map.start_position.x, self.map.start_position.y)
    }
}