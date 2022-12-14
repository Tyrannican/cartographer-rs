use bracket_pathfinding::prelude::*;

use crate::maps::{utils::*, Architect};

#[derive(PartialEq, Copy, Clone)]
pub enum DistanceAlgorithm {
    Pythagoras,
    Manhattan,
    Chebyshev
}

pub struct VoronoiCellMap {
    pub map: Map,
    pub width: i32,
    pub height: i32,
    n_seeds: usize,
    distance_algorithm: DistanceAlgorithm
}

impl VoronoiCellMap {
    pub fn pythagoras(width: i32, height: i32) -> Self {
        Self {
            map: Map::new(width, height),
            width,
            height,
            n_seeds: 64,
            distance_algorithm: DistanceAlgorithm::Pythagoras
        }
    }

    pub fn manhattan(width: i32, height: i32) -> Self {
        Self {
            map: Map::new(width, height),
            width,
            height,
            n_seeds: 64,
            distance_algorithm: DistanceAlgorithm::Manhattan
        }
    }
}

impl Architect for VoronoiCellMap {
    fn build(&mut self) {
        let mut rng = RandomNumberGenerator::new();

        // Make a Voronoi diagram. We'll do this the hard way to learn about the technique!
        let mut voronoi_seeds : Vec<(usize, Point)> = Vec::new();

        while voronoi_seeds.len() < self.n_seeds {
            let vx = rng.roll_dice(1, self.width-1);
            let vy = rng.roll_dice(1, self.height-1);
            let vidx = self.map.xy_idx(vx, vy);
            let candidate = (vidx, Point::new(vx, vy));
            if !voronoi_seeds.contains(&candidate) {
                voronoi_seeds.push(candidate);
            }
        }

        let mut voronoi_distance = vec![(0, 0.0f32) ; self.n_seeds];
        let mut voronoi_membership : Vec<i32> = vec![0 ; self.width as usize * self.height as usize];
        for (i, vid) in voronoi_membership.iter_mut().enumerate() {
            let x = i as i32 % self.width;
            let y = i as i32 / self.width;

            for (seed, pos) in voronoi_seeds.iter().enumerate() {
                let distance;
                match self.distance_algorithm {
                    DistanceAlgorithm::Pythagoras => {
                        distance = DistanceAlg::PythagorasSquared.distance2d(
                            Point::new(x, y),
                            pos.1
                        );
                    }
                    DistanceAlgorithm::Manhattan => {
                        distance = DistanceAlg::Manhattan.distance2d(
                            Point::new(x, y),
                            pos.1
                        );
                    }
                    DistanceAlgorithm::Chebyshev => {
                        distance = DistanceAlg::Chebyshev.distance2d(
                            Point::new(x, y),
                            pos.1
                        );
                    }
                }
                voronoi_distance[seed] = (seed, distance);
            }

            voronoi_distance.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());

            *vid = voronoi_distance[0].0 as i32;
        }


        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                let mut neighbors = 0;
                let my_idx = self.map.xy_idx(x, y);
                let my_seed = voronoi_membership[my_idx];
                if voronoi_membership[self.map.xy_idx(x-1, y)] != my_seed { neighbors += 1; }
                if voronoi_membership[self.map.xy_idx(x+1, y)] != my_seed { neighbors += 1; }
                if voronoi_membership[self.map.xy_idx(x, y-1)] != my_seed { neighbors += 1; }
                if voronoi_membership[self.map.xy_idx(x, y+1)] != my_seed { neighbors += 1; }

                if neighbors < 2 {
                    self.map.set_tile_at_idx(my_idx, TileType::Floor);
                }
            }
        }

        // Find a starting point; start at the middle and walk left until we find an open tile
        let mut start_position = Position::new(self.width / 2, self.height / 2);
        let mut start_idx = self.map.xy_idx(start_position.x, start_position.y);
        while self.map.tiles[start_idx] != TileType::Floor {
            start_position.x -= 1;
            start_idx = self.map.xy_idx(start_position.x, start_position.y);
        }

        self.map.start_position = start_position;

        let exit_tile = remove_unreachable_areas_returning_most_distant(&mut self.map, start_idx);
        self.map.set_tile_at_idx(exit_tile, TileType::Exit);
    }

    fn get_map(&self) -> &Map {
        &self.map
    }

    fn start_position(&self) -> (i32, i32) {
        (self.map.start_position.x, self.map.start_position.y)
    }
}