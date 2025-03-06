use std::process::exit;
use clap::{Parser, Subcommand};

mod helper;

fn main() {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Generate { name } => {
            if helper::is_flutter_project() {
                println!("This is a not flutter project");
                exit(0)
            }
            println!("Generating specifications for {}", name);
            create_bloc_files(&name);
        }
    }
}

#[derive(Parser, Debug)]
#[command(name = "fgr", about = "This is CLI", version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Generate { name: String },
}


fn create_bloc_files(name: &str) {
    let name_path = format!("./lib/ui/{}", name);

    println!("name: {}", name_path);
}