#!/bin/bash
# Conway's Game of Life Pattern Visualizer
# Creates visual representations of patterns for documentation and sharing

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

print_header() {
    echo -e "${BLUE}╔═════════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║                ${YELLOW}${BOLD}Conway's Game of Life Visualizer${NC}${BLUE}                  ║${NC}"
    echo -e "${BLUE}╚═════════════════════════════════════════════════════════════════════╝${NC}"
    echo
}

check_executable() {
    if [[ ! -f "$SCRIPT_DIR/target/release/examples/visualizer" ]]; then
        echo -e "${YELLOW}Building Conway's Game of Life Visualizer...${NC}"
        
        # Create the visualizer example if it doesn't exist
        if [[ ! -f "$SCRIPT_DIR/examples/visualizer.rs" ]]; then
            echo -e "${YELLOW}Creating visualizer runner...${NC}"
            mkdir -p "$SCRIPT_DIR/examples"
            
            cat > "$SCRIPT_DIR/examples/visualizer.rs" << 'EOF'
// Conway's Game of Life Visualizer Example

use std::env;
use std::path::Path;
use conway::visualizer::{Visualizer, VisualizerSettings, VisualTheme};
use conway::patterns::PatternLibrary;
use conway::config::BoundaryType;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Display usage if no arguments
    if args.len() < 3 {
        println!("Conway's Game of Life Visualizer");
        println!("===============================");
        println!();
        println!("Usage: {} <mode> <pattern_name> <output_path> [options...]", args[0]);
        println!();
        println!("Modes:");
        println!("  gif       - Create animated GIF");
        println!("  images    - Create sequence of images");
        println!("  evolution - Create evolution composite image");
        println!();
        println!("Examples:");
        println!("  {} gif glider glider.gif", args[0]);
        println!("  {} images blinker ./blinker_frames/", args[0]);
        println!("  {} evolution r-pentomino r-pentomino_evolution.png 20 4", args[0]);
        println!();
        
        println!("Available patterns:");
        for pattern in PatternLibrary::get_all_patterns() {
            println!("  - {}", pattern.name);
        }
        
        println!();
        println!("Available themes:");
        println!("  - classic (Black and white)");
        println!("  - matrix (Green on black)");
        println!("  - ocean (Blue gradient)");
        println!("  - inferno (Fire colors)");
        println!("  - rainbow (Multiple colors)");
        
        return Ok(());
    }
    
    // Parse arguments
    let mode = &args[1];
    let pattern_name = &args[2];
    let output_path = args.get(3).ok_or("Output path must be specified")?;
    
    // Get the pattern
    let pattern = PatternLibrary::get_by_name(pattern_name)
        .ok_or_else(|| format!("Pattern '{}' not found", pattern_name))?;
    
    // Default settings
    let mut settings = VisualizerSettings::default();
    
    // Parse optional settings if provided
    if args.len() > 4 {
        let theme_name = &args[4];
        settings.theme = match theme_name.to_lowercase().as_str() {
            "classic" => VisualTheme::Classic,
            "matrix" => VisualTheme::Matrix,
            "ocean" => VisualTheme::Ocean,
            "inferno" => VisualTheme::Inferno,
            "rainbow" => VisualTheme::Rainbow,
            _ => VisualTheme::Matrix,
        };
    }
    
    if args.len() > 5 {
        if let Ok(generations) = args[5].parse() {
            settings.generations = generations;
        }
    }
    
    if args.len() > 6 {
        if let Ok(cell_size) = args[6].parse() {
            settings.cell_size = cell_size;
        }
    }
    
    if args.len() > 7 {
        if let Ok(delay) = args[7].parse() {
            settings.frame_delay = delay;
        }
    }
    
    // Create visualizer
    let mut visualizer = Visualizer::new(settings);
    
    // Determine grid size based on pattern
    let grid_size = (
        pattern.width * 4,
        pattern.height * 4,
    );
    
    // Process based on mode
    match mode.to_lowercase().as_str() {
        "gif" => {
            println!("Creating GIF for pattern '{}' at '{}'", pattern_name, output_path);
            visualizer.create_pattern_gif(
                pattern,
                output_path,
                grid_size,
                BoundaryType::Wrap,
            )?;
            println!("GIF created successfully!");
        },
        "images" => {
            println!("Creating image sequence for pattern '{}' at '{}'", pattern_name, output_path);
            visualizer.create_pattern_images(
                pattern,
                output_path,
                grid_size,
                BoundaryType::Wrap,
            )?;
            println!("Image sequence created successfully!");
        },
        "evolution" => {
            println!("Creating evolution image for pattern '{}' at '{}'", pattern_name, output_path);
            
            // Parse additional parameters for evolution image
            let generations = if args.len() > 4 {
                args[4].parse().unwrap_or(20)
            } else {
                20
            };
            
            let columns = if args.len() > 5 {
                args[5].parse().unwrap_or(4)
            } else {
                4
            };
            
            visualizer.create_pattern_evolution_image(
                pattern,
                output_path,
                grid_size,
                BoundaryType::Wrap,
                generations,
                columns,
            )?;
            println!("Evolution image created successfully!");
        },
        _ => {
            return Err(format!("Unknown mode: {}", mode).into());
        }
    }
    
    Ok(())
}
EOF
        }
        
        cd "$SCRIPT_DIR" && cargo build --release --example visualizer
        if [ $? -ne 0 ]; then
            echo -e "${RED}Failed to build the visualizer!${NC}"
            exit 1
        fi
        echo -e "${GREEN}Build successful!${NC}"
    fi
}

visualize_gif() {
    local pattern=$1
    local output_file=$2
    local theme=${3:-matrix}
    local generations=${4:-100}
    local cell_size=${5:-10}
    local delay=${6:-100}
    
    echo -e "${CYAN}Creating GIF for pattern: ${YELLOW}$pattern${NC}"
    echo -e "${CYAN}Output file: ${YELLOW}$output_file${NC}"
    echo -e "${CYAN}Theme: ${YELLOW}$theme${NC}"
    echo -e "${CYAN}Generations: ${YELLOW}$generations${NC}"
    echo -e "${CYAN}Cell size: ${YELLOW}${cell_size}px${NC}"
    echo -e "${CYAN}Frame delay: ${YELLOW}${delay}ms${NC}"
    echo
    
    echo -e "${GREEN}Generating GIF...${NC}"
    
    # Make sure output directory exists
    mkdir -p "$(dirname "$output_file")"
    
    "$SCRIPT_DIR/target/release/examples/visualizer" gif "$pattern" "$output_file" "$theme" "$generations" "$cell_size" "$delay"
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}GIF created successfully: ${YELLOW}$output_file${NC}"
    else
        echo -e "${RED}Failed to create GIF!${NC}"
    fi
}

visualize_images() {
    local pattern=$1
    local output_dir=$2
    local theme=${3:-matrix}
    local generations=${4:-20}
    local cell_size=${5:-20}
    
    echo -e "${CYAN}Creating image sequence for pattern: ${YELLOW}$pattern${NC}"
    echo -e "${CYAN}Output directory: ${YELLOW}$output_dir${NC}"
    echo -e "${CYAN}Theme: ${YELLOW}$theme${NC}"
    echo -e "${CYAN}Generations: ${YELLOW}$generations${NC}"
    echo -e "${CYAN}Cell size: ${YELLOW}${cell_size}px${NC}"
    echo
    
    echo -e "${GREEN}Generating images...${NC}"
    
    # Make sure output directory exists
    mkdir -p "$output_dir"
    
    "$SCRIPT_DIR/target/release/examples/visualizer" images "$pattern" "$output_dir" "$theme" "$generations" "$cell_size"
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}Images created successfully in: ${YELLOW}$output_dir${NC}"
    else
        echo -e "${RED}Failed to create images!${NC}"
    fi
}

visualize_evolution() {
    local pattern=$1
    local output_file=$2
    local generations=${3:-20}
    local columns=${4:-4}
    local theme=${5:-matrix}
    local cell_size=${6:-15}
    
    echo -e "${CYAN}Creating evolution image for pattern: ${YELLOW}$pattern${NC}"
    echo -e "${CYAN}Output file: ${YELLOW}$output_file${NC}"
    echo -e "${CYAN}Generations: ${YELLOW}$generations${NC}"
    echo -e "${CYAN}Columns: ${YELLOW}$columns${NC}"
    echo -e "${CYAN}Theme: ${YELLOW}$theme${NC}"
    echo -e "${CYAN}Cell size: ${YELLOW}${cell_size}px${NC}"
    echo
    
    echo -e "${GREEN}Generating evolution image...${NC}"
    
    # Make sure output directory exists
    mkdir -p "$(dirname "$output_file")"
    
    "$SCRIPT_DIR/target/release/examples/visualizer" evolution "$pattern" "$output_file" "$theme" "$generations" "$columns" "$cell_size"
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}Evolution image created successfully: ${YELLOW}$output_file${NC}"
    else
        echo -e "${RED}Failed to create evolution image!${NC}"
    fi
}

batch_process() {
    local mode=$1
    local output_dir=$2
    local theme=${3:-matrix}
    
    echo -e "${CYAN}Batch processing all patterns...${NC}"
    echo -e "${CYAN}Mode: ${YELLOW}$mode${NC}"
    echo -e "${CYAN}Output directory: ${YELLOW}$output_dir${NC}"
    echo -e "${CYAN}Theme: ${YELLOW}$theme${NC}"
    echo
    
    # Make sure output directory exists
    mkdir -p "$output_dir"
    
    # Get all patterns
    patterns=$(cd "$SCRIPT_DIR" && "$SCRIPT_DIR/target/release/examples/visualizer" 2>&1 | grep "^ *- " | awk '{print $2}')
    
    for pattern in $patterns; do
        echo -e "${YELLOW}Processing pattern: $pattern${NC}"
        
        case $mode in
            gif)
                visualize_gif "$pattern" "$output_dir/${pattern,,}.gif" "$theme"
                ;;
            images)
                visualize_images "$pattern" "$output_dir/${pattern,,}_frames" "$theme"
                ;;
            evolution)
                visualize_evolution "$pattern" "$output_dir/${pattern,,}_evolution.png" 20 4 "$theme"
                ;;
            *)
                echo -e "${RED}Unknown mode: $mode${NC}"
                return 1
                ;;
        esac
        
        echo
    done
    
    echo -e "${GREEN}Batch processing complete!${NC}"
}

show_menu() {
    clear
    print_header
    
    echo -e "${GREEN}Conway's Game of Life Pattern Visualizer${NC}"
    echo -e "${YELLOW}This tool creates visual representations of Game of Life patterns.${NC}"
    echo
    
    echo -e "${GREEN}Available options:${NC}"
    echo
    
    echo -e "${MAGENTA}1${NC}. ${YELLOW}Create animated GIF${NC}"
    echo -e "${MAGENTA}2${NC}. ${YELLOW}Create image sequence${NC}"
    echo -e "${MAGENTA}3${NC}. ${YELLOW}Create evolution composite image${NC}"
    echo -e "${MAGENTA}4${NC}. ${YELLOW}Batch process all patterns${NC}"
    echo -e "${MAGENTA}q${NC}. ${RED}Quit${NC}"
    echo
    
    read -p "Select an option (1-4) or 'q' to quit: " choice
    
    case $choice in
        1)
            clear
            print_header
            echo -e "${GREEN}Create Animated GIF${NC}"
            echo
            
            # Get list of available patterns
            check_executable
            patterns=$(cd "$SCRIPT_DIR" && "$SCRIPT_DIR/target/release/examples/visualizer" 2>&1 | grep "^ *- " | awk '{print $2}')
            
            # Display patterns in columns
            echo -e "${CYAN}Available patterns:${NC}"
            echo -e "${YELLOW}$(echo "$patterns" | column)${NC}"
            echo
            
            read -p "Enter pattern name: " pattern
            echo
            
            read -p "Enter output file path: " output_file
            
            if [[ -z "$output_file" ]]; then
                echo -e "${RED}No output file specified.${NC}"
                sleep 1
                return
            fi
            
            # Get theme
            echo -e "${CYAN}Available themes:${NC}"
            echo -e "${YELLOW}classic${NC} - Black and white"
            echo -e "${YELLOW}matrix${NC} - Green on black"
            echo -e "${YELLOW}ocean${NC} - Blue gradient"
            echo -e "${YELLOW}inferno${NC} - Fire colors"
            echo -e "${YELLOW}rainbow${NC} - Multiple colors"
            echo
            read -p "Select theme [matrix]: " theme
            theme=${theme:-matrix}
            
            read -p "Number of generations [100]: " generations
            generations=${generations:-100}
            
            read -p "Cell size in pixels [10]: " cell_size
            cell_size=${cell_size:-10}
            
            read -p "Frame delay in milliseconds [100]: " delay
            delay=${delay:-100}
            
            clear
            print_header
            visualize_gif "$pattern" "$output_file" "$theme" "$generations" "$cell_size" "$delay"
            echo
            read -p "Press Enter to return to the menu..."
            ;;
        2)
            clear
            print_header
            echo -e "${GREEN}Create Image Sequence${NC}"
            echo
            
            # Get list of available patterns
            check_executable
            patterns=$(cd "$SCRIPT_DIR" && "$SCRIPT_DIR/target/release/examples/visualizer" 2>&1 | grep "^ *- " | awk '{print $2}')
            
            # Display patterns in columns
            echo -e "${CYAN}Available patterns:${NC}"
            echo -e "${YELLOW}$(echo "$patterns" | column)${NC}"
            echo
            
            read -p "Enter pattern name: " pattern
            echo
            
            read -p "Enter output directory: " output_dir
            
            if [[ -z "$output_dir" ]]; then
                echo -e "${RED}No output directory specified.${NC}"
                sleep 1
                return
            fi
            
            # Get theme
            echo -e "${CYAN}Available themes:${NC}"
            echo -e "${YELLOW}classic${NC} - Black and white"
            echo -e "${YELLOW}matrix${NC} - Green on black"
            echo -e "${YELLOW}ocean${NC} - Blue gradient"
            echo -e "${YELLOW}inferno${NC} - Fire colors"
            echo -e "${YELLOW}rainbow${NC} - Multiple colors"
            echo
            read -p "Select theme [matrix]: " theme
            theme=${theme:-matrix}
            
            read -p "Number of generations [20]: " generations
            generations=${generations:-20}
            
            read -p "Cell size in pixels [20]: " cell_size
            cell_size=${cell_size:-20}
            
            clear
            print_header
            visualize_images "$pattern" "$output_dir" "$theme" "$generations" "$cell_size"
            echo
            read -p "Press Enter to return to the menu..."
            ;;
        3)
            clear
            print_header
            echo -e "${GREEN}Create Evolution Composite Image${NC}"
            echo
            
            # Get list of available patterns
            check_executable
            patterns=$(cd "$SCRIPT_DIR" && "$SCRIPT_DIR/target/release/examples/visualizer" 2>&1 | grep "^ *- " | awk '{print $2}')
            
            # Display patterns in columns
            echo -e "${CYAN}Available patterns:${NC}"
            echo -e "${YELLOW}$(echo "$patterns" | column)${NC}"
            echo
            
            read -p "Enter pattern name: " pattern
            echo
            
            read -p "Enter output file path: " output_file
            
            if [[ -z "$output_file" ]]; then
                echo -e "${RED}No output file specified.${NC}"
                sleep 1
                return
            fi
            
            read -p "Number of generations [20]: " generations
            generations=${generations:-20}
            
            read -p "Number of columns [4]: " columns
            columns=${columns:-4}
            
            # Get theme
            echo -e "${CYAN}Available themes:${NC}"
            echo -e "${YELLOW}classic${NC} - Black and white"
            echo -e "${YELLOW}matrix${NC} - Green on black"
            echo -e "${YELLOW}ocean${NC} - Blue gradient"
            echo -e "${YELLOW}inferno${NC} - Fire colors"
            echo -e "${YELLOW}rainbow${NC} - Multiple colors"
            echo
            read -p "Select theme [matrix]: " theme
            theme=${theme:-matrix}
            
            read -p "Cell size in pixels [15]: " cell_size
            cell_size=${cell_size:-15}
            
            clear
            print_header
            visualize_evolution "$pattern" "$output_file" "$generations" "$columns" "$theme" "$cell_size"
            echo
            read -p "Press Enter to return to the menu..."
            ;;
        4)
            clear
            print_header
            echo -e "${GREEN}Batch Process All Patterns${NC}"
            echo
            
            echo -e "${CYAN}Select processing mode:${NC}"
            echo -e "${YELLOW}1${NC}. Create GIFs for all patterns"
            echo -e "${YELLOW}2${NC}. Create image sequences for all patterns"
            echo -e "${YELLOW}3${NC}. Create evolution images for all patterns"
            echo
            
            read -p "Select mode (1-3): " mode_choice
            
            case $mode_choice in
                1) local mode="gif" ;;
                2) local mode="images" ;;
                3) local mode="evolution" ;;
                *)
                    echo -e "${RED}Invalid choice.${NC}"
                    sleep 1
                    return
                    ;;
            esac
            
            read -p "Enter output directory: " output_dir
            
            if [[ -z "$output_dir" ]]; then
                echo -e "${RED}No output directory specified.${NC}"
                sleep 1
                return
            fi
            
            # Get theme
            echo -e "${CYAN}Available themes:${NC}"
            echo -e "${YELLOW}classic${NC} - Black and white"
            echo -e "${YELLOW}matrix${NC} - Green on black"
            echo -e "${YELLOW}ocean${NC} - Blue gradient"
            echo -e "${YELLOW}inferno${NC} - Fire colors"
            echo -e "${YELLOW}rainbow${NC} - Multiple colors"
            echo
            read -p "Select theme [matrix]: " theme
            theme=${theme:-matrix}
            
            clear
            print_header
            batch_process "$mode" "$output_dir" "$theme"
            echo
            read -p "Press Enter to return to the menu..."
            ;;
        q|Q)
            exit 0
            ;;
        *)
            echo -e "${RED}Invalid choice. Please try again.${NC}"
            sleep 1
            ;;
    esac
}

# Check for arguments
if [[ $# -gt 0 ]]; then
    check_executable
    
    # Handle command line args
    case $1 in
        "gif")
            if [[ $# -lt 3 ]]; then
                echo -e "${RED}Missing parameters. Usage: $0 gif <pattern_name> <output_file> [theme] [generations] [cell_size] [delay]${NC}"
                exit 1
            fi
            visualize_gif "$2" "$3" "${4:-matrix}" "${5:-100}" "${6:-10}" "${7:-100}"
            ;;
        "images")
            if [[ $# -lt 3 ]]; then
                echo -e "${RED}Missing parameters. Usage: $0 images <pattern_name> <output_dir> [theme] [generations] [cell_size]${NC}"
                exit 1
            fi
            visualize_images "$2" "$3" "${4:-matrix}" "${5:-20}" "${6:-20}"
            ;;
        "evolution")
            if [[ $# -lt 3 ]]; then
                echo -e "${RED}Missing parameters. Usage: $0 evolution <pattern_name> <output_file> [generations] [columns] [theme] [cell_size]${NC}"
                exit 1
            fi
            visualize_evolution "$2" "$3" "${4:-20}" "${5:-4}" "${6:-matrix}" "${7:-15}"
            ;;
        "batch")
            if [[ $# -lt 3 ]]; then
                echo -e "${RED}Missing parameters. Usage: $0 batch <mode> <output_dir> [theme]${NC}"
                echo -e "${RED}Mode can be: gif, images, or evolution${NC}"
                exit 1
            fi
            batch_process "$2" "$3" "${4:-matrix}"
            ;;
        *)
            echo -e "${RED}Unknown command: $1${NC}"
            echo -e "${YELLOW}Available commands: gif, images, evolution, batch${NC}"
            exit 1
            ;;
    esac
else
    # Run interactive menu
    while true; do
        check_executable
        show_menu
    done
fi