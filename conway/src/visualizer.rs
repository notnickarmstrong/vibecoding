// Conway's Game of Life Visualizer
// Creates visual representations of patterns for documentation and sharing

use std::fs::File;
use std::path::Path;
use std::io::BufWriter;

use image::{Rgba, RgbaImage};
use gif::Encoder;

// Custom gradient implementation since we're having issues with the palette crate
struct CustomGradient {
    colors: Vec<[f32; 4]>, // RGBA colors
}

// Simple linear interpolation of colors
impl CustomGradient {
    fn new(colors: Vec<[f32; 4]>) -> Self {
        Self { colors }
    }
    
    fn get(&self, t: f32) -> [f32; 4] {
        if t <= 0.0 || self.colors.len() == 1 {
            return self.colors[0];
        }
        if t >= 1.0 {
            return self.colors[self.colors.len() - 1];
        }
        
        let segments = self.colors.len() - 1;
        let segment_width = 1.0 / segments as f32;
        let segment_index = (t / segment_width).floor() as usize;
        
        if segment_index >= segments {
            return self.colors[segments];
        }
        
        let segment_t = (t - segment_index as f32 * segment_width) / segment_width;
        let c1 = self.colors[segment_index];
        let c2 = self.colors[segment_index + 1];
        
        [
            c1[0] * (1.0 - segment_t) + c2[0] * segment_t,
            c1[1] * (1.0 - segment_t) + c2[1] * segment_t,
            c1[2] * (1.0 - segment_t) + c2[2] * segment_t,
            c1[3] * (1.0 - segment_t) + c2[3] * segment_t,
        ]
    }
}

use crate::grid::Grid;
use crate::patterns::Pattern;
use crate::config::BoundaryType;

// Color themes for different visualization styles
pub enum VisualTheme {
    // Classic black and white
    Classic,
    // Green on black (terminal style)
    Matrix,
    // Blue gradient based on cell age
    Ocean,
    // Fire colors based on cell age
    Inferno,
    // Rainbow colors
    Rainbow,
    // Custom gradient from start to end color
    Custom([f32; 4], [f32; 4]),
}

impl VisualTheme {
    // Get the gradient for this theme
    fn get_gradient(&self) -> CustomGradient {
        match self {
            VisualTheme::Classic => {
                CustomGradient::new(vec![
                    [0.0, 0.0, 0.0, 1.0],
                    [1.0, 1.0, 1.0, 1.0],
                ])
            },
            VisualTheme::Matrix => {
                CustomGradient::new(vec![
                    [0.0, 0.0, 0.0, 1.0],
                    [0.0, 0.8, 0.2, 1.0],
                ])
            },
            VisualTheme::Ocean => {
                CustomGradient::new(vec![
                    [0.0, 0.0, 0.1, 1.0],
                    [0.0, 0.3, 0.6, 1.0],
                    [0.0, 0.5, 0.9, 1.0],
                ])
            },
            VisualTheme::Inferno => {
                CustomGradient::new(vec![
                    [0.0, 0.0, 0.0, 1.0],
                    [0.5, 0.0, 0.0, 1.0],
                    [0.8, 0.3, 0.0, 1.0],
                    [1.0, 0.8, 0.0, 1.0],
                    [1.0, 1.0, 0.3, 1.0],
                ])
            },
            VisualTheme::Rainbow => {
                CustomGradient::new(vec![
                    [0.8, 0.0, 0.0, 1.0], // Red
                    [0.8, 0.4, 0.0, 1.0], // Orange
                    [0.8, 0.8, 0.0, 1.0], // Yellow
                    [0.0, 0.8, 0.0, 1.0], // Green
                    [0.0, 0.0, 0.8, 1.0], // Blue
                    [0.5, 0.0, 0.8, 1.0], // Purple
                ])
            },
            VisualTheme::Custom(start, end) => {
                CustomGradient::new(vec![*start, *end])
            },
        }
    }
}

// Settings for the visualization
pub struct VisualizerSettings {
    // Size of each cell in pixels
    pub cell_size: u32,
    // Padding between cells
    pub cell_padding: u32,
    // Background color (RGBA)
    pub background_color: [u8; 4],
    // Color theme for cells
    pub theme: VisualTheme,
    // Frame delay in milliseconds
    pub frame_delay: u16,
    // Number of generations to simulate
    pub generations: usize,
    // Set to true to loop the GIF
    pub loop_animation: bool,
    // Grid lines (optional)
    pub show_grid_lines: bool,
    // Grid line color (RGBA)
    pub grid_line_color: [u8; 4],
    // Border around the entire grid
    pub border_size: u32,
    // Border color (RGBA)
    pub border_color: [u8; 4],
}

impl Default for VisualizerSettings {
    fn default() -> Self {
        Self {
            cell_size: 10,
            cell_padding: 1,
            background_color: [0, 0, 0, 255],
            theme: VisualTheme::Matrix,
            frame_delay: 100,
            generations: 100,
            loop_animation: true,
            show_grid_lines: false,
            grid_line_color: [50, 50, 50, 255],
            border_size: 1,
            border_color: [100, 100, 100, 255],
        }
    }
}

// The visualizer itself
pub struct Visualizer {
    settings: VisualizerSettings,
    // Keep track of how long cells have been alive
    cell_age: Vec<Vec<usize>>,
}

impl Visualizer {
    // Create a new visualizer with the given settings
    pub fn new(settings: VisualizerSettings) -> Self {
        Self {
            settings,
            cell_age: Vec::new(),
        }
    }
    
    // Create a GIF of a pattern's evolution
    pub fn create_pattern_gif<P: AsRef<Path>>(
        &mut self,
        pattern: &Pattern,
        output_path: P,
        grid_size: (usize, usize),
        boundary: BoundaryType,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Create grid and place pattern in center
        let mut grid = Grid::new(grid_size.0, grid_size.1, boundary);
        let x = grid_size.0 / 2 - pattern.width / 2;
        let y = grid_size.1 / 2 - pattern.height / 2;
        pattern.place(&mut grid, x, y);
        
        // Initialize cell age tracking
        self.cell_age = vec![vec![0; grid_size.1]; grid_size.0];
        
        // Create output file
        let file = File::create(output_path)?;
        let writer = BufWriter::new(file);
        
        // Calculate image dimensions
        let img_width = grid_size.0 as u32 * (self.settings.cell_size + self.settings.cell_padding) 
                          + self.settings.border_size * 2;
        let img_height = grid_size.1 as u32 * (self.settings.cell_size + self.settings.cell_padding)
                          + self.settings.border_size * 2;
        
        // Set up GIF encoder
        let mut encoder = Encoder::new(
            writer,
            img_width as u16,
            img_height as u16,
            &[]
        )?;
        
        // Set GIF parameters
        if self.settings.loop_animation {
            // Setting repeat mode
            encoder.set_repeat(gif::Repeat::Infinite)?;
        }
        
        // Color gradient for the theme
        let gradient = self.settings.theme.get_gradient();
        
        // Generate frames
        for _ in 0..self.settings.generations {
            // Create frame
            let mut frame = RgbaImage::new(img_width, img_height);
            
            // Fill background
            for pixel in frame.pixels_mut() {
                *pixel = Rgba(self.settings.background_color);
            }
            
            // Draw border if configured
            if self.settings.border_size > 0 {
                self.draw_border(&mut frame, img_width, img_height);
            }
            
            // Draw grid lines if configured
            if self.settings.show_grid_lines {
                self.draw_grid_lines(&mut frame, grid_size);
            }
            
            // Draw cells
            for y in 0..grid_size.1 {
                for x in 0..grid_size.0 {
                    if grid.get(x, y) {
                        // Increment age for living cells
                        self.cell_age[x][y] += 1;
                        
                        // Calculate color based on cell age
                        let rel_age = (self.cell_age[x][y] as f32).min(100.0) / 100.0;
                        let color = gradient.get(rel_age);
                        
                        // Convert to RGBA
                        let rgba = [
                            (color[0] * 255.0) as u8,
                            (color[1] * 255.0) as u8,
                            (color[2] * 255.0) as u8,
                            255,
                        ];
                        
                        // Draw the cell
                        self.draw_cell(&mut frame, x, y, rgba);
                    } else {
                        // Reset age for dead cells
                        self.cell_age[x][y] = 0;
                    }
                }
            }
            
            // Add the frame to the GIF
            // Create a gif frame
            let buffer = frame.into_raw();
            
            // Create a new frame
            let mut frame_data = vec![0; (img_width * img_height * 4) as usize];
            frame_data.copy_from_slice(&buffer);
            
            // Create a gif frame from RGBA data
            let mut gif_frame = gif::Frame::from_rgba(
                img_width as u16, 
                img_height as u16, 
                &mut frame_data
            );
            
            // Set delay in centiseconds
            gif_frame.delay = self.settings.frame_delay / 10; // Convert to centiseconds
            encoder.write_frame(&gif_frame)?;
            
            // Update the grid for the next frame
            grid.update();
        }
        
        Ok(())
    }
    
    // Draw a single cell on the image
    fn draw_cell(&self, frame: &mut RgbaImage, x: usize, y: usize, color: [u8; 4]) {
        let cell_size = self.settings.cell_size;
        let padding = self.settings.cell_padding;
        let border = self.settings.border_size;
        
        let start_x = border + (x as u32) * (cell_size + padding);
        let start_y = border + (y as u32) * (cell_size + padding);
        
        // Fill the cell with the given color
        for cy in 0..cell_size {
            for cx in 0..cell_size {
                let px = start_x + cx;
                let py = start_y + cy;
                if px < frame.width() && py < frame.height() {
                    frame.put_pixel(px, py, Rgba(color));
                }
            }
        }
    }
    
    // Draw grid lines between cells
    fn draw_grid_lines(&self, frame: &mut RgbaImage, grid_size: (usize, usize)) {
        let color = self.settings.grid_line_color;
        let cell_size = self.settings.cell_size;
        let padding = self.settings.cell_padding;
        let border = self.settings.border_size;
        
        // Draw horizontal grid lines
        for y in 0..=grid_size.1 {
            let y_pos = border + y as u32 * (cell_size + padding);
            
            if padding == 0 && y < grid_size.1 {
                continue; // Skip if we have no padding and not at the edge
            }
            
            for x in 0..frame.width() {
                if y_pos < frame.height() {
                    frame.put_pixel(x, y_pos, Rgba(color));
                }
            }
        }
        
        // Draw vertical grid lines
        for x in 0..=grid_size.0 {
            let x_pos = border + x as u32 * (cell_size + padding);
            
            if padding == 0 && x < grid_size.0 {
                continue; // Skip if we have no padding and not at the edge
            }
            
            for y in 0..frame.height() {
                if x_pos < frame.width() {
                    frame.put_pixel(x_pos, y, Rgba(color));
                }
            }
        }
    }
    
    // Draw a border around the entire grid
    fn draw_border(&self, frame: &mut RgbaImage, width: u32, height: u32) {
        let border_size = self.settings.border_size;
        let color = self.settings.border_color;
        
        if border_size == 0 {
            return;
        }
        
        // Draw top and bottom borders
        for y in 0..border_size {
            for x in 0..width {
                frame.put_pixel(x, y, Rgba(color));
                frame.put_pixel(x, height - 1 - y, Rgba(color));
            }
        }
        
        // Draw left and right borders
        for x in 0..border_size {
            for y in 0..height {
                frame.put_pixel(x, y, Rgba(color));
                frame.put_pixel(width - 1 - x, y, Rgba(color));
            }
        }
    }
    
    // Create a sequence of images for each generation
    pub fn create_pattern_images<P: AsRef<Path>>(
        &mut self,
        pattern: &Pattern,
        output_dir: P,
        grid_size: (usize, usize),
        boundary: BoundaryType,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Create grid and place pattern in center
        let mut grid = Grid::new(grid_size.0, grid_size.1, boundary);
        let x = grid_size.0 / 2 - pattern.width / 2;
        let y = grid_size.1 / 2 - pattern.height / 2;
        pattern.place(&mut grid, x, y);
        
        // Initialize cell age tracking
        self.cell_age = vec![vec![0; grid_size.1]; grid_size.0];
        
        // Create output directory if it doesn't exist
        let output_dir = output_dir.as_ref();
        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir)?;
        }
        
        // Calculate image dimensions
        let img_width = grid_size.0 as u32 * (self.settings.cell_size + self.settings.cell_padding) 
                          + self.settings.border_size * 2;
        let img_height = grid_size.1 as u32 * (self.settings.cell_size + self.settings.cell_padding)
                          + self.settings.border_size * 2;
        
        // Color gradient for the theme
        let gradient = self.settings.theme.get_gradient();
        
        // Generate frames
        for generation in 0..self.settings.generations {
            // Create frame
            let mut frame = RgbaImage::new(img_width, img_height);
            
            // Fill background
            for pixel in frame.pixels_mut() {
                *pixel = Rgba(self.settings.background_color);
            }
            
            // Draw border if configured
            if self.settings.border_size > 0 {
                self.draw_border(&mut frame, img_width, img_height);
            }
            
            // Draw grid lines if configured
            if self.settings.show_grid_lines {
                self.draw_grid_lines(&mut frame, grid_size);
            }
            
            // Draw cells
            for y in 0..grid_size.1 {
                for x in 0..grid_size.0 {
                    if grid.get(x, y) {
                        // Increment age for living cells
                        self.cell_age[x][y] += 1;
                        
                        // Calculate color based on cell age
                        let rel_age = (self.cell_age[x][y] as f32).min(100.0) / 100.0;
                        let color = gradient.get(rel_age);
                        
                        // Convert to RGBA
                        let rgba = [
                            (color[0] * 255.0) as u8,
                            (color[1] * 255.0) as u8,
                            (color[2] * 255.0) as u8,
                            255,
                        ];
                        
                        // Draw the cell
                        self.draw_cell(&mut frame, x, y, rgba);
                    } else {
                        // Reset age for dead cells
                        self.cell_age[x][y] = 0;
                    }
                }
            }
            
            // Save the frame as an image
            let file_name = format!("{}_gen_{:04}.png", pattern.name.to_lowercase(), generation);
            let file_path = output_dir.join(file_name);
            frame.save(file_path)?;
            
            // Update the grid for the next frame
            grid.update();
        }
        
        Ok(())
    }
    
    // Create a composite image of pattern evolution
    pub fn create_pattern_evolution_image<P: AsRef<Path>>(
        &mut self,
        pattern: &Pattern,
        output_path: P,
        grid_size: (usize, usize),
        boundary: BoundaryType,
        generations: usize,
        columns: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Create grid and place pattern in center
        let mut grid = Grid::new(grid_size.0, grid_size.1, boundary);
        let x = grid_size.0 / 2 - pattern.width / 2;
        let y = grid_size.1 / 2 - pattern.height / 2;
        pattern.place(&mut grid, x, y);
        
        // Initialize cell age tracking
        self.cell_age = vec![vec![0; grid_size.1]; grid_size.0];
        
        // Calculate frame dimensions
        let frame_width = grid_size.0 as u32 * (self.settings.cell_size + self.settings.cell_padding) 
                        + self.settings.border_size * 2;
        let frame_height = grid_size.1 as u32 * (self.settings.cell_size + self.settings.cell_padding)
                        + self.settings.border_size * 2;
        
        // Calculate composite image dimensions
        let rows = (generations + columns - 1) / columns;
        let img_width = frame_width * columns as u32;
        let img_height = frame_height * rows as u32;
        
        // Create composite image
        let mut composite = RgbaImage::new(img_width, img_height);
        
        // Fill background
        for pixel in composite.pixels_mut() {
            *pixel = Rgba(self.settings.background_color);
        }
        
        // Color gradient for the theme
        let gradient = self.settings.theme.get_gradient();
        
        // Generate frames and add them to composite
        for generation in 0..generations {
            let col = generation % columns;
            let row = generation / columns;
            
            // Create frame
            let mut frame = RgbaImage::new(frame_width, frame_height);
            
            // Fill background
            for pixel in frame.pixels_mut() {
                *pixel = Rgba(self.settings.background_color);
            }
            
            // Draw border if configured
            if self.settings.border_size > 0 {
                self.draw_border(&mut frame, frame_width, frame_height);
            }
            
            // Draw grid lines if configured
            if self.settings.show_grid_lines {
                self.draw_grid_lines(&mut frame, grid_size);
            }
            
            // Draw cells
            for y in 0..grid_size.1 {
                for x in 0..grid_size.0 {
                    if grid.get(x, y) {
                        // Increment age for living cells
                        self.cell_age[x][y] += 1;
                        
                        // Calculate color based on cell age
                        let rel_age = (self.cell_age[x][y] as f32).min(100.0) / 100.0;
                        let color = gradient.get(rel_age);
                        
                        // Convert to RGBA
                        let rgba = [
                            (color[0] * 255.0) as u8,
                            (color[1] * 255.0) as u8,
                            (color[2] * 255.0) as u8,
                            255,
                        ];
                        
                        // Draw the cell
                        self.draw_cell(&mut frame, x, y, rgba);
                    } else {
                        // Reset age for dead cells
                        self.cell_age[x][y] = 0;
                    }
                }
            }
            
            // Add frame to composite
            let start_x = col as u32 * frame_width;
            let start_y = row as u32 * frame_height;
            
            for (x, y, pixel) in frame.enumerate_pixels() {
                let comp_x = start_x + x;
                let comp_y = start_y + y;
                if comp_x < img_width && comp_y < img_height {
                    composite.put_pixel(comp_x, comp_y, *pixel);
                }
            }
            
            // Update the grid for the next frame
            grid.update();
        }
        
        // Save the composite image
        composite.save(output_path)?;
        
        Ok(())
    }
}