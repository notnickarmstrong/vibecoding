# Conway's Game of Life - User Manual

## Overview

This application is a high-performance implementation of Conway's Game of Life, a cellular automaton devised by mathematician John Conway in 1970. The application runs in the terminal and provides a rich set of features including pattern libraries, benchmarking tools, and interactive tutorials.

## Components

The project consists of several components:

1. **Main Game**: The core Conway's Game of Life implementation
2. **Pattern Library**: A collection of common Game of Life patterns
3. **Benchmark Tools**: Utilities for measuring and optimizing performance
4. **Tutorial System**: An interactive guide to learning about Conway's Game of Life

## Getting Started

### Running the Game

To run the basic game with default settings:

```bash
./conway
```

Or use the release version for better performance:

```bash
./target/release/conway
```

### Game Controls

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
  - q: Quit the application

## Command-line Options

The game accepts various command-line options:

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
  -p, --initial-pattern <PATTERN>  Initial pattern to place on the grid (glider, blinker, etc.)
  -h, --help                       Display help
  -V, --version                    Print version information
```

## Utility Scripts

The project includes several utility scripts to enhance your experience:

### 1. Pattern Explorer

Explore different Game of Life patterns:

```bash
./show-patterns.sh
```

This interactive menu allows you to browse and learn about various patterns before launching them in the game.

### 2. Learning Tutorial

Learn about Conway's Game of Life through an interactive tutorial:

```bash
./learn-conway.sh
```

This guided tutorial explains the rules, pattern types, and behaviors of the Game of Life with interactive examples.

### 3. Performance Tools

Benchmark and optimize your Game of Life experience:

```bash
./conway-tools.sh
```

This utility provides performance benchmarks, optimization settings, and tools for generating interesting patterns.

## Pattern Library

The built-in pattern library includes many classic Game of Life patterns:

### Still Lifes (Stable Patterns)
- Block
- Beehive
- Loaf
- Boat

### Oscillators (Repeating Patterns)
- Blinker (period 2)
- Toad (period 2)
- Beacon (period 2)
- Pulsar (period 3)

### Spaceships (Moving Patterns)
- Glider
- Lightweight Spaceship (LWSS)

### Methuselahs (Long-lived Patterns)
- R-pentomino
- Diehard
- Acorn

### Guns (Pattern Generators)
- Gosper Glider Gun

To use any of these patterns, specify them with the `--initial-pattern` option:

```bash
./conway --initial-pattern glider_gun
```

## Advanced Features

### Saving and Loading

You can save and load grid states using the `-f` or `--file` option:

```bash
# Save the current game state
./conway --file my_pattern.bin

# Load a saved game state
./conway --file my_pattern.bin
```

### Benchmarking

Measure the performance of your Conway's Game of Life implementation:

```bash
# Run a comprehensive benchmark
./target/release/examples/benchmark

# Benchmark specific grid sizes
./target/release/examples/benchmark size 500 100

# Benchmark specific patterns
./target/release/examples/benchmark pattern 500 100
```

## Troubleshooting

If you encounter issues:

1. Make sure you have Rust and Cargo installed
2. Try rebuilding the project with `cargo build --release`
3. Check terminal size - some patterns require larger terminals
4. For performance issues, try smaller grid sizes or simpler patterns

## Further Reading

- [Conway's Game of Life on Wikipedia](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)
- [LifeWiki](https://conwaylife.com/wiki/Main_Page) - A comprehensive wiki about Conway's Game of Life
- [Golly](http://golly.sourceforge.net/) - Another Conway's Game of Life implementation with many advanced features