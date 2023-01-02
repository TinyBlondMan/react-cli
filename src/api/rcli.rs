use super::{
    creates_helpers::{create_folder_structure, create_src_files, init_vite_project},
    files_contents::{inject_files, return_files},
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

pub fn create_component<'a, 'b, 'c>(
    name: &'a String,
    comp_format: &'b String,
    comp_type: &'c String,
    has_javascript: bool,
    has_no_style: bool,
    has_css: bool,
) -> (&'a String, &'b String, &'c String, bool, bool, bool) {
    // Defines files extension
    // let extension: String = if !has_javascript {
    //     String::from(".ts")
    // } else {
    //     String::from(".js")
    // };
    return (
        name,
        comp_format,
        comp_type,
        has_javascript,
        has_no_style,
        has_css,
    );
}
