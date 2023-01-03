use std::{
    fs::{self},
    path::Path,
};

use crate::api::helpers_create::create_component_file;

use super::{
    helpers_create::{
        create_folder_structure, create_src_files, get_current_working_dir, init_vite_project,
    },
    helpers_write::{inject_files, return_files},
};

// -----------------------------------------------
// ----------- Create project function -----------
// -----------------------------------------------
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

    // Defines files extension
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

    // Everything runs in these four functions
    init_vite_project(&package, &project_name, &language);
    create_folder_structure(&project_name);
    create_src_files(&project_name, &extension);
    inject_files(return_files(&project_name, &extension));

    return (project_name, has_javascript, has_npm);
}

// -----------------------------------------------
// ----------- Create router function ------------
// -----------------------------------------------
pub fn create_router(
    routes_folder_name: &String,
    has_javascript: bool,
    has_npm: bool,
    has_basic_links: bool,
    has_routing_system: bool,
) -> (&String, bool, bool, bool, bool) {
    return (
        routes_folder_name,
        has_javascript,
        has_npm,
        has_basic_links,
        has_routing_system,
    );
}

// -----------------------------------------------
// ---------- Create component function ----------
// -----------------------------------------------
pub fn create_component<'a, 'b, 'c>(
    name: &'a String,
    comp_format: &'b Option<String>,
    comp_type: &'c Option<String>,
    has_javascript: bool,
    has_no_style: bool,
    has_css: bool,
) -> (
    &'a String,
    &'b Option<String>,
    &'c Option<String>,
    bool,
    bool,
    bool,
) {
    // Defines files extension
    let js_extension: String = if !has_javascript {
        String::from(".tsx")
    } else {
        String::from(".jsx")
    };
    let css_extension = if !has_css {
        String::from(".scss")
    } else {
        String::from(".css")
    };

    // Treats optional string arguments (comp_format and comp_type)
    let component_type: &str = match comp_type {
        Some(s) => s,
        None => "common",
    };
    let component_format: &str = match comp_format {
        Some(s) => s,
        None => "tsdrpfc",
    };

    // Creating component directory
    let dir_binding = &(get_current_working_dir()
        + &("/src/components/".to_owned())
        + component_type
        + "/"
        + name);
    let component_dir: &Path = Path::new(dir_binding);

    // ---------------------------
    // If component does not exist
    // ---------------------------
    if !component_dir.exists() {
        // Directory creation
        match fs::create_dir_all(component_dir) {
            Err(e) => println!("{:?}", e),
            _ => (),
        };

        // Index file creation
        let index_binding = &(get_current_working_dir()
            + &("/src/components/".to_owned())
            + component_type
            + "/"
            + name
            + "/index"
            + &js_extension);
        let index_path: &Path = Path::new(index_binding);
        create_component_file(index_path);

        // Style file creation
        if !has_no_style {
            let css_binding = &(get_current_working_dir()
                + &("/src/components/".to_owned())
                + component_type
                + "/"
                + name
                + "/style"
                + &css_extension);
            let style_path: &Path = Path::new(css_binding);
            create_component_file(style_path);
        }

        // Console output when component created
        println!(
            "\nCreated new component in src/components/{:?}/{}",
            comp_type, name
        );
        println!("Component format choosen: {:?}", comp_format);
        println!("Component type choosen: {:?}", comp_type);
        if !has_javascript {
            println!("Created index.tsx");
        } else {
            println!("Created index.jsx");
        }
        if has_no_style {
            println!("Component will only be an index file with no CSS.")
        }
        if has_css {
            println!("Component created with basic CSS and not SCSS.")
        }
    }
    // ---------------------------
    // If component already exists
    // ---------------------------
    else {
        println!("Component {} in {:?} already exists.", name, comp_type)
    }

    return (
        name,
        comp_format,
        comp_type,
        has_javascript,
        has_no_style,
        has_css,
    );
}
