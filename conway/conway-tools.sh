#!/bin/bash
# Conway's Game of Life Tools
# A utility script for Conway's Game of Life

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
    echo -e "${BLUE}╔══════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║            ${YELLOW}Conway's Game of Life Tools${BLUE}                   ║${NC}"
    echo -e "${BLUE}╚══════════════════════════════════════════════════════════╝${NC}"
    echo
}

check_executable() {
    if [[ ! -f "$SCRIPT_DIR/target/release/conway" ]]; then
        echo -e "${YELLOW}Building Conway's Game of Life...${NC}"
        cd "$SCRIPT_DIR" && cargo build --release
        if [ $? -ne 0 ]; then
            echo -e "${RED}Build failed! Please check your Rust installation and dependencies.${NC}"
            exit 1
        fi
        echo -e "${GREEN}Build successful!${NC}"
    fi
}

run_benchmark() {
    echo -e "${CYAN}${BOLD}Running Performance Benchmarks${NC}"
    echo -e "${YELLOW}This will measure the performance of Conway's Game of Life with different grid sizes and patterns.${NC}"
    echo

    # Build benchmark tool if needed
    if [[ ! -f "$SCRIPT_DIR/target/release/examples/benchmark" ]]; then
        echo -e "${YELLOW}Building benchmark tool...${NC}"
        cd "$SCRIPT_DIR" && cargo build --release --example benchmark
        if [ $? -ne 0 ]; then
            echo -e "${RED}Failed to build benchmark tool!${NC}"
            echo -e "${YELLOW}Creating benchmark example...${NC}"
            
            mkdir -p "$SCRIPT_DIR/examples"
            cat > "$SCRIPT_DIR/examples/benchmark.rs" << 'EOF'
// Conway's Game of Life Benchmark Example

use std::env;
use conway::benchmark::{run_size_benchmarks, run_pattern_benchmarks};

fn main() {
    let args: Vec<String> = env::args().collect();
    let benchmark_type = if args.len() > 1 { &args[1] } else { "all" };
    
    let max_size = if args.len() > 2 {
        args[2].parse().unwrap_or(1000)
    } else {
        1000
    };
    
    let generations = if args.len() > 3 {
        args[3].parse().unwrap_or(100)
    } else {
        100
    };
    
    println!("Conway's Game of Life Benchmark");
    println!("===============================");
    println!();
    
    match benchmark_type {
        "size" => {
            println!("Running size benchmarks with {} generations", generations);
            println!("Max grid size: {}x{}", max_size, max_size);
            println!();
            
            let results = run_size_benchmarks(max_size, generations);
            
            println!("Results:");
            println!("---------");
            for result in results {
                println!("{}", result.to_string());
            }
        },
        "pattern" => {
            println!("Running pattern benchmarks with grid size {}x{} for {} generations", 
                    max_size, max_size, generations);
            println!();
            
            let results = run_pattern_benchmarks(max_size, max_size, generations);
            
            println!("Results:");
            println!("---------");
            for result in results {
                println!("{}", result.to_string());
            }
        },
        _ => {
            println!("Running all benchmarks");
            println!("Max grid size: {}x{}", max_size, max_size);
            println!("Generations: {}", generations);
            println!();
            
            println!("Size Benchmarks:");
            println!("---------------");
            let size_results = run_size_benchmarks(max_size, generations);
            for result in size_results {
                println!("{}", result.to_string());
            }
            
            println!();
            println!("Pattern Benchmarks (500x500):");
            println!("---------------------------");
            let pattern_size = std::cmp::min(500, max_size);
            let pattern_results = run_pattern_benchmarks(pattern_size, pattern_size, generations);
            for result in pattern_results {
                println!("{}", result.to_string());
            }
        }
    }
}
EOF
            cd "$SCRIPT_DIR" && cargo build --release --example benchmark
            if [ $? -ne 0 ]; then
                echo -e "${RED}Still failed to build benchmark tool!${NC}"
                echo -e "${RED}You may need to run the benchmarks manually with the provided benchmark.rs file.${NC}"
                read -p "Press Enter to return to the main menu..."
                return
            fi
        fi
    fi
    
    echo -e "${GREEN}Running benchmarks...${NC}"
    echo -e "${YELLOW}This may take a while for large grid sizes.${NC}"
    echo
    
    "$SCRIPT_DIR/target/release/examples/benchmark"
    
    echo
    echo -e "${GREEN}Benchmark complete!${NC}"
    read -p "Press Enter to return to the main menu..."
}

generate_interesting_pattern() {
    echo -e "${CYAN}${BOLD}Generate Interesting Pattern${NC}"
    echo -e "${YELLOW}This will create a random interesting pattern by combining known patterns.${NC}"
    echo
    
    read -p "Enter grid width [default: 100]: " width
    width=${width:-100}
    
    read -p "Enter grid height [default: 50]: " height
    height=${height:-50}
    
    read -p "Enter pattern complexity (1-10) [default: 5]: " complexity
    complexity=${complexity:-5}
    
    if ! [[ "$complexity" =~ ^[0-9]+$ && "$complexity" -ge 1 && "$complexity" -le 10 ]]; then
        echo -e "${RED}Invalid complexity. Using default value of 5.${NC}"
        complexity=5
    fi
    
    echo
    echo -e "${GREEN}Generating pattern with complexity ${complexity}...${NC}"
    
    # Check if the executable exists
    check_executable
    
    # Create a pattern seed file
    SEED_FILE="$SCRIPT_DIR/pattern_seed.txt"
    echo "$complexity" > "$SEED_FILE"
    
    # Run Conway with the pattern generator option
    echo -e "${CYAN}Launching Conway's Game of Life with the generated pattern...${NC}"
    echo -e "${CYAN}Press Enter to start the simulation once loaded${NC}"
    echo -e "${CYAN}Press 'q' to quit and return to the menu${NC}"
    echo
    
    # Add a small delay for better UX
    sleep 1
    
    "$SCRIPT_DIR/target/release/conway" --width "$width" --height "$height" --generate-from-seed "$SEED_FILE"
    
    # Clean up the seed file
    rm -f "$SEED_FILE"
    
    echo
    echo -e "${GREEN}Done!${NC}"
    read -p "Press Enter to return to the main menu..."
}

optimize_game() {
    echo -e "${CYAN}${BOLD}Optimization Settings${NC}"
    echo -e "${YELLOW}This will allow you to adjust optimization settings for Conway's Game of Life.${NC}"
    echo
    
    echo -e "${GREEN}Current optimizations enabled:${NC}"
    echo -e " ${CYAN}* Bit-packed grid representation${NC}"
    echo -e " ${CYAN}* Parallel processing with Rayon${NC}"
    echo -e " ${CYAN}* Memory-efficient data structures${NC}"
    
    echo
    echo -e "${YELLOW}Additional optimizations:${NC}"
    
    # Display optimization options
    echo -e "${MAGENTA}1${NC}. Enable SIMD instructions (may require supported CPU)"
    echo -e "${MAGENTA}2${NC}. Use spatial hashing for sparse grids"
    echo -e "${MAGENTA}3${NC}. Adjust parallel processing settings"
    echo -e "${MAGENTA}b${NC}. Go back to main menu"
    echo
    
    read -p "Select optimization option: " choice
    
    case $choice in
        1)
            echo -e "${YELLOW}SIMD optimization is experimental and requires rebuilding.${NC}"
            read -p "Would you like to rebuild with SIMD? (y/n): " confirm
            if [[ $confirm == "y" || $confirm == "Y" ]]; then
                echo -e "${GREEN}Rebuilding with SIMD support...${NC}"
                # Example command: cd "$SCRIPT_DIR" && RUSTFLAGS="-C target-feature=+avx2" cargo build --release
                echo -e "${RED}This feature is not yet implemented.${NC}"
            fi
            ;;
        2)
            echo -e "${YELLOW}Spatial hashing is useful for sparse grids with few active cells.${NC}"
            read -p "Would you like to enable spatial hashing? (y/n): " confirm
            if [[ $confirm == "y" || $confirm == "Y" ]]; then
                echo -e "${GREEN}Setting up spatial hashing...${NC}"
                echo -e "${RED}This feature is not yet implemented.${NC}"
            fi
            ;;
        3)
            echo -e "${YELLOW}Rayon parallel processing is already enabled.${NC}"
            echo -e "${YELLOW}You can adjust the number of threads used by setting the RAYON_NUM_THREADS environment variable.${NC}"
            read -p "Enter number of threads (leave empty for default): " threads
            if [[ -n "$threads" ]]; then
                export RAYON_NUM_THREADS=$threads
                echo -e "${GREEN}Set RAYON_NUM_THREADS to $threads${NC}"
            fi
            ;;
        b|B)
            return
            ;;
        *)
            echo -e "${RED}Invalid choice.${NC}"
            ;;
    esac
    
    read -p "Press Enter to return to the optimization menu..."
    optimize_game
}

play_game() {
    check_executable
    
    echo -e "${CYAN}${BOLD}Launch Conway's Game of Life${NC}"
    echo -e "${YELLOW}This will start Conway's Game of Life with custom settings.${NC}"
    echo
    
    read -p "Enter grid width [default: 100]: " width
    width=${width:-100}
    
    read -p "Enter grid height [default: 50]: " height
    height=${height:-50}
    
    read -p "Enter theme (classic, block, dot) [default: block]: " theme
    theme=${theme:-block}
    
    read -p "Enter color theme (green, blue, rainbow) [default: green]: " color
    color=${color:-green}
    
    read -p "Enter initial pattern (leave empty for random): " pattern
    
    echo
    echo -e "${GREEN}Launching Conway's Game of Life...${NC}"
    
    # Build command
    CMD="$SCRIPT_DIR/target/release/conway --width $width --height $height --theme $theme --color-theme $color"
    
    if [[ -n "$pattern" ]]; then
        CMD="$CMD --initial-pattern $pattern"
    fi
    
    # Run Conway
    echo -e "${CYAN}Press Enter to start the simulation once loaded${NC}"
    echo -e "${CYAN}Press 'q' to quit and return to the menu${NC}"
    echo
    
    # Add a small delay for better UX
    sleep 1
    
    $CMD
    
    echo
    echo -e "${GREEN}Done!${NC}"
    read -p "Press Enter to return to the main menu..."
}

show_menu() {
    clear
    print_header
    
    echo -e "${GREEN}Available tools:${NC}"
    echo
    
    echo -e "${MAGENTA}1${NC}. ${YELLOW}Run performance benchmarks${NC}"
    echo -e "${MAGENTA}2${NC}. ${YELLOW}Generate interesting random pattern${NC}"
    echo -e "${MAGENTA}3${NC}. ${YELLOW}Play Conway's Game of Life${NC}"
    echo -e "${MAGENTA}4${NC}. ${YELLOW}Optimization settings${NC}"
    echo -e "${MAGENTA}q${NC}. ${RED}Quit${NC}"
    echo
    
    read -p "Select a tool (1-4) or 'q' to quit: " choice
    
    case $choice in
        1)
            clear
            run_benchmark
            ;;
        2)
            clear
            generate_interesting_pattern
            ;;
        3)
            clear
            play_game
            ;;
        4)
            clear
            optimize_game
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

# Main loop
while true; do
    show_menu
done