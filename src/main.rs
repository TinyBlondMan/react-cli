mod api;
use api::commands::{Component, Project, Router};

use clap::{command, Parser, Subcommand};

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
    /// Creates a new React project
    Project(Project),
    /// Creates a router (TODO)
    Router(Router),
    /// Creates a component (ONGOING)
    Component(Component),
}

fn main() {
    let cli = CLI::parse();

    match &cli.command {
        Some(Commands::Project(name)) => match name.project_name {
            Some(ref _name) => {
                let (proj, js, npm) =
                    api::rcli::create_project(_name, name.with_javascript, name.with_npm);
                println!("\nCreated new project {} with React-CLI", proj);
                if !js {
                    println!("Project language: Typescript (default)");
                } else {
                    println!("Project language: Javascript");
                }
                if !npm {
                    println!("Project package manager: Yarn (default)");
                } else {
                    println!("Project package manager: Npm");
                }

                println!("\nDone. Now run:\n");
                println!("   cd {}", proj);
                println!("   yarn install / npm install");
                println!("   yarn dev / npm run dev\n");
                println!("Have fun!\n");
            }
            None => {
                println!("Please provide a project name");
            }
        },
        Some(Commands::Router(name)) => match name.routes_folder_name {
            Some(ref _name) => {
                let (folder, js, npm, links, routing) = api::rcli::create_router(
                    _name,
                    name.with_javascript,
                    name.with_npm,
                    name.with_links,
                    name.with_routing_system,
                );
                println!("\nCreating new router in /{} with React-CLI", folder);
                if !js {
                    println!("Project language: Typescript (default)");
                } else {
                    println!("Project language: Javascript");
                }
                if !npm {
                    println!("Project package manager: Yarn (default)");
                } else {
                    println!("Project package manager: Npm");
                }
                if links || routing {
                    println!("We will add some basic links to help you start!");
                }
                if routing {
                    println!("We will add a basic routing system to help you start!");
                }
            }
            None => {
                println!("Please provide a route folder name");
            }
        },
        Some(Commands::Component(name)) => match name.component_name {
            Some(ref _name) => {
                let (comp, comp_fmt, comp_type, js, no_style, css) = api::rcli::create_component(
                    _name,
                    &name.comp_format,
                    &name.comp_type,
                    name.with_javascript,
                    name.with_css,
                    name.without_style,
                );
                println!(
                    "\nCreated new component in src/components/{}/{}",
                    comp_type, comp
                );
                println!("Component format choosen: {}", comp_fmt);
                println!("Component type choosen: {}", comp_type);
                if !js {
                    println!("Created index.tsx.");
                } else {
                    println!("Created index.jsx.");
                }
                if no_style {
                    println!("Component will only be an index file with no CSS.")
                }
                if css {
                    println!("Component created with basic CSS and not SCSS.")
                }
            }
            None => {
                println!("Please provide a component name");
            }
        },
        None => {}
    }
}
