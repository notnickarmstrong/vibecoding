#!/bin/bash
# Interactive Tutorial for Conway's Game of Life

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# Colors
BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# ASCII art for the header
print_header() {
    echo -e "${BLUE}╔════════════════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║                     ${YELLOW}${BOLD}Conway's Game of Life Tutorial${NC}${BLUE}                      ║${NC}"
    echo -e "${BLUE}╚════════════════════════════════════════════════════════════════════════════╝${NC}"
    echo
}

# Check if the executable exists and build if needed
check_executable() {
    if [[ ! -f "$SCRIPT_DIR/target/release/examples/tutorial" ]]; then
        echo -e "${YELLOW}Building Conway's Game of Life Tutorial...${NC}"
        
        # Create the tutorial example if it doesn't exist
        if [[ ! -f "$SCRIPT_DIR/examples/tutorial.rs" ]]; then
            echo -e "${YELLOW}Creating tutorial runner...${NC}"
            mkdir -p "$SCRIPT_DIR/examples"
            
            cat > "$SCRIPT_DIR/examples/tutorial.rs" << 'EOF'
// Conway's Game of Life Tutorial Runner

use std::io::{self, Write};
use std::time::Duration;
use std::thread;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor, ResetColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

use conway::tutorial::{Tutorial, Action, UserInputType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        Hide
    )?;
    
    // Create tutorial
    let mut tutorial = Tutorial::new();
    let mut running = true;
    
    while running {
        // Clear screen
        execute!(
            stdout,
            Clear(ClearType::All),
            MoveTo(0, 0)
        )?;
        
        // Get current step
        let step = tutorial.current_step();
        
        // Print step title and description
        execute!(
            stdout,
            SetForegroundColor(Color::Yellow),
            Print(format!("Step {}: {}\n\n", tutorial.current_step() + 1, step.title)),
            ResetColor,
            Print(format!("{}\n\n", step.description))
        )?;
        
        // Render grid
        render_grid(tutorial.grid(), &mut stdout)?;
        
        // Print instructions
        execute!(
            stdout,
            MoveTo(0, 25),
            SetForegroundColor(Color::Cyan),
            Print("\nInstructions: Press SPACE to advance to the next action, ENTER to skip to the next step, 'q' to quit\n"),
            ResetColor
        )?;
        
        // Wait for user input
        match wait_for_key()? {
            KeyCode::Char('q') => {
                running = false;
                break;
            },
            KeyCode::Enter => {
                if !tutorial.next_step(0) {
                    // No more steps, exit
                    running = false;
                    break;
                }
                continue;
            },
            _ => {
                // Execute next action
                if !tutorial.execute_actions(0) {
                    // No more actions, move to next step
                    if !tutorial.next_step(0) {
                        // No more steps, exit
                        running = false;
                        break;
                    }
                }
            }
        }
    }
    
    // Clean up
    execute!(
        stdout,
        Show,
        LeaveAlternateScreen
    )?;
    terminal::disable_raw_mode()?;
    
    println!("Thank you for exploring Conway's Game of Life!");
    
    Ok(())
}

// Render the grid
fn render_grid(grid: &conway::grid::Grid, stdout: &mut io::Stdout) -> crossterm::Result<()> {
    execute!(
        stdout,
        MoveTo(0, 5)
    )?;
    
    let (width, height) = grid.dimensions();
    let display_height = std::cmp::min(height, 15);
    
    for y in 0..display_height {
        for x in 0..width {
            let cell_char = if grid.get(x, y) { "█" } else { " " };
            
            if grid.get(x, y) {
                execute!(
                    stdout,
                    SetForegroundColor(Color::Green),
                    Print(cell_char),
                    ResetColor
                )?;
            } else {
                execute!(
                    stdout,
                    Print(cell_char)
                )?;
            }
        }
        execute!(stdout, Print("\n"))?;
    }
    
    Ok(())
}

// Wait for a key press
fn wait_for_key() -> crossterm::Result<KeyCode> {
    loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                return Ok(key_event.code);
            }
        }
    }
}
EOF
        }
        
        cd "$SCRIPT_DIR" && cargo build --release --example tutorial
        if [ $? -ne 0 ]; then
            echo -e "${RED}Failed to build the tutorial!${NC}"
            exit 1
        fi
        echo -e "${GREEN}Build successful!${NC}"
    fi
}

# Main function
main() {
    clear
    print_header
    
    echo -e "${GREEN}Welcome to the Conway's Game of Life Interactive Tutorial!${NC}"
    echo -e "${YELLOW}This tutorial will guide you through the basic concepts and patterns of Conway's Game of Life.${NC}"
    echo
    echo -e "${CYAN}Conway's Game of Life is a cellular automaton devised by mathematician John Conway in 1970.${NC}"
    echo -e "${CYAN}It consists of a grid of cells, each of which can be either alive or dead, and evolves according to simple rules.${NC}"
    echo
    echo -e "${GREEN}In this tutorial, you will learn about:${NC}"
    echo -e " ${YELLOW}* The basic rules of the Game of Life${NC}"
    echo -e " ${YELLOW}* Common patterns and their behaviors${NC}"
    echo -e " ${YELLOW}* How simple rules can lead to complex emergent behaviors${NC}"
    echo
    
    read -p "Press Enter to begin the tutorial..." 
    
    # Check and build tutorial if needed
    check_executable
    
    # Run the tutorial
    "$SCRIPT_DIR/target/release/examples/tutorial"
    
    # Return to the script after tutorial finishes
    echo
    echo -e "${GREEN}Tutorial completed!${NC}"
    echo -e "${YELLOW}Want to explore more? Try running the game with different patterns using:${NC}"
    echo -e " ${CYAN}./conway --initial-pattern <pattern_name>${NC}"
    echo
    echo -e "${YELLOW}Available patterns: glider, blinker, toad, beacon, pulsar, glider_gun, lwss, r-pentomino, acorn, diehard${NC}"
    echo
    echo -e "${GREEN}Happy exploring!${NC}"
}

# Run the main function
main