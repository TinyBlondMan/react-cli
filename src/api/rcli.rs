use super::creates_helpers::{
    create_folder_structure,
    create_src_files,
    init_vite_project,
    // write_in_file,
};

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

    let extension: String = if !has_javascript {
        String::from(".ts")
    } else {
        String::from(".js")
    };

    // Defines package manager
    let package: String = if !has_npm {
        String::from("yarn")
    } else {
        String::from("npm")
    };

    // Everything runs in these three functions
    init_vite_project(&package, &project_name, &language);
    create_folder_structure(&project_name);
    create_src_files(&project_name, &extension);

    // Writing example links constants in links.tsx
    // match write_in_file(
    //     String::from("./my-test-project/src/routes/"),
    //     String::from("links.ts"),
    //     &return_links(),
    // ) {
    //     Err(e) => println!("{:?}", e),
    //     _ => (),
    // }

    return (project_name, has_javascript, has_npm);
}
