
pub mod testing;
pub mod bsp;
pub(crate) mod utils;

use testing::TestingMap;
use bsp::bsp::BspMap;
use bsp::bsp_interior::BspInteriorMap;

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
}