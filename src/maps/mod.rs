pub mod utils;

pub mod basic;
pub mod bsp;
pub mod cellular_automata;
pub mod drunkard_walk;

use basic::BasicMap;
use bsp::bsp::BspMap;
use bsp::bsp_interior::BspInteriorMap;
use cellular_automata::CellularAutomataMap;
use drunkard_walk::DrunkardWalkMap;

pub trait Architect {
    fn build(&mut self);
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
}