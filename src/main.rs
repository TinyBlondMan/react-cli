use clap::{command, Args, Parser, Subcommand};
mod api;

fn main() {
    #[derive(Parser)]
    #[command(author, version)]
    #[command(
        about = "A CLI thought for boosting React productivity.",
        long_about = "A CLI thought for boosting React productivity and avoid boilerplate. 
        This comes from the lassitude you can have always installing the same dependencies
        creating over and over the same folders, files, routing system, 
        copying hooks, constants, utils, and helper functions from one project to another"
    )]

    struct CLI {
        #[command(subcommand)]
        command: Option<Commands>,
    }

    #[derive(Subcommand)]
    enum Commands {
        // Reverses a string
        Reverse(Reverse),
        // Inspects a string
        Inspect(Inspect),
    }

    #[derive(Args)]
    struct Reverse {
        // The string to reverse
        string: Option<String>,
    }

    #[derive(Args)]
    struct Inspect {
        // The string to inspect
        string: Option<String>,

        #[arg(short = 'd', long = "digits")]
        only_digits: bool,
    }
}
