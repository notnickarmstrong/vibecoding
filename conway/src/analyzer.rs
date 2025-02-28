// Conway's Game of Life Pattern Analyzer
// Analyzes patterns and their behavior over time

use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::grid::Grid;
use crate::patterns::Pattern;
use crate::config::BoundaryType;

/// Represents the life cycle classification of a pattern
#[derive(Debug, Clone, PartialEq)]
pub enum PatternType {
    /// Pattern that dies out completely
    ExtinctPattern {
        generations_to_extinction: usize,
    },
    /// Pattern that stabilizes into still lifes and oscillators
    StablePattern {
        generations_to_stabilize: usize,
        oscillator_period: Option<usize>,
        final_population: usize,
    },
    /// Pattern that grows indefinitely or beyond analysis bounds
    ExplodingPattern {
        average_growth_rate: f64,
    },
    /// Pattern that moves across the grid (spaceship)
    SpaceshipPattern {
        period: usize,
        displacement: (isize, isize),  // (dx, dy) per period
        speed: f64,                    // cells per generation
    },
    /// Pattern that periodically emits other patterns
    PatternEmitter {
        period: usize,
        emitted_pattern_type: Box<PatternType>,
    },
    /// Unclassified pattern
    Unknown,
}

/// Detailed statistics about a pattern's evolution
#[derive(Debug, Clone)]
pub struct PatternStats {
    pub name: String,
    pub initial_population: usize,
    pub max_population: usize,
    pub generation_of_max: usize,
    pub final_population: usize,
    pub generations_analyzed: usize,
    pub pattern_type: PatternType,
    pub stable_formations: HashMap<String, usize>, // Formation name -> count
    pub population_history: Vec<usize>,
    pub analysis_duration: Duration,
}

impl PatternStats {
    pub fn new(name: &str, initial_population: usize) -> Self {
        Self {
            name: name.to_string(),
            initial_population,
            max_population: initial_population,
            generation_of_max: 0,
            final_population: initial_population,
            generations_analyzed: 0,
            pattern_type: PatternType::Unknown,
            stable_formations: HashMap::new(),
            population_history: vec![initial_population],
            analysis_duration: Duration::from_secs(0),
        }
    }
    
    /// Generate a report of the pattern statistics
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str(&format!("Pattern Analysis: {}\n", self.name));
        report.push_str(&format!("===================={}\n\n", "=".repeat(self.name.len())));
        
        report.push_str(&format!("Initial population: {}\n", self.initial_population));
        report.push_str(&format!("Final population: {}\n", self.final_population));
        report.push_str(&format!("Maximum population: {} (generation {})\n", self.max_population, self.generation_of_max));
        report.push_str(&format!("Generations analyzed: {}\n", self.generations_analyzed));
        report.push_str(&format!("Analysis duration: {:.2?}\n\n", self.analysis_duration));
        
        report.push_str("Pattern classification: ");
        match &self.pattern_type {
            PatternType::ExtinctPattern { generations_to_extinction } => {
                report.push_str(&format!("Extinct (died out after {} generations)\n", generations_to_extinction));
            },
            PatternType::StablePattern { generations_to_stabilize, oscillator_period, final_population } => {
                if let Some(period) = oscillator_period {
                    report.push_str(&format!("Oscillator with period {} (stabilized after {} generations)\n", 
                        period, generations_to_stabilize));
                } else {
                    report.push_str(&format!("Still life (stabilized after {} generations)\n", 
                        generations_to_stabilize));
                }
                report.push_str(&format!("Final stable population: {}\n", final_population));
            },
            PatternType::ExplodingPattern { average_growth_rate } => {
                report.push_str(&format!("Exploding pattern (average growth rate: {:.2} cells/generation)\n", 
                    average_growth_rate));
            },
            PatternType::SpaceshipPattern { period, displacement, speed } => {
                report.push_str(&format!("Spaceship with period {} and displacement ({}, {})\n", 
                    period, displacement.0, displacement.1));
                report.push_str(&format!("Speed: {:.2} cells/generation\n", speed));
            },
            PatternType::PatternEmitter { period, emitted_pattern_type } => {
                report.push_str(&format!("Pattern emitter with period {}\n", period));
                report.push_str(&format!("Emits: {:?}\n", *emitted_pattern_type));
            },
            PatternType::Unknown => {
                report.push_str("Unknown pattern type\n");
            },
        }
        
        if !self.stable_formations.is_empty() {
            report.push_str("\nStable formations detected:\n");
            for (formation, count) in &self.stable_formations {
                report.push_str(&format!("  - {} × {}\n", count, formation));
            }
        }
        
        // Add population history graph if not too large
        if self.population_history.len() <= 100 {
            report.push_str("\nPopulation history:\n");
            
            // Find max for scaling
            let max_pop = self.population_history.iter().max().unwrap_or(&1);
            let scale_factor = 40.0 / *max_pop as f64;
            
            for (generation, &pop) in self.population_history.iter().enumerate() {
                let bar_length = (pop as f64 * scale_factor).round() as usize;
                report.push_str(&format!("{:4}: {:5} {}\n", 
                    generation, pop, "#".repeat(bar_length)));
            }
        } else {
            // Just show key points for larger histories
            report.push_str("\nPopulation key points:\n");
            
            // Start
            report.push_str(&format!("Generation {:4}: {}\n", 0, self.population_history[0]));
            
            // Max
            report.push_str(&format!("Generation {:4}: {} (maximum)\n", 
                self.generation_of_max, self.max_population));
            
            // Every 25% point
            let step = self.generations_analyzed / 4;
            for i in 1..4 {
                let generation = i * step;
                if generation < self.population_history.len() {
                    report.push_str(&format!("Generation {:4}: {}\n", 
                        generation, self.population_history[generation]));
                }
            }
            
            // End
            report.push_str(&format!("Generation {:4}: {}\n", 
                self.generations_analyzed, self.final_population));
        }
        
        report
    }
}

/// A pattern analyzer for Conway's Game of Life
pub struct PatternAnalyzer {
    max_generations: usize,
    grid_size: (usize, usize),
    boundary: BoundaryType,
}

impl PatternAnalyzer {
    pub fn new(max_generations: usize, grid_size: (usize, usize), boundary: BoundaryType) -> Self {
        Self {
            max_generations,
            grid_size,
            boundary,
        }
    }
    
    /// Analyze a pattern and return detailed statistics
    pub fn analyze_pattern(&self, pattern: &Pattern, x: usize, y: usize) -> PatternStats {
        let start_time = Instant::now();
        
        // Create a grid and place the pattern
        let mut grid = Grid::new(self.grid_size.0, self.grid_size.1, self.boundary.clone());
        pattern.place(&mut grid, x, y);
        
        // Initialize stats
        let initial_population = grid.count_alive();
        let mut stats = PatternStats::new(&pattern.name, initial_population);
        
        // Track grid hashes to detect cycles
        let mut grid_history: HashMap<u64, usize> = HashMap::new();
        let mut hash = self.hash_grid(&grid);
        grid_history.insert(hash, 0);
        
        // Track pattern center and detect movement
        let mut center_history: Vec<(usize, usize)> = Vec::new();
        center_history.push(self.find_pattern_center(&grid));
        
        for generation in 1..=self.max_generations {
            // Update the grid
            grid.update();
            
            // Update population stats
            let population = grid.count_alive();
            stats.population_history.push(population);
            
            if population > stats.max_population {
                stats.max_population = population;
                stats.generation_of_max = generation;
            }
            
            // Find pattern center
            center_history.push(self.find_pattern_center(&grid));
            
            // Check for extinction
            if population == 0 {
                stats.pattern_type = PatternType::ExtinctPattern {
                    generations_to_extinction: generation,
                };
                break;
            }
            
            // Check for cycles (stable patterns)
            hash = self.hash_grid(&grid);
            if let Some(previous_gen) = grid_history.get(&hash) {
                let period = generation - previous_gen;
                
                // Determine if it's a still life or oscillator
                if period == 1 {
                    stats.pattern_type = PatternType::StablePattern {
                        generations_to_stabilize: generation - 1,
                        oscillator_period: None,
                        final_population: population,
                    };
                } else {
                    stats.pattern_type = PatternType::StablePattern {
                        generations_to_stabilize: *previous_gen,
                        oscillator_period: Some(period),
                        final_population: population,
                    };
                }
                
                break;
            }
            
            // Check for spaceships (moving stable patterns)
            if center_history.len() > 10 {
                if let Some(spaceship_info) = self.detect_spaceship(&center_history, &stats.population_history) {
                    stats.pattern_type = spaceship_info;
                    break;
                }
            }
            
            // Detect if it's an exploding pattern (significant growth over time)
            if generation > 50 && population > initial_population * 2 {
                let growth_rate = (population - initial_population) as f64 / generation as f64;
                
                if growth_rate > 0.1 {
                    stats.pattern_type = PatternType::ExplodingPattern {
                        average_growth_rate: growth_rate,
                    };
                    break;
                }
            }
            
            // Store grid hash for cycle detection
            grid_history.insert(hash, generation);
        }
        
        // Update final stats
        stats.generations_analyzed = stats.population_history.len() - 1;
        stats.final_population = *stats.population_history.last().unwrap_or(&0);
        stats.analysis_duration = start_time.elapsed();
        
        // Identify stable formations
        if let PatternType::StablePattern { .. } = stats.pattern_type {
            stats.stable_formations = self.identify_stable_formations(&grid);
        }
        
        stats
    }
    
    /// Analyze multiple patterns and compare their behavior
    pub fn compare_patterns(&self, patterns: &[(&Pattern, usize, usize)]) -> Vec<PatternStats> {
        patterns.iter()
            .map(|(pattern, x, y)| self.analyze_pattern(pattern, *x, *y))
            .collect()
    }
    
    /// Calculate a hash of the grid state for cycle detection
    fn hash_grid(&self, grid: &Grid) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        
        for y in 0..self.grid_size.1 {
            for x in 0..self.grid_size.0 {
                grid.get(x, y).hash(&mut hasher);
            }
        }
        
        hasher.finish()
    }
    
    /// Find the center point of a pattern
    fn find_pattern_center(&self, grid: &Grid) -> (usize, usize) {
        let mut sum_x = 0;
        let mut sum_y = 0;
        let mut count = 0;
        
        for y in 0..self.grid_size.1 {
            for x in 0..self.grid_size.0 {
                if grid.get(x, y) {
                    sum_x += x;
                    sum_y += y;
                    count += 1;
                }
            }
        }
        
        if count == 0 {
            (self.grid_size.0 / 2, self.grid_size.1 / 2)
        } else {
            (sum_x / count, sum_y / count)
        }
    }
    
    /// Detect if a pattern is a spaceship
    fn detect_spaceship(
        &self, 
        center_history: &[(usize, usize)], 
        population_history: &[usize]
    ) -> Option<PatternType> {
        // Need enough history to detect movement
        if center_history.len() < 10 {
            return None;
        }
        
        // Check if population is stable
        let recent_populations = &population_history[population_history.len() - 10..];
        let population_stable = recent_populations.windows(2)
            .all(|w| w[0] == w[1]);
            
        if !population_stable {
            return None;
        }
        
        // Look for cyclic movement
        for period in 2..=10 {
            if center_history.len() <= period * 2 {
                continue;
            }
            
            let samples = center_history.len() / period;
            if samples < 2 {
                continue;
            }
            
            let mut displacements = Vec::new();
            
            for i in 0..samples {
                let pos1 = center_history[i * period];
                let pos2 = center_history[(i + 1) * period];
                
                let dx = pos2.0 as isize - pos1.0 as isize;
                let dy = pos2.1 as isize - pos1.1 as isize;
                
                displacements.push((dx, dy));
            }
            
            // Check if all displacements are the same
            if displacements.windows(2).all(|w| w[0] == w[1]) {
                let displacement = displacements[0];
                
                // Calculate speed
                let distance = ((displacement.0.pow(2) + displacement.1.pow(2)) as f64).sqrt();
                let speed = distance / period as f64;
                
                return Some(PatternType::SpaceshipPattern {
                    period,
                    displacement,
                    speed,
                });
            }
        }
        
        None
    }
    
    /// Identify common stable formations in the grid
    fn identify_stable_formations(&self, grid: &Grid) -> HashMap<String, usize> {
        let mut formations = HashMap::new();
        
        // Define common still lifes
        let block = "Block";
        let _beehive = "Beehive";  // Reserved for future implementation
        let _loaf = "Loaf";        // Reserved for future implementation
        let _boat = "Boat";        // Reserved for future implementation
        let _tub = "Tub";          // Reserved for future implementation
        
        // Define common oscillators
        let blinker = "Blinker";
        let _toad = "Toad";        // Reserved for future implementation
        let _beacon = "Beacon";    // Reserved for future implementation
        
        // Scan grid for patterns (simplified detection)
        for y in 1..self.grid_size.1 - 2 {
            for x in 1..self.grid_size.0 - 2 {
                // Check for a block
                if x < self.grid_size.0 - 1 && y < self.grid_size.1 - 1 &&
                   grid.get(x, y) && grid.get(x + 1, y) && 
                   grid.get(x, y + 1) && grid.get(x + 1, y + 1) {
                    *formations.entry(block.to_string()).or_insert(0) += 1;
                    continue;
                }
                
                // Check for a blinker (horizontal)
                if x < self.grid_size.0 - 2 &&
                   grid.get(x, y) && grid.get(x + 1, y) && grid.get(x + 2, y) &&
                   !grid.get(x, y - 1) && !grid.get(x + 1, y - 1) && !grid.get(x + 2, y - 1) &&
                   !grid.get(x, y + 1) && !grid.get(x + 1, y + 1) && !grid.get(x + 2, y + 1) {
                    *formations.entry(blinker.to_string()).or_insert(0) += 1;
                    continue;
                }
                
                // Other patterns can be added with more complex detection logic
            }
        }
        
        formations
    }
    
    /// Generate a comparison report for multiple patterns
    pub fn generate_comparison_report(&self, stats: &[PatternStats]) -> String {
        if stats.is_empty() {
            return "No patterns to compare.".to_string();
        }
        
        let mut report = String::new();
        
        report.push_str("Pattern Comparison Report\n");
        report.push_str("========================\n\n");
        
        // Table header
        report.push_str(&format!("{:<20} | {:<15} | {:<15} | {:<15} | {:<15}\n", 
            "Pattern", "Init. Pop.", "Max. Pop.", "Final Pop.", "Classification"));
        report.push_str(&format!("{:<20} | {:<15} | {:<15} | {:<15} | {:<15}\n", 
            "-".repeat(20), "-".repeat(15), "-".repeat(15), "-".repeat(15), "-".repeat(15)));
            
        // Table rows
        for stat in stats {
            let classification = match &stat.pattern_type {
                PatternType::ExtinctPattern { .. } => "Extinct",
                PatternType::StablePattern { oscillator_period: None, .. } => "Still Life",
                PatternType::StablePattern { oscillator_period: Some(p), .. } => 
                    &format!("Oscillator (p={})", p),
                PatternType::ExplodingPattern { .. } => "Exploding",
                PatternType::SpaceshipPattern { .. } => "Spaceship",
                PatternType::PatternEmitter { .. } => "Emitter",
                PatternType::Unknown => "Unknown",
            };
            
            report.push_str(&format!("{:<20} | {:<15} | {:<15} | {:<15} | {:<15}\n", 
                stat.name, stat.initial_population, stat.max_population, 
                stat.final_population, classification));
        }
        
        // Add growth rate comparison if there are exploding patterns
        let exploding_patterns: Vec<_> = stats.iter()
            .filter_map(|stat| {
                if let PatternType::ExplodingPattern { average_growth_rate } = stat.pattern_type {
                    Some((stat.name.clone(), average_growth_rate))
                } else {
                    None
                }
            })
            .collect();
            
        if !exploding_patterns.is_empty() {
            report.push_str("\nGrowth Rate Comparison:\n");
            report.push_str("----------------------\n");
            
            for (name, rate) in exploding_patterns {
                report.push_str(&format!("{:<20}: {:.2} cells/generation\n", name, rate));
            }
        }
        
        report
    }
}