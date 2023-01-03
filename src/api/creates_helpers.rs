use std::{
    env,
    fs::{self, File},
    path::Path,
    process::Command,
};

// Launches following command in terminal
// <yarn/npm> create vite <project_name> --template <react/react-ts>
pub fn init_vite_project(package: &String, project: &String, language: &String) {
    Command::new(&package)
        .arg("create")
        .arg("vite")
        .arg(project)
        .arg("--template")
        .arg(language)
        .output()
        .expect("Project init command failed to start");
}

pub fn create_folder_structure(project: &String) {
    // Main folder creation list is here and nowhere else
    let folders: Vec<&str> = vec![
        "pages",
        "components",
        "components/common",
        "components/forms",
        "components/sections",
        "layout",
        "hooks",
        "data",
        "utils",
        "config",
        "styles",
        "api",
    ];
    for folder in folders {
        match create_folder_for_init(project, folder) {
            Err(e) => println!("{:?}", e),
            _ => (),
        };
    }
}

pub fn create_src_files(project: &String, extension: &String) {
    // Main source files creation list is here and nowhere else
    let files: Vec<(&str, &str)> = vec![
        ("/src/layout/navbar", "x"),
        ("/src/layout/navbar", ".scss"),
        ("/src/layout/footer", "x"),
        ("/src/layout/footer", ".scss"),
        ("/src/pages/Home", "x"),
        ("/src/pages/Mentions", "x"),
        ("/src/pages/Blog", "x"),
        ("/src/pages/BlogDetail", "x"),
        ("/src/pages/About", "x"),
        ("/src/hooks/useWindowSize", ""),
        ("/src/styles/variables", ".scss"),
        ("/src/config/index", ""),
        ("/src/utils/functions", ""),
    ];

    for f in files {
        if f.1 == ".scss" {
            create_file_for_init(project.to_string() + &f.0 + f.1);
        } else {
            create_file_for_init(project.to_string() + &f.0 + extension + f.1);
        }
    }
}

pub fn create_folder_for_init(project: &String, folder: &str) -> std::io::Result<()> {
    fs::create_dir(project.to_string() + "/src/" + folder)?;
    Ok(())
}

fn create_file_for_init(file_path: String) {
    // Open a file in write-only mode, returns `io::Result<File>`
    match File::create(&file_path) {
        Err(e) => panic!("couldn't create {}: {}", file_path, e),
        Ok(file) => file,
    };
}

pub fn create_component_file(path: &Path) {
    match File::create(&path) {
        Err(e) => panic!("Couldn't create component file: {}", e),
        Ok(file) => file,
    };
}

pub fn get_current_working_dir() -> String {
    let path_buf = env::current_dir();
    let result_current_dir = match path_buf {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let current_dir = result_current_dir.display().to_string();
    current_dir
}
