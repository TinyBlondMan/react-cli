use clap::{command, Args, Parser, Subcommand};
mod api;

#[derive(Parser)]
#[command(author, version)]
#[command(
    about = "A React-oriented CLI thought for boosting React productivity.",
    long_about = "React-CLI
A CLI thought for boosting React productivity and avoid boilerplate. 
This comes from the lassitude you can have always installing the same dependencies, creating over and over the same folders, files, routing system, copying hooks, constants, utils, and helper functions from one project to another"
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
    Project(Project),
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

#[derive(Args)]
struct Project {
    // Directory of installation (default value: current directory)
    #[arg(short = 'd', long = "directory")]
    directory: Option<String>,

    // Installed with TS by default, changes to JS if true
    #[arg(short = 'j', long = "javascript")]
    with_javascript: bool,

    // Installed with YARN by default, changes to NPM if true
    #[arg(short = 'n', long = "npm")]
    with_npm: bool,
}

fn main() {
    let cli = CLI::parse();

    match &cli.command {
        Some(Commands::Reverse(name)) => match name.string {
            Some(ref _name) => {
                let reverse = api::rcli::reverse(_name);
                println!("{}", reverse);
            }
            None => {
                println!("Please provide a string to reverse");
            }
        },
        Some(Commands::Inspect(name)) => match name.string {
            Some(ref _name) => {
                let (res, kind) = api::rcli::inspect(_name, name.only_digits);

                let mut plural_s = "s";
                if res == 1 {
                    plural_s = "";
                }

                println!("{:?} has {} {}{}.", _name, res, kind, plural_s);
            }
            None => {
                println!("Please provide a string to inspect");
            }
        },
        Some(Commands::Project(name)) => match name.directory {
            Some(ref _name) => {
                let dir = api::rcli::create_project(_name);
                println!("Creating new project with React-CLI in {}", dir);
            }
            None => {
                println!("Please provide a directory");
            }
        },
        None => {}
    }
}
