// Conway's Game of Life Interactive Tutorial
// This module provides an interactive tutorial for learning about Conway's Game of Life

use std::collections::HashMap;
use std::time::Duration;
use std::thread;

use crate::grid::Grid;
use crate::config::BoundaryType;
use crate::patterns::Pattern;

// Tutorial step structure
pub struct TutorialStep {
    pub title: &'static str,
    pub description: &'static str,
    pub grid_config: GridConfig,
    pub actions: Vec<Action>,
    pub expected_outcome: Option<Outcome>,
    pub next_steps: Vec<usize>,
}

// Grid configuration for a tutorial step
pub struct GridConfig {
    pub width: usize,
    pub height: usize,
    pub initial_patterns: Vec<(Pattern, usize, usize)>, // Pattern and position (x, y)
    pub boundary: BoundaryType,
}

// Actions that can be performed in a tutorial step
#[derive(Clone)]
pub enum Action {
    Wait(usize),        // Wait for a number of generations
    SetCells(Vec<(usize, usize)>, bool), // Set cells at positions to a state
    RunUntilStable,     // Run until the grid stabilizes
    Observe(&'static str), // Observe a specific phenomenon
    UserInput(UserInputType), // Wait for user input
}

// Types of user input
#[derive(Clone)]
pub enum UserInputType {
    AnyKey,
    SpecificKey(char),
    Position,
}

// Expected outcomes
pub struct Outcome {
    pub description: &'static str,
    pub grid_state: Option<HashMap<(usize, usize), bool>>, // Optional expected grid state
    pub stable_after: Option<usize>,     // Stable after n generations
    pub oscillator_period: Option<usize>, // Oscillates with period n
}

// Tutorial manager
pub struct Tutorial {
    steps: Vec<TutorialStep>,
    current_step: usize,
    grid: Grid,
}

impl Tutorial {
    // Create a new tutorial
    pub fn new() -> Self {
        let steps = Self::create_tutorial_steps();
        let first_step = &steps[0];
        let grid_config = &first_step.grid_config;
        
        let mut grid = Grid::new(
            grid_config.width,
            grid_config.height,
            grid_config.boundary.clone(),
        );
        
        // Apply initial patterns
        for (pattern, x, y) in &grid_config.initial_patterns {
            pattern.place(&mut grid, *x, *y);
        }
        
        Self {
            steps,
            current_step: 0,
            grid,
        }
    }
    
    // Get current tutorial step
    pub fn current_step(&self) -> &TutorialStep {
        &self.steps[self.current_step]
    }
    
    // Move to the next step
    pub fn next_step(&mut self, choice: usize) -> bool {
        let current = &self.steps[self.current_step];
        
        if choice >= current.next_steps.len() {
            return false;
        }
        
        // Update current step
        self.current_step = current.next_steps[choice];
        
        // Setup grid for new step
        let grid_config = &self.steps[self.current_step].grid_config;
        
        self.grid = Grid::new(
            grid_config.width,
            grid_config.height,
            grid_config.boundary.clone(),
        );
        
        // Apply initial patterns
        for (pattern, x, y) in &grid_config.initial_patterns {
            pattern.place(&mut self.grid, *x, *y);
        }
        
        true
    }
    
    // Get current grid
    pub fn grid(&self) -> &Grid {
        &self.grid
    }
    
    // Get mutable grid
    pub fn grid_mut(&mut self) -> &mut Grid {
        &mut self.grid
    }
    
    // Execute actions for the current step
    pub fn execute_actions(&mut self, action_index: usize) -> bool {
        if action_index >= self.current_step().actions.len() {
            return false;
        }
        
        // Clone the action to avoid borrowing issues
        let action = self.current_step().actions[action_index].clone();
        
        match action {
            Action::Wait(generations) => {
                for _ in 0..generations {
                    self.grid.update();
                    thread::sleep(Duration::from_millis(100));
                }
            },
            Action::SetCells(cells, state) => {
                for (x, y) in cells {
                    self.grid.set(x, y, state);
                }
            },
            Action::RunUntilStable => {
                let mut prev_state = self.grid.count_alive();
                let mut generations = 0;
                let max_generations = 1000; // Safety limit
                
                while generations < max_generations {
                    self.grid.update();
                    
                    let current_state = self.grid.count_alive();
                    if current_state == prev_state {
                        break;
                    }
                    
                    prev_state = current_state;
                    generations += 1;
                    
                    thread::sleep(Duration::from_millis(50));
                }
            },
            Action::Observe(_) => {
                // This is just a marker, no actual action
                thread::sleep(Duration::from_secs(2));
            },
            Action::UserInput(_) => {
                // Handled by the UI
            },
        }
        
        true
    }
    
    // Verify if the expected outcome has been achieved
    pub fn verify_outcome(&self) -> bool {
        if let Some(outcome) = &self.current_step().expected_outcome {
            if let Some(grid_state) = &outcome.grid_state {
                // Check if grid matches expected state
                for ((x, y), expected) in grid_state {
                    if self.grid.get(*x, *y) != *expected {
                        return false;
                    }
                }
            }
            
            // More checks could be added here
            
            true
        } else {
            // No expected outcome, so consider it achieved
            true
        }
    }
    
    // Define all tutorial steps
    fn create_tutorial_steps() -> Vec<TutorialStep> {
        vec![
            // Introduction
            TutorialStep {
                title: "Introduction to Conway's Game of Life",
                description: "Conway's Game of Life is a cellular automaton devised by mathematician John Conway in 1970. It consists of a grid of cells, each of which can be alive or dead. The grid evolves according to simple rules based on the state of neighboring cells.",
                grid_config: GridConfig {
                    width: 20,
                    height: 20,
                    initial_patterns: vec![],
                    boundary: BoundaryType::Wrap,
                },
                actions: vec![
                    Action::UserInput(UserInputType::AnyKey),
                ],
                expected_outcome: None,
                next_steps: vec![1],
            },
            
            // Rules
            TutorialStep {
                title: "The Rules",
                description: "The rules of the Game of Life are simple:\n1. Any live cell with fewer than two live neighbors dies (underpopulation)\n2. Any live cell with two or three live neighbors lives on\n3. Any live cell with more than three live neighbors dies (overpopulation)\n4. Any dead cell with exactly three live neighbors becomes alive (reproduction)",
                grid_config: GridConfig {
                    width: 20,
                    height: 20,
                    initial_patterns: vec![],
                    boundary: BoundaryType::Wrap,
                },
                actions: vec![
                    Action::SetCells(vec![(9, 9), (10, 9), (11, 9)], true),
                    Action::UserInput(UserInputType::AnyKey),
                    Action::Wait(1),
                    Action::Observe("Notice how the pattern changes from a horizontal line to a vertical line"),
                    Action::Wait(1),
                    Action::Observe("The pattern oscillates between these two states - this is called a 'blinker'"),
                ],
                expected_outcome: Some(Outcome {
                    description: "The blinker pattern oscillates between horizontal and vertical orientations.",
                    grid_state: None,
                    stable_after: None,
                    oscillator_period: Some(2),
                }),
                next_steps: vec![2],
            },
            
            // Still Lifes
            TutorialStep {
                title: "Still Lifes",
                description: "Still lifes are patterns that do not change from one generation to the next. They are stable configurations where each live cell has exactly 2 or 3 live neighbors.",
                grid_config: GridConfig {
                    width: 30,
                    height: 20,
                    initial_patterns: vec![],
                    boundary: BoundaryType::Wrap,
                },
                actions: vec![
                    // Block pattern
                    Action::SetCells(vec![(5, 5), (5, 6), (6, 5), (6, 6)], true),
                    Action::Observe("This is a 'block', the simplest still life"),
                    
                    // Beehive pattern
                    Action::SetCells(vec![(15, 5), (16, 4), (17, 4), (18, 5), (17, 6), (16, 6)], true),
                    Action::Observe("This is a 'beehive', another common still life"),
                    
                    // Loaf pattern
                    Action::SetCells(vec![(25, 5), (26, 4), (27, 4), (28, 5), (27, 6), (26, 6), (27, 7)], true),
                    Action::Observe("This is a 'loaf', another stable pattern"),
                    
                    Action::Wait(5),
                    Action::Observe("Notice that none of these patterns change over time"),
                ],
                expected_outcome: Some(Outcome {
                    description: "The still life patterns remain unchanged.",
                    grid_state: None,
                    stable_after: Some(1),
                    oscillator_period: None,
                }),
                next_steps: vec![3],
            },
            
            // Oscillators
            TutorialStep {
                title: "Oscillators",
                description: "Oscillators are patterns that cycle through a fixed sequence of states, eventually returning to their initial configuration.",
                grid_config: GridConfig {
                    width: 40,
                    height: 20,
                    initial_patterns: vec![],
                    boundary: BoundaryType::Wrap,
                },
                actions: vec![
                    // Blinker
                    Action::SetCells(vec![(5, 5), (6, 5), (7, 5)], true),
                    Action::Observe("The 'blinker' oscillates with period 2"),
                    
                    // Toad
                    Action::SetCells(vec![(15, 5), (16, 5), (17, 5), (14, 6), (15, 6), (16, 6)], true),
                    Action::Observe("The 'toad' also oscillates with period 2"),
                    
                    // Beacon
                    Action::SetCells(vec![(25, 5), (26, 5), (25, 6), (28, 7), (29, 8), (28, 8)], true),
                    Action::Observe("The 'beacon' oscillates with period 2 as well"),
                    
                    // Pulsar
                    Action::SetCells(vec![
                        (32, 2), (33, 2), (34, 2), (38, 2), (39, 2), (40, 2),
                        (30, 4), (35, 4), (37, 4), (42, 4),
                        (30, 5), (35, 5), (37, 5), (42, 5),
                        (30, 6), (35, 6), (37, 6), (42, 6),
                        (32, 7), (33, 7), (34, 7), (38, 7), (39, 7), (40, 7),
                        (32, 9), (33, 9), (34, 9), (38, 9), (39, 9), (40, 9),
                        (30, 10), (35, 10), (37, 10), (42, 10),
                        (30, 11), (35, 11), (37, 11), (42, 11),
                        (30, 12), (35, 12), (37, 12), (42, 12),
                        (32, 14), (33, 14), (34, 14), (38, 14), (39, 14), (40, 14),
                    ], true),
                    Action::Observe("The 'pulsar' is a larger oscillator with period 3"),
                    
                    Action::Wait(10),
                    Action::Observe("Watch as these patterns cycle through their states"),
                ],
                expected_outcome: None,
                next_steps: vec![4],
            },
            
            // Spaceships
            TutorialStep {
                title: "Spaceships",
                description: "Spaceships are patterns that translate across the grid, returning to their original shape but in a different location.",
                grid_config: GridConfig {
                    width: 40,
                    height: 20,
                    initial_patterns: vec![],
                    boundary: BoundaryType::Wrap,
                },
                actions: vec![
                    // Glider
                    Action::SetCells(vec![(5, 5), (6, 6), (7, 6), (5, 7), (6, 7)], true),
                    Action::Observe("This is a 'glider', the smallest spaceship"),
                    
                    // Lightweight spaceship
                    Action::SetCells(vec![(15, 5), (18, 5), (14, 6), (14, 7), (18, 7), (14, 8), (15, 8), (16, 8), (17, 8)], true),
                    Action::Observe("This is a 'lightweight spaceship' (LWSS)"),
                    
                    Action::Wait(20),
                    Action::Observe("Watch as these patterns move across the grid"),
                ],
                expected_outcome: None,
                next_steps: vec![5],
            },
            
            // Methuselahs
            TutorialStep {
                title: "Methuselahs",
                description: "Methuselahs are small patterns that take a long time to stabilize, often creating complex structures in the process.",
                grid_config: GridConfig {
                    width: 40,
                    height: 30,
                    initial_patterns: vec![],
                    boundary: BoundaryType::Wrap,
                },
                actions: vec![
                    // R-pentomino
                    Action::SetCells(vec![(20, 15), (21, 15), (19, 16), (20, 16), (20, 17)], true),
                    Action::Observe("This is the 'R-pentomino', which takes 1103 generations to stabilize"),
                    
                    Action::Wait(50),
                    Action::Observe("The R-pentomino produces a complex pattern that evolves for many generations"),
                ],
                expected_outcome: None,
                next_steps: vec![6],
            },
            
            // Guns and Puffers
            TutorialStep {
                title: "Guns and Puffers",
                description: "Guns are patterns that periodically emit spaceships. Puffers are moving patterns that leave debris behind them.",
                grid_config: GridConfig {
                    width: 50,
                    height: 30,
                    initial_patterns: vec![],
                    boundary: BoundaryType::Wrap,
                },
                actions: vec![
                    // Gosper Glider Gun
                    Action::SetCells(vec![
                        (25, 5),
                        (23, 6), (25, 6),
                        (13, 7), (14, 7), (21, 7), (22, 7), (35, 7), (36, 7),
                        (12, 8), (16, 8), (21, 8), (22, 8), (35, 8), (36, 8),
                        (1, 9), (2, 9), (11, 9), (17, 9), (21, 9), (22, 9),
                        (1, 10), (2, 10), (11, 10), (15, 10), (17, 10), (18, 10), (23, 10), (25, 10),
                        (11, 11), (17, 11), (25, 11),
                        (12, 12), (16, 12),
                        (13, 13), (14, 13),
                    ], true),
                    Action::Observe("This is the 'Gosper Glider Gun', which emits a glider every 30 generations"),
                    
                    Action::Wait(100),
                    Action::Observe("Watch as the gun produces a stream of gliders"),
                ],
                expected_outcome: None,
                next_steps: vec![7],
            },
            
            // Conclusion
            TutorialStep {
                title: "Conclusion",
                description: "You've now seen the basic patterns and behaviors in Conway's Game of Life. Despite its simple rules, the Game of Life is Turing complete, meaning it can simulate any computer algorithm. Explore more by creating your own patterns or searching for famous discoveries!",
                grid_config: GridConfig {
                    width: 40,
                    height: 20,
                    initial_patterns: vec![],
                    boundary: BoundaryType::Wrap,
                },
                actions: vec![
                    Action::UserInput(UserInputType::AnyKey),
                ],
                expected_outcome: None,
                next_steps: vec![0], // Loop back to start
            },
        ]
    }
}