use crossterm::{
    cursor::{MoveTo, Hide, Show},
    execute,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor, ResetColor},
    terminal::{Clear, ClearType},
};
use std::io::Write;
use std::time::Instant;

use crate::grid::Grid;
use crate::config::{CellTheme, ColorTheme};

pub struct Renderer<W: Write> {
    output: W,
    width: usize,
    height: usize,
    cell_theme: CellTheme,
    color_theme: ColorTheme,
    zoom: usize,
    viewport_x: usize,
    viewport_y: usize,
    cursor_x: usize,
    cursor_y: usize,
    fps_counter: FpsCounter,
}

struct FpsCounter {
    frame_count: usize,
    last_update: Instant,
    current_fps: f64,
}

impl FpsCounter {
    fn new() -> Self {
        Self {
            frame_count: 0,
            last_update: Instant::now(),
            current_fps: 0.0,
        }
    }

    fn update(&mut self) {
        self.frame_count += 1;
        let elapsed = self.last_update.elapsed();

        // Update FPS every second
        if elapsed.as_secs_f64() >= 1.0 {
            self.current_fps = self.frame_count as f64 / elapsed.as_secs_f64();
            self.frame_count = 0;
            self.last_update = Instant::now();
        }
    }

    fn get_fps(&self) -> f64 {
        self.current_fps
    }
}

impl<W: Write> Renderer<W> {
    pub fn new(
        output: W,
        width: usize,
        height: usize,
        cell_theme: CellTheme,
        color_theme: ColorTheme,
    ) -> Self {
        Self {
            output,
            width,
            height,
            cell_theme,
            color_theme,
            zoom: 1,
            viewport_x: 0,
            viewport_y: 0,
            cursor_x: width / 2,
            cursor_y: height / 2,
            fps_counter: FpsCounter::new(),
        }
    }

    // Prepare terminal for rendering
    pub fn init(&mut self) -> crossterm::Result<()> {
        execute!(
            self.output,
            Hide,
            Clear(ClearType::All)
        )
    }

    // Cleanup terminal
    pub fn cleanup(&mut self) -> crossterm::Result<()> {
        execute!(
            self.output,
            ResetColor,
            Clear(ClearType::All),
            Show,
            MoveTo(0, 0)
        )
    }

    // Move cursor
    pub fn move_cursor(&mut self, dx: isize, dy: isize) {
        let new_x = self.cursor_x as isize + dx;
        let new_y = self.cursor_y as isize + dy;

        if new_x >= 0 && new_x < self.width as isize {
            self.cursor_x = new_x as usize;
        }

        if new_y >= 0 && new_y < self.height as isize {
            self.cursor_y = new_y as usize;
        }

        // Adjust viewport if cursor is outside
        self.ensure_cursor_in_viewport();
    }

    // Ensure cursor is visible in the viewport
    fn ensure_cursor_in_viewport(&mut self) {
        let visible_width = self.width / self.zoom;
        let visible_height = self.height / self.zoom;

        if self.cursor_x < self.viewport_x {
            self.viewport_x = self.cursor_x;
        } else if self.cursor_x >= self.viewport_x + visible_width {
            self.viewport_x = self.cursor_x - visible_width + 1;
        }

        if self.cursor_y < self.viewport_y {
            self.viewport_y = self.cursor_y;
        } else if self.cursor_y >= self.viewport_y + visible_height {
            self.viewport_y = self.cursor_y - visible_height + 1;
        }
    }

    // Move viewport
    pub fn pan_viewport(&mut self, dx: isize, dy: isize) {
        let visible_width = self.width / self.zoom;
        let visible_height = self.height / self.zoom;

        let new_x = self.viewport_x as isize + dx;
        let new_y = self.viewport_y as isize + dy;

        if new_x >= 0 && new_x + visible_width as isize <= self.width as isize {
            self.viewport_x = new_x as usize;
        }

        if new_y >= 0 && new_y + visible_height as isize <= self.height as isize {
            self.viewport_y = new_y as usize;
        }
    }

    // Change zoom level
    pub fn zoom(&mut self, delta: isize) {
        let _old_zoom = self.zoom;
        
        // Update zoom (min 1, max 10)
        let new_zoom = (self.zoom as isize + delta).max(1).min(10) as usize;
        if new_zoom != self.zoom {
            self.zoom = new_zoom;
            
            // Adjust viewport to keep cursor position stable
            let visible_width_new = self.width / new_zoom;
            let visible_height_new = self.height / new_zoom;
            
            // Center on cursor
            self.viewport_x = (self.cursor_x as isize - (visible_width_new / 2) as isize).max(0) as usize;
            self.viewport_y = (self.cursor_y as isize - (visible_height_new / 2) as isize).max(0) as usize;
            
            // Ensure we don't go out of bounds
            let max_viewport_x = self.width.saturating_sub(visible_width_new);
            let max_viewport_y = self.height.saturating_sub(visible_height_new);
            
            self.viewport_x = self.viewport_x.min(max_viewport_x);
            self.viewport_y = self.viewport_y.min(max_viewport_y);
        }
    }

    // Reset zoom and center viewport
    pub fn reset_view(&mut self) {
        self.zoom = 1;
        self.viewport_x = 0;
        self.viewport_y = 0;
    }

    // Get cursor position
    pub fn get_cursor_pos(&self) -> (usize, usize) {
        (self.cursor_x, self.cursor_y)
    }

    // Get cell color based on theme and position
    fn get_cell_color(&self, x: usize, y: usize) -> Color {
        match self.color_theme {
            ColorTheme::Green => Color::Green,
            ColorTheme::Blue => Color::Blue,
            ColorTheme::Rainbow => {
                // Rainbow pattern based on position
                let hue = ((x + y) % 6) as u8;
                match hue {
                    0 => Color::Red,
                    1 => Color::Yellow,
                    2 => Color::Green,
                    3 => Color::Cyan,
                    4 => Color::Blue,
                    5 => Color::Magenta,
                    _ => Color::White,
                }
            }
        }
    }

    // Render the grid
    pub fn render(&mut self, grid: &Grid, game_state: &str, generation: usize, speed: usize) -> crossterm::Result<()> {
        self.fps_counter.update();
        
        execute!(
            self.output,
            Clear(ClearType::All),
            MoveTo(0, 0)
        )?;
        
        let (grid_width, grid_height) = grid.dimensions();
        let visible_width = self.width / self.zoom;
        let visible_height = self.height / self.zoom;
        
        // Adjust viewport if necessary
        let max_viewport_x = grid_width.saturating_sub(visible_width);
        let max_viewport_y = grid_height.saturating_sub(visible_height);
        
        let viewport_x = self.viewport_x.min(max_viewport_x);
        let viewport_y = self.viewport_y.min(max_viewport_y);
        
        // Render visible cells
        for vy in 0..visible_height {
            for vx in 0..visible_width {
                let x = viewport_x + vx;
                let y = viewport_y + vy;
                
                if x >= grid_width || y >= grid_height {
                    continue;
                }
                
                let is_cursor = x == self.cursor_x && y == self.cursor_y;
                let is_alive = grid.get(x, y);
                
                let cell_char = if is_alive {
                    self.cell_theme.alive_cell()
                } else {
                    self.cell_theme.dead_cell()
                };
                
                if is_cursor {
                    execute!(
                        self.output,
                        SetBackgroundColor(Color::Grey),
                        Print(cell_char),
                        ResetColor
                    )?;
                } else if is_alive {
                    let color = self.get_cell_color(x, y);
                    execute!(
                        self.output,
                        SetForegroundColor(color),
                        Print(cell_char),
                        ResetColor
                    )?;
                } else {
                    execute!(self.output, Print(cell_char))?;
                }
            }
            execute!(self.output, Print("\n"))?;
        }
        
        // Render status bar
        let population = grid.count_alive();
        let fps = self.fps_counter.get_fps();
        
        execute!(
            self.output,
            MoveTo(0, visible_height as u16 + 1),
            Print(format!(
                "Status: {} | Gen: {} | Pop: {} | FPS: {:.1} | Speed: {} | Zoom: {}x | Cursor: ({}, {})",
                game_state, generation, population, fps, speed, self.zoom, self.cursor_x, self.cursor_y
            ))
        )?;
        
        // Render help
        execute!(
            self.output,
            MoveTo(0, visible_height as u16 + 3),
            Print("Controls: hjkl-move | Space-toggle | Shift+Space-glider | Ctrl+Space-random | Enter-pause/resume")
        )?;
        
        execute!(
            self.output,
            MoveTo(0, visible_height as u16 + 4),
            Print("          r-randomize | c-clear | 0-9-speed | +/--zoom | Arrows-pan | z-reset view | q-quit")
        )?;
        
        Ok(())
    }
}