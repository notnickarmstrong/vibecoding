# Conway's Game of Life

A high-performance terminal-based implementation of Conway's Game of Life written in Rust.

## Features

- Blazing-fast simulation supporting grid sizes up to 1000x1000
- Memory-efficient bit-packed grid representation
- Parallel processing for maximum performance
- Terminal UI with color support and smooth animations
- Multiple cell appearance themes
- Multiple color themes
- Zooming and panning
- Save/load functionality
- Support for different boundary conditions (wrap, fixed)
- Statistics display (generation count, population, FPS)
- Library of common patterns (gliders, oscillators, spaceships, and more)
- Interactive pattern explorer script
- Advanced pattern analyzer for studying pattern behavior
- Detailed classification and statistical reports
- Pattern visualization tools for creating GIFs and images
- Multiple visual themes and rendering options

## Controls

### Navigation
- `h`, `j`, `k`, `l`: Vim-style movement to move the cursor around the grid

### Cell Manipulation
- `Space`: Toggle cell state (alive/dead) at cursor position
- `Shift+Space`: Place a glider at cursor position
- `Ctrl+Space`: Place a random small pattern

### Simulation Control
- `Enter`: Pause/resume simulation
- `r`: Randomize the entire grid with configurable density (default 30%)
- `c`: Clear the grid
- `0-9`: Adjust simulation speed (0=slowest, 9=fastest)

### View Control
- `+`, `-`: Zoom in/out
- Arrow keys: Pan the viewport when zoomed in
- `z`: Reset zoom and center viewport
- `q`: Quit the application

## Usage

```bash
# Run with default settings
cargo run --release

# Run with custom grid size
cargo run --release -- --width 200 --height 100

# Run with custom settings
cargo run --release -- --width 200 --height 100 --density 0.4 --theme dot --color-theme rainbow

# Save/load grid state
cargo run --release -- --file game_state.bin

# Use fixed boundary conditions (non-wrapping)
cargo run --release -- --boundary fixed
```

## Command Line Options

```
Options:
  -w, --width <WIDTH>              Width of the grid [default: 100]
  -H, --height <HEIGHT>            Height of the grid [default: 50]
      --max-fps <MAX_FPS>          Maximum frames per second [default: 60]
  -d, --density <DENSITY>          Initial density for random initialization (0.0-1.0) [default: 0.3]
  -t, --theme <THEME>              Cell theme to use (classic, block, dot) [default: block]
  -c, --color-theme <COLOR_THEME>  Color theme to use (green, blue, rainbow) [default: green]
  -f, --file <FILE>                Path to save/load grid state
  -b, --boundary <BOUNDARY>        Boundary condition type (wrap, fixed) [default: wrap]
  -p, --initial-pattern <PATTERN>  Initial pattern to place on the grid (glider, blinker, pulsar, etc.)
  -V, --version                    Print version information
  -h, --help                       Display help
```

## Patterns

The game includes a library of common Conway's Game of Life patterns that can be placed on the grid. For a detailed explanation of each pattern, see the [PATTERNS.md](PATTERNS.md) file.

To explore patterns interactively, use the included scripts:

```bash
# Pattern explorer - Try different patterns
./show-patterns.sh

# Pattern analyzer - Study pattern behavior and classification
./analyze-patterns.sh

# Pattern visualizer - Create GIFs and images of patterns
./visualize-patterns.sh

# Performance tools - Benchmark and optimize
./conway-tools.sh

# Interactive tutorial - Learn about Conway's Game of Life
./learn-conway.sh
```

These provide menu-based interfaces to explore different aspects of Conway's Game of Life.

## Implementation Details

This implementation uses several optimization techniques:

1. **Bit-packed grid representation**: Each cell state requires only 1 bit, allowing efficient storage and manipulation.
2. **Parallel processing**: Uses Rayon for parallel grid updates, taking advantage of all available CPU cores.
3. **Efficient algorithms**: Uses optimized algorithms for neighbor counting and state updates.
4. **Smart rendering**: Only renders the visible portion of the grid, supporting large grid sizes with minimal performance impact.

## Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/conway.git
cd conway

# Build in release mode
cargo build --release

# Run the executable
./target/release/conway
```

## License

MIT