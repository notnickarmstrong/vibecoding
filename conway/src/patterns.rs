// Conway's Game of Life Pattern Library
// This module contains implementations of common Game of Life patterns

use crate::grid::Grid;

/// Structure representing a pattern that can be placed on the grid
pub struct Pattern {
    pub name: &'static str,
    pub description: &'static str,
    pub width: usize,
    pub height: usize,
    pub cells: Vec<(usize, usize)>,
}

impl Pattern {
    /// Place this pattern on the grid at the specified position
    pub fn place(&self, grid: &mut Grid, x: usize, y: usize) {
        // Clear the area
        for dy in 0..self.height {
            for dx in 0..self.width {
                if x + dx < grid.dimensions().0 && y + dy < grid.dimensions().1 {
                    grid.set(x + dx, y + dy, false);
                }
            }
        }
        
        // Place the pattern
        for &(px, py) in &self.cells {
            if x + px < grid.dimensions().0 && y + py < grid.dimensions().1 {
                grid.set(x + px, y + py, true);
            }
        }
    }
}

/// Collection of common patterns
pub struct PatternLibrary;

impl PatternLibrary {
    pub fn get_all_patterns() -> Vec<Pattern> {
        vec![
            Self::glider(),
            Self::blinker(),
            Self::toad(),
            Self::beacon(),
            Self::pulsar(),
            Self::glider_gun(),
            Self::lightweight_spaceship(),
            Self::r_pentomino(),
            Self::diehard(),
            Self::acorn(),
        ]
    }
    
    /// Get a pattern by name
    pub fn get_by_name(name: &str) -> Option<Pattern> {
        Self::get_all_patterns().into_iter().find(|p| p.name.to_lowercase() == name.to_lowercase())
    }
    
    /// Simple glider pattern
    pub fn glider() -> Pattern {
        Pattern {
            name: "Glider",
            description: "The smallest, most common spaceship",
            width: 3,
            height: 3,
            cells: vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)],
        }
    }
    
    /// Blinker oscillator pattern
    pub fn blinker() -> Pattern {
        Pattern {
            name: "Blinker",
            description: "The smallest oscillator with period 2",
            width: 3,
            height: 3,
            cells: vec![(1, 0), (1, 1), (1, 2)],
        }
    }
    
    /// Toad oscillator pattern
    pub fn toad() -> Pattern {
        Pattern {
            name: "Toad",
            description: "A period 2 oscillator",
            width: 4,
            height: 2,
            cells: vec![(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)],
        }
    }
    
    /// Beacon oscillator pattern
    pub fn beacon() -> Pattern {
        Pattern {
            name: "Beacon",
            description: "A period 2 oscillator",
            width: 4,
            height: 4,
            cells: vec![(0, 0), (1, 0), (0, 1), (3, 2), (2, 3), (3, 3)],
        }
    }
    
    /// Pulsar oscillator pattern
    pub fn pulsar() -> Pattern {
        Pattern {
            name: "Pulsar",
            description: "A period 3 oscillator",
            width: 13,
            height: 13,
            cells: vec![
                (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
                (0, 2), (5, 2), (7, 2), (12, 2),
                (0, 3), (5, 3), (7, 3), (12, 3),
                (0, 4), (5, 4), (7, 4), (12, 4),
                (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),
                (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
                (0, 8), (5, 8), (7, 8), (12, 8),
                (0, 9), (5, 9), (7, 9), (12, 9),
                (0, 10), (5, 10), (7, 10), (12, 10),
                (2, 12), (3, 12), (4, 12), (8, 12), (9, 12), (10, 12),
            ],
        }
    }
    
    /// Gosper's Glider Gun pattern
    pub fn glider_gun() -> Pattern {
        Pattern {
            name: "Glider Gun",
            description: "Gosper's Glider Gun - produces gliders periodically",
            width: 36,
            height: 9,
            cells: vec![
                (24, 0),
                (22, 1), (24, 1),
                (12, 2), (13, 2), (20, 2), (21, 2), (34, 2), (35, 2),
                (11, 3), (15, 3), (20, 3), (21, 3), (34, 3), (35, 3),
                (0, 4), (1, 4), (10, 4), (16, 4), (20, 4), (21, 4),
                (0, 5), (1, 5), (10, 5), (14, 5), (16, 5), (17, 5), (22, 5), (24, 5),
                (10, 6), (16, 6), (24, 6),
                (11, 7), (15, 7),
                (12, 8), (13, 8),
            ],
        }
    }
    
    /// Lightweight spaceship pattern
    pub fn lightweight_spaceship() -> Pattern {
        Pattern {
            name: "LWSS",
            description: "Lightweight Spaceship - moves across the grid",
            width: 5,
            height: 4,
            cells: vec![
                (1, 0), (4, 0),
                (0, 1),
                (0, 2), (4, 2),
                (0, 3), (1, 3), (2, 3), (3, 3),
            ],
        }
    }
    
    /// R-pentomino methuselah pattern
    pub fn r_pentomino() -> Pattern {
        Pattern {
            name: "R-pentomino",
            description: "A methuselah that evolves for many generations",
            width: 3,
            height: 3,
            cells: vec![(1, 0), (2, 0), (0, 1), (1, 1), (1, 2)],
        }
    }
    
    /// Diehard methuselah pattern
    pub fn diehard() -> Pattern {
        Pattern {
            name: "Diehard",
            description: "A methuselah that vanishes after 130 generations",
            width: 8,
            height: 3,
            cells: vec![(6, 0), (0, 1), (1, 1), (1, 2), (5, 2), (6, 2), (7, 2)],
        }
    }
    
    /// Acorn methuselah pattern
    pub fn acorn() -> Pattern {
        Pattern {
            name: "Acorn",
            description: "A methuselah that evolves for thousands of generations",
            width: 7,
            height: 3,
            cells: vec![(1, 0), (3, 1), (0, 2), (1, 2), (4, 2), (5, 2), (6, 2)],
        }
    }
}