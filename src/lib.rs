mod maps;

use maps::*;

#[cfg(test)]
mod tests {
    use super::maps::{Architect, MapSelector};
    use super::maps::utils::output_map;

    #[test]
    fn build_test_map() {
        let mut m = MapSelector::basic_map(100, 100);
        m.build();
        output_map(&m.map, "basic_map.txt");
    }

    #[test]
    fn build_bsp_map() {
        let mut m = MapSelector::bsp_map(300, 300);
        m.build();
        output_map(&m.map, "bsp_map.txt")
    }

    #[test]
    fn build_bsp_interior_map() {
        let mut m = MapSelector::bsp_interior_map(300, 300);
        m.build();
        output_map(&m.map, "bsp_interior_map.txt");
    }

    #[test]
    fn build_cellular_automata_map() {
        let mut m = MapSelector::cellular_automata_map(300, 300);
        m.build();
        output_map(&m.map, "cellular_automata_map.txt");
    }

    #[test]
    fn build_drunkard_walk_map_open_area() {
        let mut m = MapSelector::drunkard_walk_map_open_area(80, 50);
        m.build();
        output_map(&m.map, "drunkard_walk_map_open_area.txt");
    }

    #[test]
    fn build_drunkard_walk_map_open_halls() {
        let mut m = MapSelector::drunkard_walk_map_open_halls(100, 100);
        m.build();
        output_map(&m.map, "drunkard_walk_map_open_halls.txt");
    }

    #[test]
    fn build_drunkard_walk_map_winding_passages() {
        let mut m = MapSelector::drunkard_walk_map_winding_passages(100, 100);
        m.build();
        output_map(&m.map, "drunkard_walk_map_winding_passages.txt");
    }

    #[test]
    fn build_drunkard_walk_map_fat_passages() {
        let mut m = MapSelector::drunkard_walk_fat_passages(100, 100);
        m.build();
        output_map(&m.map, "drunkard_walk_map_fat_passages.txt");
    }

    #[test]
    fn build_drunkard_walk_map_fearful_symmetry() {
        let mut m = MapSelector::drunkard_walk_fearful_symmetry(100, 100);
        m.build();
        output_map(&m.map, "drunkard_walk_map_fearful_symmetry.txt");
    }

    #[test]
    fn build_maze_map() {
        let mut m = MapSelector::maze_map(80, 40);
        m.build();
        output_map(&m.map, "maze_map.txt");
    }

    #[test]
    fn build_dla_map_walk_inwards() {
        let mut m = MapSelector::dla_map_walk_inwards(100, 100);
        m.build();
        output_map(&m.map, "dla_map_walk_inwards.txt")
    }

    #[test]
    fn build_dla_map_walk_outwards() {
        let mut m = MapSelector::dla_map_walk_outwards(100, 100);
        m.build();
        output_map(&m.map, "dla_map_walk_outwards.txt")
    }

    #[test]
    fn build_dla_map_central_attractor() {
        let mut m = MapSelector::dla_map_central_attractor(100, 100);
        m.build();
        output_map(&m.map, "dla_map_central_attractor.txt")
    }

    #[test]
    fn build_dla_map_insectoid() {
        let mut m = MapSelector::dla_map_insectoid(100, 100);
        m.build();
        output_map(&m.map, "dla_map_insectoid.txt")
    }

    #[test]
    fn build_voronoi_cell_map_pythagoras() {
        let mut m = MapSelector::voronoi_cell_map_pythagoras(100, 100);
        m.build();
        output_map(&m.map, "voronoi_cell_map_pythagoras.txt")
    }

    #[test]
    fn build_voronoi_cell_map_manhattan() {
        let mut m = MapSelector::voronoi_cell_map_manhattan(100, 100);
        m.build();
        output_map(&m.map, "voronoi_cell_map_manhattan.txt")
    }

    #[test]
    fn build_random_map() {
        let mut m = MapSelector::random_map(100, 100);
        m.build();
        output_map(m.get_map(), "random_map.txt");
    }
}
