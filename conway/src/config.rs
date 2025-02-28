use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Width of the grid
    #[arg(short = 'w', long, default_value_t = 100)]
    pub width: usize,

    /// Height of the grid
    #[arg(short = 'H', long, default_value_t = 50)]
    pub height: usize,

    /// Maximum frames per second
    #[arg(long, default_value_t = 60)]
    pub max_fps: u64,

    /// Initial density for random initialization (0.0-1.0)
    #[arg(short, long, default_value_t = 0.3)]
    pub density: f64,

    /// Cell theme to use (classic, block, dot)
    #[arg(short, long, default_value = "block")]
    pub theme: String,

    /// Color theme to use (green, blue, rainbow)
    #[arg(short = 'c', long, default_value = "green")]
    pub color_theme: String,

    /// Path to save/load grid state
    #[arg(short, long)]
    pub file: Option<PathBuf>,

    /// Boundary condition type (wrap, fixed)
    #[arg(short = 'b', long, default_value = "wrap")]
    pub boundary: String,
    
    /// Initial pattern to place on the grid (glider, blinker, toad, beacon, etc.)
    #[arg(short = 'p', long)]
    pub initial_pattern: Option<String>,
    
    /// Generate an interesting pattern based on a complexity value in a seed file
    #[arg(long)]
    pub generate_from_seed: Option<PathBuf>,
}

// Different cell appearance themes
pub enum CellTheme {
    Classic,
    Block,
    Dot,
}

impl CellTheme {
    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "classic" => CellTheme::Classic,
            "dot" => CellTheme::Dot,
            _ => CellTheme::Block,
        }
    }

    pub fn alive_cell(&self) -> &str {
        match self {
            CellTheme::Classic => "O",
            CellTheme::Block => "█",
            CellTheme::Dot => "•",
        }
    }

    pub fn dead_cell(&self) -> &str {
        match self {
            CellTheme::Classic => " ",
            CellTheme::Block => " ",
            CellTheme::Dot => " ",
        }
    }
}

// Different color themes
pub enum ColorTheme {
    Green,
    Blue,
    Rainbow,
}

impl ColorTheme {
    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "blue" => ColorTheme::Blue,
            "rainbow" => ColorTheme::Rainbow,
            _ => ColorTheme::Green,
        }
    }
}

// Boundary condition types
#[derive(Clone)]
pub enum BoundaryType {
    Wrap,
    Fixed,
}

impl BoundaryType {
    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "fixed" => BoundaryType::Fixed,
            _ => BoundaryType::Wrap,
        }
    }
}