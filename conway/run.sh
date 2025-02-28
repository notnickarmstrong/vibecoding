#!/bin/bash
# Conway's Game of Life launcher

# Build the project
cargo build --release

# Run the game
./target/release/conway "$@"