use clap::{Parser, Subcommand};
use anyhow::Result;


#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    action: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create a new lib project for Godot project
    New{
        /// Name of the project to create
        name: String,
        /// Path to godot project that will use this library
        godot_project: String,
    },
    /// Create a game struct from the derived godot node type
    Class{
        /// Name of the struct to create
        name: String,
        /// Node class to extend from 
        class: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    println!("Hello, world!");

    Ok(())
}
