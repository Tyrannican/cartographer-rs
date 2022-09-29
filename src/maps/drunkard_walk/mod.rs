use crate::maps::{utils::*, Architect};

#[derive(PartialEq, Copy, Clone)]
pub enum DrunkSpawnMode { StartingPoint, Random }

pub struct DrunkardSettings {
    pub spawn_mode : DrunkSpawnMode,
    pub lifetime: i32,
    pub floor_percent: f32
}

pub struct DrunkardWalkMap {
    pub map: Map,
    pub width: i32,
    pub height: i32,
    settings: DrunkardSettings
}

impl DrunkardWalkMap {
    pub fn new(width: i32, height: i32, settings: DrunkardSettings) -> Self {
        Self {
            map: Map::new(width, height),
            width,
            height,
            settings
        }
    }

    pub fn open_area(width: i32, height: i32) -> Self {
        Self {
            map: Map::new(width, height),
            width,
            height,
            settings: DrunkardSettings {
                spawn_mode: DrunkSpawnMode::StartingPoint,
                lifetime: 400,
                floor_percent: 0.5
            }
        }
    }

    pub fn open_halls(width: i32, height: i32) -> Self {
        Self {
            map: Map::new(width, height),
            width,
            height,
            settings: DrunkardSettings {
                spawn_mode: DrunkSpawnMode::Random,
                lifetime: 400,
                floor_percent: 0.5
            }
        }
    }

    pub fn winding_passages(width: i32, height: i32) -> Self {
        Self {
            map: Map::new(width, height),
            width,
            height,
            settings: DrunkardSettings {
                spawn_mode: DrunkSpawnMode::Random,
                lifetime: 100,
                floor_percent: 0.4
            }
        }
    }
}

impl Architect for DrunkardWalkMap {
    fn build(&mut self) {
        let mut rng = RandomNumberGenerator::new();

        // Set a central starting point
        let start_position = Position::new(self.width / 2, self.height / 2);
        let start_idx = self.map.xy_idx(start_position.x, start_position.y);
        self.map.start_position = start_position.clone();
        self.map.set_tile_at_idx(start_idx, TileType::Floor);

        let total_tiles = self.width * self.height;
        let desired_floor_tiles = (self.settings.floor_percent * total_tiles as f32) as usize;
        let mut floor_tile_count = self.map.count_tile_type(TileType::Floor);
        
        let mut digger_count = 0;
        while floor_tile_count < desired_floor_tiles {
            let mut drunk_x;
            let mut drunk_y;

            match self.settings.spawn_mode {
                DrunkSpawnMode::StartingPoint => {
                    drunk_x = start_position.x;
                    drunk_y = start_position.y;
                },
                DrunkSpawnMode::Random => {
                    if digger_count == 0 {
                        drunk_x = start_position.x;
                        drunk_y = start_position.y;
                    } else {
                        drunk_x = rng.roll_dice(1, self.width - 3) + 1;
                        drunk_y = rng.roll_dice(1, self.height - 3) + 1;
                    }
                }
            }

            let mut drunk_life = self.settings.lifetime;

            while drunk_life > 0 {
                self.map.set_tile(drunk_x, drunk_y, TileType::Exit);

                let stagger_direction = rng.roll_dice(1, 4);
                match stagger_direction {
                    1 => { if drunk_x > 2 { drunk_x -= 1; } }
                    2 => { if drunk_x < self.width-2 { drunk_x += 1; } }
                    3 => { if drunk_y > 2 { drunk_y -=1; } }
                    _ => { if drunk_y < self.height-2 { drunk_y += 1; } }
                }

                drunk_life -= 1;
            }

            digger_count += 1;
            for t in self.map.tiles.iter_mut() {
                if *t == TileType::Exit {
                    *t = TileType::Floor;
                }
            }
            floor_tile_count = self.map.count_tile_type(TileType::Floor);
        }

        // Set the exit
        let exit_idx = remove_unreachable_areas_returning_most_distant(&mut self.map, start_idx);
        self.map.set_tile_at_idx(exit_idx, TileType::Exit);

    }
}