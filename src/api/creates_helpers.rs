use std::{
    fs::{self, File},
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
        match create_folder(project, folder) {
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
            create_file(project.to_string() + &f.0 + f.1);
        } else {
            create_file(project.to_string() + &f.0 + extension + f.1);
        }
    }
}

fn create_folder(project: &String, folder: &str) -> std::io::Result<()> {
    fs::create_dir(project.to_string() + "/src/" + folder)?;
    Ok(())
}

fn create_file(file_path: String) {
    // Open a file in write-only mode, returns `io::Result<File>`
    match File::create(&file_path) {
        Err(e) => panic!("couldn't create {}: {}", file_path, e),
        Ok(file) => file,
    };
}
