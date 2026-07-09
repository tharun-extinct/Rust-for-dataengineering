use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(version = "1.0", author = "THARUN", about = "A Marco Polo game")]


struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Play {
        #[arg(short, long)]
        name: String,

    },
}


fn main() {
    println!("Rust Codespace is ready.");

    // Parse command line arguments
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Play { name }) => {
            //println!("Starting the game for player: {}", name);
            // Add game logic here
            let result = hello_macro::marco_polo(name);
            println!("{}", result);

        }
        None => {
            println!("No command provided. Use --help for more information.");
        }
    }
}