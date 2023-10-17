mod api;

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
        // Path to godot project that will use this library
        //godot_project: String,
    },
    /// Create a game struct from the derived godot node type
    Class{
        /// Name of the struct to create
        name: String,
        /// Node class to extend from 
        class: String,
    },
}

// https://github.com/cajun-code/godot-rs-template
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.action {
    Some(Commands::Class { name, class }) => {
        api::generate_class(name, class).await?;
    },
    Some(Commands::New { name}) =>{
        let mut args = cargo_generate::GenerateArgs::default();
        args.template_path.auto_path = Some(String::from("https://github.com/cajun-code/godot-rs-template"));
        args.name = Some(name.clone());
        let path = cargo_generate::generate(args).expect("Could not generate project");
        print!("Generate Project: {:?}", path);
    }
    None => todo!(),
}

    Ok(())
}
