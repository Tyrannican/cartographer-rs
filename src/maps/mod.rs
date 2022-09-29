
pub mod testing;
pub mod bsp;
pub mod cellular_automata;
pub mod utils;

use testing::TestingMap;
use bsp::bsp::BspMap;
use bsp::bsp_interior::BspInteriorMap;

use self::cellular_automata::CellularAutomataMap;

pub trait Architect {
    fn build(&mut self);
}

pub struct MapSelector;

impl MapSelector {
    pub fn testing_map(width: i32, height: i32) -> TestingMap {
        TestingMap::new(width, height)
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
}