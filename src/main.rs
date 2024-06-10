use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Command, 
}

#[derive(Subcommand, Default)]
pub enum Command {
    #[default]
    Run,
    Editor,
    Test,
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Command::Run => todo!(),
        Command::Editor => todo!(),
        Command::Test => todo!(),
    }
}
