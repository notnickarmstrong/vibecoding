
# High-Performance Terminal Conway's Game of Life in Rust

Create an elegant, blazing-fast implementation of Conway's Game of Life that renders in the terminal with the following specifications:

## Core Requirements

- Implement Conway's Game of Life with correct cellular automaton rules
- Optimize for maximum performance using Rust's capabilities and appropriate data structures
- Create a visually stunning terminal UI with smooth animations and optional color support

## User Interface & Controls

- **Navigation**: Use Vim-style movement (h,j,k,l) to move the cursor around the grid
- **Cell Manipulation**: 
  - Space: Toggle cell state (alive/dead) at cursor position
  - Shift+Space: Place a glider at cursor position
  - Ctrl+Space: Place a random small pattern
- **Simulation Control**:
  - Enter: Pause/resume simulation
  - r: Randomize the entire grid with configurable density (default 30%)
  - c: Clear the grid
  - 0-9: Adjust simulation speed (0=slowest, 9=fastest)
- **View Control**:
  - +/-: Zoom in/out
  - Arrow keys: Pan the viewport when zoomed in
  - z: Reset zoom and center viewport

## Technical Specifications

- **Performance Targets**:
  - Support grid sizes of at least 1000x1000 cells
  - Maintain 60+ FPS on modern hardware
  - Implement efficient algorithms for neighbor counting and state updates
- **Optimizations to Consider**:
  - Use bit packing for cell states to minimize memory usage
  - Implement parallel processing with Rayon or similar
  - Apply spatial hashing or similar techniques to only update active regions
  - Utilize SIMD instructions where applicable

## Code Quality

- Apply idiomatic Rust patterns and best practices
- Implement clean architecture with separation of concerns:
  - Game logic
  - Rendering
  - User input handling
- Use appropriate error handling with Result/Option types
- Include comprehensive documentation and comments
- Write tests for core game logic and edge cases

## Advanced Features 

- Save/load functionality for grid states
- Multiple color themes for visualization
- Support for different boundary conditions (wrapped, fixed)
- Ability to adjust rules for different cellular automaton variants
- Statistics display (generation count, population, FPS)
- Configurable cell appearance (Unicode blocks, ASCII, etc.)

## Dependencies to Consider

- crossterm or termion for terminal manipulation
- rayon for parallelization
- clap for command-line argument parsing
- config for configuration management

The final application should be a showcase of Rust's capabilities for high-performance computing while maintaining code clarity and organization.







