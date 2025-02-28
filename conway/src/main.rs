mod config;
mod grid;
mod renderer;
mod game;
mod patterns;

use std::path::Path;
use std::fs::File;
use std::io::{self, Read};
use clap::Parser;
use config::{Config, CellTheme, ColorTheme, BoundaryType};
use game::Game;
use patterns::PatternLibrary;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let config = Config::parse();
    
    // Create game instance
    let mut game = Game::new(
        config.width,
        config.height,
        config.max_fps,
        BoundaryType::from_string(&config.boundary),
        config.file.clone(),
    );
    
    // Apply initial pattern if specified
    if let Some(pattern_name) = &config.initial_pattern {
        if let Some(pattern) = PatternLibrary::get_by_name(pattern_name) {
            let x = config.width / 2 - pattern.width / 2;
            let y = config.height / 2 - pattern.height / 2;
            game.initialize_with_pattern(&pattern, x, y);
        }
    }
    
    // If generate-from-seed is specified, create a custom pattern
    if let Some(seed_path) = &config.generate_from_seed {
        if let Ok(complexity) = read_complexity_from_file(seed_path) {
            generate_custom_pattern(&mut game, complexity);
        }
    }
    
    // Start the game
    game.run(
        CellTheme::from_string(&config.theme),
        ColorTheme::from_string(&config.color_theme),
    )?;
    
    Ok(())
}

fn read_complexity_from_file(path: &Path) -> io::Result<usize> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    contents.trim().parse::<usize>().map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidData, "Invalid complexity value in seed file")
    })
}

fn generate_custom_pattern(game: &mut Game, complexity: usize) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    // Get all available patterns
    let patterns = PatternLibrary::get_all_patterns();
    
    // Calculate the grid dimensions
    let (width, height) = game.get_grid_dimensions();
    
    // Place random patterns at random locations
    for _ in 0..complexity {
        let pattern_idx = rng.gen_range(0..patterns.len());
        let pattern = &patterns[pattern_idx];
        
        let max_x = width.saturating_sub(pattern.width);
        let max_y = height.saturating_sub(pattern.height);
        
        let x = if max_x > 0 { rng.gen_range(0..max_x) } else { 0 };
        let y = if max_y > 0 { rng.gen_range(0..max_y) } else { 0 };
        
        game.initialize_with_pattern(pattern, x, y);
    }
}