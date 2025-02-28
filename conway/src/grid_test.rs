#[cfg(test)]
mod tests {
    use crate::grid::Grid;
    use crate::config::BoundaryType;

    #[test]
    fn test_new_grid() {
        let grid = Grid::new(10, 10, BoundaryType::Wrap);
        assert_eq!(grid.dimensions(), (10, 10));
        assert_eq!(grid.count_alive(), 0);
    }

    #[test]
    fn test_set_and_get() {
        let mut grid = Grid::new(10, 10, BoundaryType::Wrap);
        
        // All cells should be dead initially
        for y in 0..10 {
            for x in 0..10 {
                assert!(!grid.get(x, y));
            }
        }
        
        // Set some cells to alive
        grid.set(1, 1, true);
        grid.set(2, 2, true);
        grid.set(3, 3, true);
        
        // Verify those cells are alive
        assert!(grid.get(1, 1));
        assert!(grid.get(2, 2));
        assert!(grid.get(3, 3));
        
        // Count how many cells are alive
        assert_eq!(grid.count_alive(), 3);
    }

    #[test]
    fn test_toggle() {
        let mut grid = Grid::new(10, 10, BoundaryType::Wrap);
        
        // Toggle a cell to alive
        grid.toggle(5, 5);
        assert!(grid.get(5, 5));
        
        // Toggle it back to dead
        grid.toggle(5, 5);
        assert!(!grid.get(5, 5));
    }

    #[test]
    fn test_clear() {
        let mut grid = Grid::new(10, 10, BoundaryType::Wrap);
        
        // Set some cells to alive
        grid.set(1, 1, true);
        grid.set(2, 2, true);
        grid.set(3, 3, true);
        
        // Clear the grid
        grid.clear();
        
        // All cells should be dead
        for y in 0..10 {
            for x in 0..10 {
                assert!(!grid.get(x, y));
            }
        }
        
        assert_eq!(grid.count_alive(), 0);
    }

    #[test]
    fn test_count_neighbors_wrap() {
        let mut grid = Grid::new(10, 10, BoundaryType::Wrap);
        
        // Set up a pattern:
        // 1 1 0
        // 0 X 0
        // 0 0 1
        grid.set(0, 0, true);
        grid.set(1, 0, true);
        grid.set(2, 2, true);
        
        // The cell at (1, 1) should have 3 neighbors
        assert_eq!(grid.count_neighbors(1, 1), 3);
    }

    #[test]
    fn test_count_neighbors_fixed() {
        let mut grid = Grid::new(10, 10, BoundaryType::Fixed);
        
        // Set up a pattern at the corner:
        // X 1
        // 1 1
        grid.set(1, 0, true);
        grid.set(0, 1, true);
        grid.set(1, 1, true);
        
        // The cell at (0, 0) should have 3 neighbors
        assert_eq!(grid.count_neighbors(0, 0), 3);
    }

    #[test]
    fn test_update_rules() {
        let mut grid = Grid::new(10, 10, BoundaryType::Wrap);
        
        // Set up a blinker pattern:
        // 0 0 0
        // 1 1 1
        // 0 0 0
        grid.set(1, 1, true);
        grid.set(2, 1, true);
        grid.set(3, 1, true);
        
        // After one update, it should transform to:
        // 0 1 0
        // 0 1 0
        // 0 1 0
        grid.update();
        
        assert!(!grid.get(1, 1));
        assert!(grid.get(2, 0));
        assert!(grid.get(2, 1));
        assert!(grid.get(2, 2));
        assert!(!grid.get(3, 1));
        
        // After another update, it should go back to horizontal
        grid.update();
        
        assert!(grid.get(1, 1));
        assert!(grid.get(2, 1));
        assert!(grid.get(3, 1));
        assert!(!grid.get(2, 0));
        assert!(!grid.get(2, 2));
    }
}