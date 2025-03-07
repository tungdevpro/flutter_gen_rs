use clap::{Parser, Subcommand};
use std::process::exit;

mod helper;
mod template;

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
#[command(
    name = "fgr",
    about = "Flutter Generate file",
    version = "1.0.0",
    author = "tungdo21899@gmail.com"
)]
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
    println!("output: {}", name_path);

    std::fs::create_dir_all(&name_path).expect("Cannot create folder");

    let bloc_path = format!("{}/bloc", name_path);
    println!("bloc_path: {}", bloc_path);

    std::fs::create_dir_all(&bloc_path).expect("Cannot create bloc folder");

    std::fs::write(
        format!("{}/{}_bloc.dart", bloc_path, name),
        template::bloc::bloc_content(name),
    )
    .expect("Cannot create file");
}
