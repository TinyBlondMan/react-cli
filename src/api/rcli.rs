use std::fs::File;
use std::io::{Error, Write};
use std::process::Command;

pub fn reverse(input: &String) -> String {
    input.chars().rev().collect()
}

pub fn inspect(input: &String, digits: bool) -> (i32, String) {
    if !digits {
        return (input.len() as i32, String::from("char"));
    } else {
        return (inspect_numbers(input), String::from("digit"));
    }
}

pub fn inspect_numbers(input: &String) -> i32 {
    let mut count = 0;

    for c in input.chars() {
        if c.is_digit(10) {
            count += 1;
        }
    }

    return count;
}

pub fn create_project(
    project_name: &String,
    has_javascript: bool,
    has_npm: bool,
) -> (&String, bool, bool) {
    // Defines vite template
    let language: String = if !has_javascript {
        String::from("react-ts")
    } else {
        String::from("react")
    };

    // Defines package manager
    let package: String = if !has_npm {
        String::from("yarn")
    } else {
        String::from("npm")
    };

    let links = "// MAIN ROUTES
export const HOME_LINK: string = '/';
export const ABOUT_LINK: string = '/à-propos';
export const EVENTS_LINK: string = '/events';
export const BLOG_LINK: string = '/blog';
export const HOME_ORGANISATEURS_LINK = '/espace-organisateurs';

// NESTED ROUTES
export const SINGLE_EVENT_LINK: string = '/events';
export const SINGLE_BLOG_LINK: string = '/blog';
export const BILLETS_LINK: string = '/billet';

// LEGALS
export const FAQ_LINK: string = '/foire-aux-questions';
export const MENTIONS_LEGALES_LINK: string = '/mentions-légales';
export const CGU_LINK: string = '/conditions-générales-utilisation';";

    // yarn create vite my-vue-app --template vue
    Command::new(&package)
        .arg("create")
        .arg("vite")
        .arg(project_name)
        .arg("--template")
        .arg(language)
        .output()
        .expect("Project init command failed to start");

    // Test for creation of folder "routes" in src
    Command::new("mkdir")
        .arg(project_name.to_string() + "/src/routes")
        .output()
        .expect("Project init command failed to start");

    // Test for creation of files "links, router & pages" in src/routes
    Command::new("touch")
        .arg(project_name.to_string() + "/src/routes/links.tsx")
        .arg(project_name.to_string() + "/src/routes/router.tsx")
        .arg(project_name.to_string() + "/src/routes/pages.tsx")
        .output()
        .expect("Project init command failed to start");

    // Writing example links constants in links.tsx
    match write_in_file(
        String::from("./my-test-project/src/routes/"),
        String::from("links.tsx"),
        links,
    ) {
        Err(e) => println!("{:?}", e),
        _ => (),
    }

    // yarn install (impossible for now)
    // Command::new(&package)
    //     .arg("install")
    //     .output()
    //     .expect("Package installation command failed to start");

    // yarn dev (impossible for now)
    // Command::new(&package)
    //     .arg("dev")
    //     .output()
    //     .expect("dev command failed to start");

    return (project_name, has_javascript, has_npm);
}

fn write_in_file(dir: String, name: String, text: &str) -> Result<(), Error> {
    let file_name = format!("{}{}", dir, name);
    let mut file = File::create(file_name)?;
    writeln!(file, "{}", text)?; // writing using the macro 'writeln!'
    Ok(())
}
