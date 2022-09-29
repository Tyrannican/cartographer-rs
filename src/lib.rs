mod maps;

#[cfg(test)]
mod tests {
    use super::maps::{Architect, MapSelector};

    #[test]
    fn build_test_map() {
        let mut m = MapSelector::basic_map(300, 300);
        m.build();
        m.map.output_map("test_map.txt");
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
        let mut m = MapSelector::drunkard_walk_map_open_area(100, 100);
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
}
