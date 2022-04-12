use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a new weblab environment.
    /// May also ask you to log in, and pulls
    /// the template (or your existing solution) from weblab,
    Start {
        // TODO: wrapper around weblab urls, validating that they're
        //  really from weblab, and providing access to what course and assignment
        //  they are from
        url: String,
    },
    /// Exits the current environment. This deletes the folder
    /// and all temporary files.
    End,

    /// Provide credentials so your assignments can be accessed.
    Login,

    /// Push your solution to weblab, saving it.
    Push,
    /// Download either the template (if you have not made a solution yet)
    /// or your existing solution.
    Pull,

    Config,

    Test,
    Build,
}

fn main() {
    let cli: Cli = Cli::parse();

    match cli.command {
        Commands::Start { .. } => {
            todo!()
        }
        Commands::End => {
            todo!()
        }
        Commands::Login => {
            todo!()
        }
        Commands::Push => {
            todo!()
        }
        Commands::Pull => {
            todo!()
        }
        Commands::Config => {
            todo!()
        }
        Commands::Test => {
            todo!()
        }
        Commands::Build => {
            todo!()
        }
    }
}
