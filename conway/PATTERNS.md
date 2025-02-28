# Conway's Game of Life Pattern Library

This project includes a comprehensive library of well-known patterns for Conway's Game of Life. You can use these patterns directly from the command line with the `--initial-pattern` flag or explore them interactively with the `show-patterns.sh` script.

## Available Patterns

### Oscillators
Patterns that repeat after a certain number of generations.

1. **Blinker** 
   - The simplest oscillator
   - Period: 2 generations

2. **Toad**
   - A period 2 oscillator
   - Resembles a toad

3. **Beacon**
   - A period 2 oscillator 
   - Two blocks that flash

4. **Pulsar**
   - A large, symmetric period 3 oscillator
   - One of the most complex common oscillators

### Spaceships
Patterns that move across the grid.

5. **Glider**
   - The smallest and most common spaceship
   - Moves diagonally across the grid

6. **LWSS (Lightweight Spaceship)**
   - Moves horizontally across the grid
   - Larger than a glider

### Methuselahs
Patterns that evolve for many generations before stabilizing.

7. **R-pentomino**
   - A small pattern that evolves for 1103 generations
   - Produces gliders and other structures

8. **Diehard**
   - A methuselah that vanishes after 130 generations
   - Leaves no permanent structures

9. **Acorn**
   - A 7-cell pattern that evolves for 5206 generations
   - Produces multiple gliders

### Guns
Patterns that periodically emit spaceships.

10. **Glider Gun**
    - Gosper's Glider Gun - the first known gun
    - Emits a glider every 30 generations
    - Used to prove that Conway's Game of Life can sustain unbounded growth

## Using the Pattern Library

### From the Command Line

```bash
# Run Conway's Game of Life with a glider in the center
./conway --initial-pattern glider

# Run with a specific pattern on a larger grid
./conway --width 200 --height 100 --initial-pattern pulsar

# Run with a glider gun
./conway --width 200 --height 100 --initial-pattern glider_gun
```

### With the Interactive Explorer

For an interactive experience, use the provided pattern explorer script:

```bash
./show-patterns.sh
```

This script allows you to:
- Browse all available patterns
- Read descriptions of each pattern
- Launch the game with your selected pattern
- Experiment with different patterns easily

## Adding Your Own Patterns

The pattern library is easy to extend. To add a new pattern:

1. Edit the `patterns.rs` file
2. Add a new function that returns a `Pattern` structure
3. Add your pattern to the `get_all_patterns()` function

Example:

```rust
pub fn my_custom_pattern() -> Pattern {
    Pattern {
        name: "My Pattern",
        description: "Description of my custom pattern",
        width: 3,
        height: 3,
        cells: vec![(0, 0), (1, 1), (2, 2)],  // Coordinates of live cells
    }
}
```