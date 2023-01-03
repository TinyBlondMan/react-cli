use std::fs::File;
use std::io::{Error, Write};

use super::contents_init::{
    return_config_js, return_config_ts, return_index_css, return_links_js, return_links_ts,
    return_style_variables, return_usewindowsize_js, return_usewindowsize_ts,
};

/// Chooses between JS and TS versions
/// extension: ".ts" | ".js"
/// ts_func: function returning the string needed for ts file (ends with _ts)
/// js_func: function returning the string needed for js file (ends with _js)
fn return_file_content(extension: &String, ts_func: String, js_func: String) -> String {
    if extension == ".ts" {
        return ts_func;
    } else {
        return js_func;
    }
}

pub struct SrcFile {
    path: String,
    name: String,
    injection: String,
}

pub fn return_files(project_name: &String, extension: &String) -> Vec<SrcFile> {
    let files: Vec<SrcFile> = vec![
        SrcFile {
            path: project_name.to_string() + "/src/routes/",
            name: String::from("links") + extension,
            injection: return_file_content(extension, return_links_ts(), return_links_js())
                .to_string(),
        },
        SrcFile {
            path: project_name.to_string() + "/src/hooks/",
            name: String::from("useWindowSize") + extension,
            injection: return_file_content(
                extension,
                return_usewindowsize_ts(),
                return_usewindowsize_js(),
            )
            .to_string(),
        },
        SrcFile {
            path: project_name.to_string() + "/src/config/",
            name: String::from("index") + extension,
            injection: return_file_content(extension, return_config_ts(), return_config_js())
                .to_string(),
        },
        SrcFile {
            path: project_name.to_string() + "/src/styles/",
            name: String::from("variables.scss"),
            injection: return_style_variables().to_string(),
        },
        SrcFile {
            path: project_name.to_string() + "/src/",
            name: String::from("index.css"),
            injection: return_index_css().to_string(),
        },
    ];

    return files;
}

fn write_in_file(dir: String, name: String, text: &str) -> Result<(), Error> {
    let file_name = format!("{}{}", dir, name);
    let mut file = File::create(file_name)?;
    writeln!(file, "{}", text)?; // writing using the macro 'writeln!'
    Ok(())
}

pub fn inject_files(files: Vec<SrcFile>) {
    for f in files {
        // Writing for each file
        match write_in_file(f.path, f.name, &f.injection) {
            Err(e) => println!("{:?}", e),
            _ => (),
        }
    }
}
