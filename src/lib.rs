mod maps;

#[cfg(test)]
mod tests {
    use super::maps::{Architect, MapSelector};

    #[test]
    fn build_test_map() {
        let mut m = MapSelector::basic_map(100, 100);
        m.build();
        m.map.output_map("basic_map.txt");
    }

    #[test]
    fn build_bsp_map() {
        let mut m = MapSelector::bsp_map(300, 300);
        m.build();
        m.map.output_map("bsp_map.txt")
    }

    #[test]
    fn build_bsp_interior_map() {
        let mut m = MapSelector::bsp_interior_map(300, 300);
        m.build();
        m.map.output_map("bsp_interior_map.txt");
    }

    #[test]
    fn build_cellular_automata_map() {
        let mut m = MapSelector::cellular_automata_map(300, 300);
        m.build();
        m.map.output_map("cellular_automata_map.txt");
    }

    #[test]
    fn build_drunkard_walk_map_open_area() {
        let mut m = MapSelector::drunkard_walk_map_open_area(80, 50);
        m.build();
        m.map.output_map("drunkard_walk_map_open_area.txt");
    }

    #[test]
    fn build_drunkard_walk_map_open_halls() {
        let mut m = MapSelector::drunkard_walk_map_open_halls(100, 100);
        m.build();
        m.map.output_map("drunkard_walk_map_open_halls.txt");
    }

    #[test]
    fn build_drunkard_walk_map_winding_passages() {
        let mut m = MapSelector::drunkard_walk_map_winding_passages(100, 100);
        m.build();
        m.map.output_map("drunkard_walk_map_winding_passages.txt");
    }

    #[test]
    fn build_maze_map() {
        let mut m = MapSelector::maze_map(80, 40);
        m.build();
        m.map.output_map("maze_map.txt");
    }

    #[test]
    fn build_dla_map_walk_inwards() {
        let mut m = MapSelector::dla_map_walk_inwards(100, 100);
        m.build();
        m.map.output_map("dla_map_walk_inwards.txt")
    }

    #[test]
    fn build_dla_map_walk_outwards() {
        let mut m = MapSelector::dla_map_walk_outwards(100, 100);
        m.build();
        m.map.output_map("dla_map_walk_outwards.txt")
    }

    #[test]
    fn build_dla_map_central_attractor() {
        let mut m = MapSelector::dla_map_central_attractor(100, 100);
        m.build();
        m.map.output_map("dla_map_central_attractor.txt")
    }

    #[test]
    fn build_dla_map_insectoid() {
        let mut m = MapSelector::dla_map_insectoid(100, 100);
        m.build();
        m.map.output_map("dla_map_insectoid.txt")
    }
}
