use clap::Parser;

mod helper;

#[derive(Parser, Debug)]
#[command(name = "Flutter Generator")]
#[command(version = "1.0")]
#[command(about = "A simple to use and efficient", long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,
}

fn main() {
    let is_proj = helper::is_flutter_project();

    if !is_proj {
        println!("Not a Flutter project. Exiting...");
        std::process::exit(0);
        return;
    }
    let args = Args::parse();
}
