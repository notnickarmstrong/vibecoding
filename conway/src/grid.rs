use rayon::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

use crate::config::BoundaryType;

// We'll use a bit-packed grid representation for efficiency
// Each u64 stores 64 cells (1 bit per cell)
pub struct Grid {
    width: usize,
    height: usize,
    stride: usize,        // Number of u64s per row (width / 64, rounded up)
    cells: Vec<u64>,      // Bit-packed cells
    boundary: BoundaryType,
}

impl Grid {
    pub fn new(width: usize, height: usize, boundary: BoundaryType) -> Self {
        let stride = (width + 63) / 64;  // Round up to nearest 64
        let cells = vec![0; stride * height];
        
        Self {
            width,
            height,
            stride,
            cells,
            boundary,
        }
    }
    
    // Get cell state (true = alive, false = dead)
    pub fn get(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        
        let bit_index = x % 64;
        let chunk_index = (y * self.stride) + (x / 64);
        
        if chunk_index >= self.cells.len() {
            return false;
        }
        
        (self.cells[chunk_index] & (1u64 << bit_index)) != 0
    }
    
    // Set cell state
    pub fn set(&mut self, x: usize, y: usize, state: bool) {
        if x >= self.width || y >= self.height {
            return;
        }
        
        let bit_index = x % 64;
        let chunk_index = (y * self.stride) + (x / 64);
        
        if chunk_index >= self.cells.len() {
            return;
        }
        
        if state {
            self.cells[chunk_index] |= 1u64 << bit_index;
        } else {
            self.cells[chunk_index] &= !(1u64 << bit_index);
        }
    }
    
    // Toggle cell state
    pub fn toggle(&mut self, x: usize, y: usize) {
        if x >= self.width || y >= self.height {
            return;
        }
        
        let bit_index = x % 64;
        let chunk_index = (y * self.stride) + (x / 64);
        
        if chunk_index >= self.cells.len() {
            return;
        }
        
        self.cells[chunk_index] ^= 1u64 << bit_index;
    }
    
    // Count neighbors for a cell
    pub fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                
                let nx = match self.boundary {
                    BoundaryType::Wrap => (x as isize + dx).rem_euclid(self.width as isize) as usize,
                    BoundaryType::Fixed => {
                        let nx = x as isize + dx;
                        if nx < 0 || nx >= self.width as isize {
                            continue;
                        }
                        nx as usize
                    }
                };
                
                let ny = match self.boundary {
                    BoundaryType::Wrap => (y as isize + dy).rem_euclid(self.height as isize) as usize,
                    BoundaryType::Fixed => {
                        let ny = y as isize + dy;
                        if ny < 0 || ny >= self.height as isize {
                            continue;
                        }
                        ny as usize
                    }
                };
                
                if self.get(nx, ny) {
                    count += 1;
                }
            }
        }
        
        count
    }
    
    // Update the grid to the next generation
    pub fn update(&mut self) {
        let mut new_cells = vec![0; self.cells.len()];
        
        // Use Rayon for parallel processing of rows
        let height = self.height;
        let width = self.width;
        let stride = self.stride;
        
        // Process rows in parallel and collect results into individual vectors
        let results: Vec<Vec<(usize, u64)>> = (0..height).into_par_iter().map(|y| {
            let mut row_updates = Vec::new();
            for x in 0..width {
                let neighbors = self.count_neighbors(x, y);
                let is_alive = self.get(x, y);
                
                let will_be_alive = match (is_alive, neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
                
                if will_be_alive {
                    let bit_index = x % 64;
                    let chunk_index = (y * stride) + (x / 64);
                    row_updates.push((chunk_index, 1u64 << bit_index));
                }
            }
            row_updates
        }).collect();
        
        // Apply all updates to the new_cells vector
        for row_updates in results {
            for (chunk_index, bit_mask) in row_updates {
                new_cells[chunk_index] |= bit_mask;
            }
        }
        
        self.cells = new_cells;
    }
    
    // Clear all cells (set to dead)
    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = 0;
        }
    }
    
    // Randomize the grid with a given density
    pub fn randomize(&mut self, density: f64) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        for y in 0..self.height {
            for x in 0..self.width {
                let alive = rng.gen_bool(density);
                self.set(x, y, alive);
            }
        }
    }
    
    // Place a glider at a given position
    pub fn place_glider(&mut self, x: usize, y: usize) {
        if x + 2 >= self.width || y + 2 >= self.height {
            return;
        }
        
        // Clear the area
        for dy in 0..3 {
            for dx in 0..3 {
                self.set(x + dx, y + dy, false);
            }
        }
        
        // Place glider
        self.set(x + 1, y, true);
        self.set(x + 2, y + 1, true);
        self.set(x, y + 2, true);
        self.set(x + 1, y + 2, true);
        self.set(x + 2, y + 2, true);
    }
    
    // Place a random pattern at a given position
    pub fn place_random_pattern(&mut self, x: usize, y: usize) {
        if x + 3 >= self.width || y + 3 >= self.height {
            return;
        }
        
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        for dy in 0..4 {
            for dx in 0..4 {
                let alive = rng.gen_bool(0.4);
                self.set(x + dx, y + dy, alive);
            }
        }
    }
    
    // Get grid dimensions
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }
    
    // Count total number of live cells
    pub fn count_alive(&self) -> usize {
        self.cells.iter()
            .map(|&chunk| chunk.count_ones() as usize)
            .sum()
    }
    
    // Save grid state to a file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        
        // Write dimensions
        file.write_all(&(self.width as u32).to_le_bytes())?;
        file.write_all(&(self.height as u32).to_le_bytes())?;
        
        // Write cells
        for &cell in &self.cells {
            file.write_all(&cell.to_le_bytes())?;
        }
        
        Ok(())
    }
    
    // Load grid state from a file
    pub fn load_from_file<P: AsRef<Path>>(&mut self, path: P) -> std::io::Result<()> {
        let mut file = File::open(path)?;
        let mut buffer = [0u8; 4];
        
        // Read dimensions
        file.read_exact(&mut buffer)?;
        let width = u32::from_le_bytes(buffer) as usize;
        
        file.read_exact(&mut buffer)?;
        let height = u32::from_le_bytes(buffer) as usize;
        
        if width != self.width || height != self.height {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("File dimensions ({}, {}) don't match grid dimensions ({}, {})",
                    width, height, self.width, self.height)
            ));
        }
        
        // Read cells
        let mut buffer = [0u8; 8];
        for cell in &mut self.cells {
            file.read_exact(&mut buffer)?;
            *cell = u64::from_le_bytes(buffer);
        }
        
        Ok(())
    }
}