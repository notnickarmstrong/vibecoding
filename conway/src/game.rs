use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::io;

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers, KeyEvent},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::grid::Grid;
use crate::renderer::Renderer;
use crate::config::{CellTheme, ColorTheme, BoundaryType};
use crate::patterns::Pattern;

pub enum GameState {
    Running,
    Paused,
}

pub struct Game {
    grid: Grid,
    state: GameState,
    speed: usize,
    generation: usize,
    max_fps: u64,
    save_path: Option<PathBuf>,
}

impl Game {
    pub fn new(
        width: usize, 
        height: usize, 
        max_fps: u64, 
        boundary: BoundaryType,
        save_path: Option<PathBuf>,
    ) -> Self {
        let grid = Grid::new(width, height, boundary);
        
        Self {
            grid,
            state: GameState::Paused,
            speed: 5,
            generation: 0,
            max_fps,
            save_path,
        }
    }
    
    /// Initialize the grid with a predefined pattern
    pub fn initialize_with_pattern(&mut self, pattern: &Pattern, x: usize, y: usize) {
        pattern.place(&mut self.grid, x, y);
    }
    
    /// Get the dimensions of the grid
    pub fn get_grid_dimensions(&self) -> (usize, usize) {
        self.grid.dimensions()
    }
    
    pub fn run(&mut self, cell_theme: CellTheme, color_theme: ColorTheme) -> crossterm::Result<()> {
        // Setup terminal
        terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        
        // Initialize renderer
        let (width, height) = self.grid.dimensions();
        let mut renderer = Renderer::new(stdout, width, height, cell_theme, color_theme);
        renderer.init()?;
        
        // If save path was provided, try to load grid state
        if let Some(path) = &self.save_path {
            if path.exists() {
                if let Err(e) = self.grid.load_from_file(path) {
                    eprintln!("Failed to load grid state: {}", e);
                }
            }
        }
        
        let mut last_update = Instant::now();
        let frame_time = Duration::from_millis(1000 / self.max_fps);
        
        // Main game loop
        'game_loop: loop {
            // Handle input
            if event::poll(Duration::from_millis(10))? {
                if let Event::Key(key_event) = event::read()? {
                    if self.handle_input(key_event, &mut renderer)? {
                        break 'game_loop;
                    }
                }
            }
            
            // Update game state
            let now = Instant::now();
            if matches!(self.state, GameState::Running) && 
               now.duration_since(last_update).as_millis() >= (1000 / (self.speed + 1) as u128) {
                self.grid.update();
                self.generation += 1;
                last_update = now;
            }
            
            // Render
            let state_text = match self.state {
                GameState::Running => "Running",
                GameState::Paused => "Paused",
            };
            
            renderer.render(&self.grid, state_text, self.generation, self.speed)?;
            
            // Cap FPS
            let elapsed = now.elapsed();
            if elapsed < frame_time {
                std::thread::sleep(frame_time - elapsed);
            }
        }
        
        // Save grid state if path was provided
        if let Some(path) = &self.save_path {
            if let Err(e) = self.grid.save_to_file(path) {
                eprintln!("Failed to save grid state: {}", e);
            }
        }
        
        // Clean up
        renderer.cleanup()?;
        execute!(io::stdout(), LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        
        Ok(())
    }
    
    fn handle_input(&mut self, key_event: KeyEvent, renderer: &mut Renderer<io::Stdout>) -> crossterm::Result<bool> {
        match key_event.code {
            KeyCode::Char('q') => return Ok(true),
            
            // Cursor movement (Vim style)
            KeyCode::Char('h') => renderer.move_cursor(-1, 0),
            KeyCode::Char('l') => renderer.move_cursor(1, 0),
            KeyCode::Char('k') => renderer.move_cursor(0, -1),
            KeyCode::Char('j') => renderer.move_cursor(0, 1),
            
            // Cell manipulation
            KeyCode::Char(' ') => {
                let (x, y) = renderer.get_cursor_pos();
                if key_event.modifiers.contains(KeyModifiers::SHIFT) {
                    self.grid.place_glider(x, y);
                } else if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                    self.grid.place_random_pattern(x, y);
                } else {
                    self.grid.toggle(x, y);
                }
            },
            
            // Simulation control
            KeyCode::Enter => {
                self.state = match self.state {
                    GameState::Running => GameState::Paused,
                    GameState::Paused => GameState::Running,
                };
            },
            KeyCode::Char('r') => self.grid.randomize(0.3),
            KeyCode::Char('c') => {
                self.grid.clear();
                self.generation = 0;
            },
            KeyCode::Char(n) if n.is_digit(10) => {
                self.speed = n.to_digit(10).unwrap() as usize;
            },
            
            // View control
            KeyCode::Char('+') => renderer.zoom(1),
            KeyCode::Char('-') => renderer.zoom(-1),
            KeyCode::Up => renderer.pan_viewport(0, -5),
            KeyCode::Down => renderer.pan_viewport(0, 5),
            KeyCode::Left => renderer.pan_viewport(-5, 0),
            KeyCode::Right => renderer.pan_viewport(5, 0),
            KeyCode::Char('z') => renderer.reset_view(),
            
            _ => {},
        }
        
        Ok(false)
    }
}