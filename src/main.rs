use clap::Parser;

mod cli;
use cli::Cli;
use cli::Commands;

mod core;
use core::Paseo;

fn main() {
    let args = Cli::parse();

    let paseo = Paseo::new(args.pathfile);

    match args.command {
        Commands::Add(add_args) => paseo.add(add_args),
        Commands::List => paseo.list(),
    }
}
