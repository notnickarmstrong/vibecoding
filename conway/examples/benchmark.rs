// Conway's Game of Life Benchmark Example

use std::env;
use conway::benchmark::{run_size_benchmarks, run_pattern_benchmarks};

fn main() {
    let args: Vec<String> = env::args().collect();
    let benchmark_type = if args.len() > 1 { &args[1] } else { "all" };
    
    let max_size = if args.len() > 2 {
        args[2].parse().unwrap_or(500)
    } else {
        500
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
            println!("Pattern Benchmarks (250x250):");
            println!("---------------------------");
            let pattern_size = std::cmp::min(250, max_size);
            let pattern_results = run_pattern_benchmarks(pattern_size, pattern_size, generations);
            for result in pattern_results {
                println!("{}", result.to_string());
            }
        }
    }
}