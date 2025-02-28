// Conway's Game of Life Benchmark Tool
// A utility for testing the performance of Conway's Game of Life

use std::time::{Duration, Instant};
use rand::Rng;

use crate::grid::Grid;
use crate::config::BoundaryType;
use crate::patterns::PatternLibrary;

pub struct BenchmarkResult {
    pub grid_size: (usize, usize),
    pub generations: usize,
    pub boundary_type: &'static str,
    pub elapsed_time: Duration,
    pub cell_updates_per_second: f64,
}

impl BenchmarkResult {
    pub fn to_string(&self) -> String {
        format!(
            "Grid Size: {}x{}, Boundary: {}, Generations: {}, Time: {:.2?}, Cell Updates/s: {:.2} billion",
            self.grid_size.0,
            self.grid_size.1,
            self.boundary_type,
            self.generations,
            self.elapsed_time,
            self.cell_updates_per_second / 1_000_000_000.0
        )
    }
}

/// Run a benchmark for a given grid size, generations, and boundary type
pub fn run_benchmark(
    width: usize,
    height: usize,
    generations: usize,
    boundary: BoundaryType,
    pattern_name: Option<&str>,
    density: f64,
) -> BenchmarkResult {
    // Create grid
    let mut grid = Grid::new(width, height, boundary.clone());
    
    // Initialize grid with pattern or random cells
    match pattern_name {
        Some(name) => {
            if let Some(pattern) = PatternLibrary::get_by_name(name) {
                // Place pattern in the center
                let x = width / 2 - pattern.width / 2;
                let y = height / 2 - pattern.height / 2;
                pattern.place(&mut grid, x, y);
            } else {
                // Invalid pattern, use random
                grid.randomize(density);
            }
        },
        None => {
            // Random initialization
            grid.randomize(density);
        }
    }
    
    // Measure performance
    let start = Instant::now();
    
    for _ in 0..generations {
        grid.update();
    }
    
    let elapsed = start.elapsed();
    
    // Calculate cell updates per second
    let total_cells = width * height * generations;
    let cell_updates_per_second = total_cells as f64 / elapsed.as_secs_f64();
    
    let boundary_str = match boundary {
        BoundaryType::Wrap => "Wrapped",
        BoundaryType::Fixed => "Fixed",
    };
    
    BenchmarkResult {
        grid_size: (width, height),
        generations,
        boundary_type: boundary_str,
        elapsed_time: elapsed,
        cell_updates_per_second,
    }
}

/// Run benchmarks for various grid sizes
pub fn run_size_benchmarks(max_size: usize, generations: usize) -> Vec<BenchmarkResult> {
    let sizes = [
        (100, 100),
        (250, 250),
        (500, 500),
        (1000, 1000),
        (max_size, max_size),
    ];
    
    let mut results = Vec::new();
    
    for (width, height) in sizes.iter() {
        if *width <= max_size && *height <= max_size {
            let result = run_benchmark(
                *width,
                *height,
                generations,
                BoundaryType::Wrap,
                None,
                0.3,
            );
            results.push(result);
        }
    }
    
    results
}

/// Run benchmarks for various patterns
pub fn run_pattern_benchmarks(width: usize, height: usize, generations: usize) -> Vec<BenchmarkResult> {
    let patterns = [
        "glider",
        "blinker",
        "pulsar",
        "glider_gun",
        "lwss",
        "r-pentomino",
        "acorn",
    ];
    
    let mut results = Vec::new();
    
    for pattern in patterns.iter() {
        let result = run_benchmark(
            width,
            height,
            generations,
            BoundaryType::Wrap,
            Some(pattern),
            0.3,
        );
        results.push(result);
    }
    
    // Also run a random benchmark for comparison
    let random_result = run_benchmark(
        width,
        height,
        generations,
        BoundaryType::Wrap,
        None,
        0.3,
    );
    results.push(random_result);
    
    results
}

/// Generate a random interesting pattern
pub fn generate_random_pattern(width: usize, height: usize, complexity: usize) -> Grid {
    let mut grid = Grid::new(width, height, BoundaryType::Wrap);
    let mut rng = rand::thread_rng();
    
    // Start with a seed pattern
    let patterns = PatternLibrary::get_all_patterns();
    
    // Place random patterns at random locations
    for _ in 0..complexity {
        let pattern_idx = rng.gen_range(0..patterns.len());
        let pattern = &patterns[pattern_idx];
        
        let x = rng.gen_range(0..width.saturating_sub(pattern.width));
        let y = rng.gen_range(0..height.saturating_sub(pattern.height));
        
        pattern.place(&mut grid, x, y);
    }
    
    // Run a few generations to create interesting dynamics
    for _ in 0..10 {
        grid.update();
    }
    
    grid
}

// Run a few generations with a timer
pub fn preview_pattern(grid: &mut Grid, generations: usize) -> Duration {
    let start = Instant::now();
    
    for _ in 0..generations {
        grid.update();
    }
    
    start.elapsed()
}

// Simple benchmark functions can be added here if needed