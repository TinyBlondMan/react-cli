use std::process::Command;

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
    Command::new("mkdir")
        .arg(project.to_string() + "/src/pages")
        .arg(project.to_string() + "/src/components")
        .arg(project.to_string() + "/src/components/common")
        .arg(project.to_string() + "/src/components/forms")
        .arg(project.to_string() + "/src/components/sections")
        // .arg(project.to_string() + "/src/routes")
        .arg(project.to_string() + "/src/layout")
        .arg(project.to_string() + "/src/hooks")
        .arg(project.to_string() + "/src/data")
        .arg(project.to_string() + "/src/utils")
        .arg(project.to_string() + "/src/config")
        .arg(project.to_string() + "/src/styles")
        .output()
        .expect("Folder structure creation failed at some point");
}

pub fn create_src_files(project: &String, extension: &String) {
    Command::new("touch")
        // .arg(project.to_string() + "/src/routes/links" + extension)
        // .arg(project.to_string() + "/src/routes/router" + extension + "x")
        // .arg(project.to_string() + "/src/routes/pages" + extension + "x")
        .arg(project.to_string() + "/src/layout/navbar" + extension + "x")
        .arg(project.to_string() + "/src/layout/navbar.scss")
        .arg(project.to_string() + "/src/layout/footer" + extension + "x")
        .arg(project.to_string() + "/src/layout/footer.scss")
        .arg(project.to_string() + "/src/pages/Home" + extension + "x")
        .arg(project.to_string() + "/src/pages/Mentions" + extension + "x")
        .arg(project.to_string() + "/src/pages/Blog" + extension + "x")
        .arg(project.to_string() + "/src/pages/BlogDetail" + extension + "x")
        .arg(project.to_string() + "/src/pages/About" + extension + "x")
        .arg(project.to_string() + "/src/hooks/useWindowSize" + extension)
        .arg(project.to_string() + "/src/styles/variables.scss")
        .arg(project.to_string() + "/src/config/index" + extension)
        .arg(project.to_string() + "/src/utils/functions" + extension)
        .output()
        .expect("Files population failed at some point");
}
