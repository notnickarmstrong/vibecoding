#!/bin/bash
# Conway's Game of Life Pattern Analyzer
# Analyzes and compares different patterns

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
    echo -e "${BLUE}║                 ${YELLOW}${BOLD}Conway's Game of Life Analyzer${NC}${BLUE}                    ║${NC}"
    echo -e "${BLUE}╚═════════════════════════════════════════════════════════════════════╝${NC}"
    echo
}

check_executable() {
    if [[ ! -f "$SCRIPT_DIR/target/release/examples/analyzer" ]]; then
        echo -e "${YELLOW}Building Conway's Game of Life Pattern Analyzer...${NC}"
        
        # Create the analyzer example if it doesn't exist
        if [[ ! -f "$SCRIPT_DIR/examples/analyzer.rs" ]]; then
            echo -e "${YELLOW}Creating analyzer runner...${NC}"
            mkdir -p "$SCRIPT_DIR/examples"
            
            cat > "$SCRIPT_DIR/examples/analyzer.rs" << 'EOF'
// Conway's Game of Life Pattern Analyzer Example

use std::env;
use std::time::Instant;
use std::fs::File;
use std::io::Write;
use conway::analyzer::PatternAnalyzer;
use conway::patterns::PatternLibrary;
use conway::config::BoundaryType;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Default parameters
    let mut analyze_mode = "all";
    let mut output_file = None;
    let mut max_generations = 1000;
    let mut grid_size = (100, 100);
    
    // Parse command line args
    if args.len() > 1 {
        analyze_mode = &args[1];
    }
    
    if args.len() > 2 {
        output_file = Some(&args[2]);
    }
    
    if args.len() > 3 {
        max_generations = args[3].parse().unwrap_or(1000);
    }
    
    if args.len() > 4 {
        let size = args[4].parse().unwrap_or(100);
        grid_size = (size, size);
    }
    
    println!("Conway's Game of Life Pattern Analyzer");
    println!("======================================");
    println!();
    println!("Analysis parameters:");
    println!("  Mode: {}", analyze_mode);
    println!("  Max generations: {}", max_generations);
    println!("  Grid size: {}x{}", grid_size.0, grid_size.1);
    if let Some(file) = output_file {
        println!("  Output file: {}", file);
    }
    println!();
    
    // Create the analyzer
    let analyzer = PatternAnalyzer::new(
        max_generations,
        grid_size,
        BoundaryType::Wrap,
    );
    
    let start_time = Instant::now();
    
    match analyze_mode {
        "all" => {
            println!("Analyzing all built-in patterns...");
            let patterns = PatternLibrary::get_all_patterns();
            
            let mut pattern_configs = Vec::new();
            for pattern in &patterns {
                // Place each pattern in the center of the grid
                let x = grid_size.0 / 2 - pattern.width / 2;
                let y = grid_size.1 / 2 - pattern.height / 2;
                pattern_configs.push((pattern, x, y));
            }
            
            // Analyze and compare all patterns
            let stats = analyzer.compare_patterns(&pattern_configs);
            let report = analyzer.generate_comparison_report(&stats);
            
            // Print the comparison report
            println!("\n{}", report);
            
            // Save individual reports to file if requested
            if let Some(file_prefix) = output_file {
                for stat in &stats {
                    let detail_report = stat.generate_report();
                    let file_name = format!("{}-{}.txt", file_prefix, stat.name.to_lowercase());
                    
                    if let Ok(mut file) = File::create(&file_name) {
                        if let Err(e) = file.write_all(detail_report.as_bytes()) {
                            eprintln!("Error writing to file {}: {}", file_name, e);
                        } else {
                            println!("Detail report saved to: {}", file_name);
                        }
                    } else {
                        eprintln!("Error creating file: {}", file_name);
                    }
                }
                
                // Also save the comparison report
                let comparison_file = format!("{}-comparison.txt", file_prefix);
                if let Ok(mut file) = File::create(&comparison_file) {
                    if let Err(e) = file.write_all(report.as_bytes()) {
                        eprintln!("Error writing to file {}: {}", comparison_file, e);
                    } else {
                        println!("Comparison report saved to: {}", comparison_file);
                    }
                } else {
                    eprintln!("Error creating file: {}", comparison_file);
                }
            }
        },
        pattern_name => {
            println!("Analyzing pattern: {}", pattern_name);
            
            if let Some(pattern) = PatternLibrary::get_by_name(pattern_name) {
                // Place pattern in the center of the grid
                let x = grid_size.0 / 2 - pattern.width / 2;
                let y = grid_size.1 / 2 - pattern.height / 2;
                
                // Analyze the pattern
                let stats = analyzer.analyze_pattern(pattern, x, y);
                let report = stats.generate_report();
                
                // Print the report
                println!("\n{}", report);
                
                // Save report to file if requested
                if let Some(file_name) = output_file {
                    if let Ok(mut file) = File::create(file_name) {
                        if let Err(e) = file.write_all(report.as_bytes()) {
                            eprintln!("Error writing to file {}: {}", file_name, e);
                        } else {
                            println!("Report saved to: {}", file_name);
                        }
                    } else {
                        eprintln!("Error creating file: {}", file_name);
                    }
                }
            } else {
                eprintln!("Error: Pattern '{}' not found", pattern_name);
                println!("Available patterns:");
                for pattern in PatternLibrary::get_all_patterns() {
                    println!("  - {}", pattern.name);
                }
            }
        }
    }
    
    let elapsed = start_time.elapsed();
    println!("\nTotal analysis time: {:.2?}", elapsed);
}
EOF
        }
        
        cd "$SCRIPT_DIR" && cargo build --release --example analyzer
        if [ $? -ne 0 ]; then
            echo -e "${RED}Failed to build the analyzer!${NC}"
            exit 1
        fi
        echo -e "${GREEN}Build successful!${NC}"
    fi
}

analyze_pattern() {
    local pattern=$1
    local output_file=$2
    local max_generations=${3:-1000}
    local grid_size=${4:-100}
    
    echo -e "${CYAN}Analyzing pattern: ${YELLOW}$pattern${NC}"
    echo -e "${CYAN}Max generations: ${YELLOW}$max_generations${NC}"
    echo -e "${CYAN}Grid size: ${YELLOW}${grid_size}x${grid_size}${NC}"
    echo
    
    echo -e "${GREEN}Running analysis...${NC}"
    
    local cmd="$SCRIPT_DIR/target/release/examples/analyzer"
    
    if [[ -n "$output_file" ]]; then
        cmd="$cmd $pattern $output_file $max_generations $grid_size"
    else
        cmd="$cmd $pattern"
    fi
    
    $cmd
    
    echo
    echo -e "${GREEN}Analysis complete!${NC}"
}

compare_patterns() {
    local output_prefix=$1
    local max_generations=${2:-1000}
    local grid_size=${3:-100}
    
    echo -e "${CYAN}Comparing all built-in patterns${NC}"
    echo -e "${CYAN}Max generations: ${YELLOW}$max_generations${NC}"
    echo -e "${CYAN}Grid size: ${YELLOW}${grid_size}x${grid_size}${NC}"
    echo
    
    echo -e "${GREEN}Running comparison...${NC}"
    
    local cmd="$SCRIPT_DIR/target/release/examples/analyzer all"
    
    if [[ -n "$output_prefix" ]]; then
        cmd="$cmd $output_prefix $max_generations $grid_size"
    fi
    
    $cmd
    
    echo
    echo -e "${GREEN}Comparison complete!${NC}"
}

analyze_custom_patterns() {
    local patternset=$1
    local output_prefix=$2
    local max_generations=${3:-1000}
    local grid_size=${4:-100}
    
    case $patternset in
        "oscillators")
            echo -e "${CYAN}Analyzing oscillator patterns${NC}"
            local patterns=("blinker" "toad" "beacon" "pulsar")
            ;;
        "spaceships")
            echo -e "${CYAN}Analyzing spaceship patterns${NC}"
            local patterns=("glider" "lwss")
            ;;
        "methuselahs")
            echo -e "${CYAN}Analyzing methuselah patterns${NC}"
            local patterns=("r-pentomino" "diehard" "acorn")
            ;;
        *)
            echo -e "${RED}Unknown pattern set: $patternset${NC}"
            echo -e "${YELLOW}Available sets: oscillators, spaceships, methuselahs${NC}"
            return
            ;;
    esac
    
    echo -e "${CYAN}Max generations: ${YELLOW}$max_generations${NC}"
    echo -e "${CYAN}Grid size: ${YELLOW}${grid_size}x${grid_size}${NC}"
    echo
    
    for pattern in "${patterns[@]}"; do
        echo -e "${YELLOW}Analyzing: $pattern${NC}"
        
        local output_file=""
        if [[ -n "$output_prefix" ]]; then
            output_file="${output_prefix}-${pattern}.txt"
        fi
        
        analyze_pattern "$pattern" "$output_file" "$max_generations" "$grid_size"
        echo
    done
    
    echo -e "${GREEN}Pattern set analysis complete!${NC}"
}

show_menu() {
    clear
    print_header
    
    echo -e "${GREEN}Conway's Game of Life Pattern Analyzer${NC}"
    echo -e "${YELLOW}This tool analyzes and compares different patterns to understand their behavior.${NC}"
    echo
    
    echo -e "${GREEN}Available options:${NC}"
    echo
    
    echo -e "${MAGENTA}1${NC}. ${YELLOW}Analyze a specific pattern${NC}"
    echo -e "${MAGENTA}2${NC}. ${YELLOW}Compare all patterns${NC}"
    echo -e "${MAGENTA}3${NC}. ${YELLOW}Analyze pattern sets (oscillators, spaceships, etc.)${NC}"
    echo -e "${MAGENTA}4${NC}. ${YELLOW}Save analysis to file${NC}"
    echo -e "${MAGENTA}q${NC}. ${RED}Quit${NC}"
    echo
    
    read -p "Select an option (1-4) or 'q' to quit: " choice
    
    case $choice in
        1)
            clear
            print_header
            echo -e "${GREEN}Available patterns:${NC}"
            echo
            
            # Get list of available patterns
            check_executable
            patterns=$(cd "$SCRIPT_DIR" && cargo run --quiet --release --example analyzer unknown 2>&1 | grep "^ *- " | awk '{print $2}')
            
            # Display patterns in columns
            echo -e "${CYAN}$(echo "$patterns" | column)${NC}"
            echo
            
            read -p "Enter pattern name to analyze: " pattern
            echo
            
            read -p "Maximum generations to simulate [1000]: " max_generations
            max_generations=${max_generations:-1000}
            
            read -p "Grid size [100]: " grid_size
            grid_size=${grid_size:-100}
            
            clear
            print_header
            analyze_pattern "$pattern" "" "$max_generations" "$grid_size"
            echo
            read -p "Press Enter to return to the menu..."
            ;;
        2)
            clear
            print_header
            
            read -p "Maximum generations to simulate [1000]: " max_generations
            max_generations=${max_generations:-1000}
            
            read -p "Grid size [100]: " grid_size
            grid_size=${grid_size:-100}
            
            read -p "Save reports to file? (y/n) [n]: " save_to_file
            
            local output_prefix=""
            if [[ "$save_to_file" == "y" || "$save_to_file" == "Y" ]]; then
                read -p "Enter output file prefix: " output_prefix
            fi
            
            clear
            print_header
            compare_patterns "$output_prefix" "$max_generations" "$grid_size"
            echo
            read -p "Press Enter to return to the menu..."
            ;;
        3)
            clear
            print_header
            echo -e "${GREEN}Available pattern sets:${NC}"
            echo
            echo -e "${CYAN}1${NC}. ${YELLOW}Oscillators${NC} (blinker, toad, beacon, pulsar)"
            echo -e "${CYAN}2${NC}. ${YELLOW}Spaceships${NC} (glider, lwss)"
            echo -e "${CYAN}3${NC}. ${YELLOW}Methuselahs${NC} (r-pentomino, diehard, acorn)"
            echo
            
            read -p "Select a pattern set (1-3): " patternset_choice
            
            case $patternset_choice in
                1) local patternset="oscillators" ;;
                2) local patternset="spaceships" ;;
                3) local patternset="methuselahs" ;;
                *) 
                    echo -e "${RED}Invalid choice.${NC}"
                    sleep 1
                    return
                    ;;
            esac
            
            read -p "Maximum generations to simulate [1000]: " max_generations
            max_generations=${max_generations:-1000}
            
            read -p "Grid size [100]: " grid_size
            grid_size=${grid_size:-100}
            
            read -p "Save reports to file? (y/n) [n]: " save_to_file
            
            local output_prefix=""
            if [[ "$save_to_file" == "y" || "$save_to_file" == "Y" ]]; then
                read -p "Enter output file prefix: " output_prefix
            fi
            
            clear
            print_header
            analyze_custom_patterns "$patternset" "$output_prefix" "$max_generations" "$grid_size"
            echo
            read -p "Press Enter to return to the menu..."
            ;;
        4)
            clear
            print_header
            echo -e "${GREEN}Save Pattern Analysis to File${NC}"
            echo
            
            # Get list of available patterns
            check_executable
            patterns=$(cd "$SCRIPT_DIR" && cargo run --quiet --release --example analyzer unknown 2>&1 | grep "^ *- " | awk '{print $2}')
            
            # Display patterns in columns
            echo -e "${CYAN}Available patterns:${NC}"
            echo -e "${YELLOW}$(echo "$patterns" | column)${NC}"
            echo
            
            read -p "Enter pattern name to analyze: " pattern
            echo
            
            read -p "Enter output file name: " output_file
            
            if [[ -z "$output_file" ]]; then
                echo -e "${RED}No output file specified.${NC}"
                sleep 1
                return
            fi
            
            read -p "Maximum generations to simulate [1000]: " max_generations
            max_generations=${max_generations:-1000}
            
            read -p "Grid size [100]: " grid_size
            grid_size=${grid_size:-100}
            
            clear
            print_header
            analyze_pattern "$pattern" "$output_file" "$max_generations" "$grid_size"
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
        "analyze")
            if [[ $# -lt 2 ]]; then
                echo -e "${RED}Missing pattern name. Usage: $0 analyze <pattern_name> [output_file] [max_generations] [grid_size]${NC}"
                exit 1
            fi
            analyze_pattern "$2" "${3:-}" "${4:-1000}" "${5:-100}"
            ;;
        "compare")
            compare_patterns "${2:-}" "${3:-1000}" "${4:-100}"
            ;;
        "set")
            if [[ $# -lt 2 ]]; then
                echo -e "${RED}Missing pattern set. Usage: $0 set <pattern_set> [output_prefix] [max_generations] [grid_size]${NC}"
                exit 1
            fi
            analyze_custom_patterns "$2" "${3:-}" "${4:-1000}" "${5:-100}"
            ;;
        *)
            echo -e "${RED}Unknown command: $1${NC}"
            echo -e "${YELLOW}Available commands: analyze, compare, set${NC}"
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