pub mod utils;

pub mod basic;
pub mod bsp;
pub mod cellular_automata;
pub mod drunkard_walk;
pub mod maze;
pub mod dla;
pub mod voronoi_cell;

use basic::BasicMap;
use bsp::bsp::BspMap;
use bsp::bsp_interior::BspInteriorMap;
use cellular_automata::CellularAutomataMap;
use drunkard_walk::DrunkardWalkMap;
use maze::MazeMap;
use dla::DlaMap;
use voronoi_cell::VoronoiCellMap;

use utils::{RandomNumberGenerator, Map};

pub trait Architect {
    fn build(&mut self);
    fn get_map(&self) -> &Map;
}

pub struct MapSelector;

impl MapSelector {
    pub fn basic_map(width: i32, height: i32) -> BasicMap {
        BasicMap::new(width, height)
    }

    pub fn bsp_map(width: i32, height: i32) -> BspMap {
        BspMap::new(width, height)
    }

    pub fn bsp_interior_map(width: i32, height: i32) -> BspInteriorMap {
        BspInteriorMap::new(width, height)
    }

    pub fn cellular_automata_map(width: i32, height: i32) -> CellularAutomataMap {
        CellularAutomataMap::new(width, height)
    }

    pub fn drunkard_walk_map_open_area(width: i32, height: i32) -> DrunkardWalkMap {
        DrunkardWalkMap::open_area(width, height)
    }

    pub fn drunkard_walk_map_open_halls(width: i32, height: i32) -> DrunkardWalkMap {
        DrunkardWalkMap::open_halls(width, height)
    }

    pub fn drunkard_walk_map_winding_passages(width: i32, height: i32) -> DrunkardWalkMap {
        DrunkardWalkMap::winding_passages(width, height)
    }

    pub fn drunkard_walk_fat_passages(width: i32, height: i32) -> DrunkardWalkMap {
        DrunkardWalkMap::fat_passages(width, height)
    }

    pub fn drunkard_walk_fearful_symmetry(width: i32, height: i32) -> DrunkardWalkMap {
        DrunkardWalkMap::fearful_symmetry(width, height)
    }

    pub fn maze_map(width: i32, height: i32) -> MazeMap {
        MazeMap::new(width, height)
    }

    pub fn dla_map_walk_inwards(width: i32, height: i32) -> DlaMap {
        DlaMap::walk_inwards(width, height)
    }

    pub fn dla_map_walk_outwards(width: i32, height: i32) -> DlaMap {
        DlaMap::walk_outwards(width, height)
    }

    pub fn dla_map_central_attractor(width: i32, height: i32) -> DlaMap {
        DlaMap::central_attractor(width, height)
    }

    pub fn dla_map_insectoid(width: i32, height: i32) -> DlaMap {
        DlaMap::insectoid(width, height)
    }

    pub fn voronoi_cell_map_pythagoras(width: i32, height: i32) -> VoronoiCellMap {
        VoronoiCellMap::pythagoras(width, height)
    }

    pub fn voronoi_cell_map_manhattan(width: i32, height: i32) -> VoronoiCellMap {
        VoronoiCellMap::manhattan(width, height)
    }

    pub fn random_map(width: i32, height: i32) -> Box<dyn Architect> {
        let mut rng = RandomNumberGenerator::new();

        let choice = rng.roll_dice(1, 16);

        match choice {
            1 => Box::new(BspMap::new(width, height)),
            2 => Box::new(BspInteriorMap::new(width, height)),
            3 => Box::new(CellularAutomataMap::new(width, height)),
            4 => Box::new(DrunkardWalkMap::open_area(width, height)),
            5 => Box::new(DrunkardWalkMap::open_halls(width, height)),
            6 => Box::new(DrunkardWalkMap::winding_passages(width, height)),
            7 => Box::new(DrunkardWalkMap::fat_passages(width, height)),
            8 => Box::new(DrunkardWalkMap::fearful_symmetry(width, height)),
            9 => Box::new(MazeMap::new(width, height)),
            10 => Box::new(DlaMap::walk_inwards(width, height)),
            11 => Box::new(DlaMap::walk_outwards(width, height)),
            12 => Box::new(DlaMap::central_attractor(width, height)),
            13 => Box::new(DlaMap::insectoid(width, height)),
            14 => Box::new(VoronoiCellMap::pythagoras(width, height)),
            15 => Box::new(VoronoiCellMap::manhattan(width, height)),
            _ => Box::new(BasicMap::new(width, height)),
        }
    }
}