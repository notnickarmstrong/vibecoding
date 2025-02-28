#!/bin/bash
# Conway's Game of Life Pattern Explorer

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PATTERNS=("glider" "blinker" "toad" "beacon" "pulsar" "glider_gun" "lwss" "r-pentomino" "diehard" "acorn")

# Colors
BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m' # No Color

print_header() {
    echo -e "${BLUE}╔═════════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║                ${YELLOW}Conway's Game of Life Pattern Explorer${BLUE}                ║${NC}"
    echo -e "${BLUE}╚═════════════════════════════════════════════════════════════════════╝${NC}"
    echo
}

print_pattern_info() {
    local pattern=$1
    local description
    
    case $pattern in
        "glider")
            description="The smallest, most common spaceship" ;;
        "blinker")
            description="The smallest oscillator with period 2" ;;
        "toad")
            description="A period 2 oscillator" ;;
        "beacon")
            description="A period 2 oscillator" ;;
        "pulsar")
            description="A period 3 oscillator" ;;
        "glider_gun")
            description="Gosper's Glider Gun - produces gliders periodically" ;;
        "lwss")
            description="Lightweight Spaceship - moves across the grid" ;;
        "r-pentomino")
            description="A methuselah that evolves for many generations" ;;
        "diehard")
            description="A methuselah that vanishes after 130 generations" ;;
        "acorn")
            description="A methuselah that evolves for thousands of generations" ;;
        *)
            description="Unknown pattern" ;;
    esac
    
    echo -e "${GREEN}Pattern: ${YELLOW}$pattern${NC}"
    echo -e "${GREEN}Description: ${CYAN}$description${NC}"
    echo
}

run_pattern() {
    local pattern=$1
    local width=100
    local height=50
    
    # For larger patterns, use a larger grid
    if [[ "$pattern" == "pulsar" || "$pattern" == "glider_gun" ]]; then
        width=150
        height=70
    fi
    
    echo -e "${GREEN}Launching Conway's Game of Life with the '$pattern' pattern...${NC}"
    echo -e "${CYAN}Press Enter to start the simulation once loaded${NC}"
    echo -e "${CYAN}Press 'q' to quit and return to the menu${NC}"
    echo
    
    # Add a small delay for better UX
    sleep 1
    
    # Run Conway's Game of Life with the specified pattern
    if [[ -f "$SCRIPT_DIR/target/release/conway" ]]; then
        "$SCRIPT_DIR/target/release/conway" --width $width --height $height --initial-pattern $pattern
    else
        echo -e "${RED}Error: Conway's Game of Life executable not found!${NC}"
        echo -e "${YELLOW}Building the project...${NC}"
        cd "$SCRIPT_DIR" && cargo build --release
        if [ $? -ne 0 ]; then
            echo -e "${RED}Build failed! Please check your Rust installation and dependencies.${NC}"
            read -p "Press Enter to continue..."
            return
        fi
        "$SCRIPT_DIR/target/release/conway" --width $width --height $height --initial-pattern $pattern
    fi
}

show_menu() {
    print_header
    
    echo -e "${GREEN}Available patterns:${NC}"
    echo
    
    for i in "${!PATTERNS[@]}"; do
        echo -e "${MAGENTA}$((i+1))${NC}. ${YELLOW}${PATTERNS[$i]}${NC}"
    done
    
    echo -e "${MAGENTA}q${NC}. ${RED}Quit${NC}"
    echo
    
    read -p "Select a pattern to display (1-${#PATTERNS[@]}) or 'q' to quit: " choice
    
    if [[ $choice == "q" || $choice == "Q" ]]; then
        exit 0
    elif [[ $choice =~ ^[0-9]+$ && $choice -ge 1 && $choice -le ${#PATTERNS[@]} ]]; then
        clear
        local pattern_index=$((choice-1))
        print_pattern_info "${PATTERNS[$pattern_index]}"
        run_pattern "${PATTERNS[$pattern_index]}"
    else
        echo -e "${RED}Invalid choice. Please try again.${NC}"
        sleep 1
    fi
    
    clear
}

# Main loop
while true; do
    show_menu
done