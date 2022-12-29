use std::fs::File;
use std::io::{Error, Write};

/// Returns string containing basic routes for project
/// TS Version
fn return_links_ts() -> String {
    return String::from(
        "// MAIN ROUTES
export const HOME_LINK: string = '/';
export const ABOUT_LINK: string = '/à-propos';
export const BLOG_LINK: string = '/blog';

// NESTED ROUTES
export const SINGLE_BLOG_LINK: string = '/blog';

// LEGALS
export const MENTIONS_LEGALES_LINK: string = '/mentions-légales';",
    );
}

/// Returns string containing basic routes for project
/// JS Version
fn return_links_js() -> String {
    return String::from(
        "// MAIN ROUTES
export const HOME_LINK = '/';
export const ABOUT_LINK = '/à-propos';
export const BLOG_LINK = '/blog';

// NESTED ROUTES
export const SINGLE_BLOG_LINK = '/blog';

// LEGALS
export const MENTIONS_LEGALES_LINK = '/mentions-légales';",
    );
}

/// Returns string containing useWindowSize hook
/// TS Version
fn return_usewindowsize_ts() -> String {
    return String::from(
        "import { useState, useEffect } from 'react';

interface IWindow {
    width: number | undefined;
    height: number | undefined;
}

// Hook
function useWindowSize(): IWindow {
    // Initialize state with undefined width/height so server and client renders match
    // Learn more here: https://joshwcomeau.com/react/the-perils-of-rehydration/
    const [windowSize, setWindowSize] = useState<IWindow>({
        width: undefined,
        height: undefined,
    });
    useEffect(() => {
        // Handler to call on window resize
        function handleResize() {
            // Set window width/height to state
            setWindowSize({
                width: window.innerWidth,
                height: window.innerHeight,
            });
        }
        // Add event listener
        window.addEventListener('resize', handleResize);
        // Call handler right away so state gets updated with initial window size
        handleResize();
        // Remove event listener on cleanup
        return () => window.removeEventListener('resize', handleResize);
    }, []); // Empty array ensures that effect is only run on mount
    return windowSize;
}

export default useWindowSize;",
    );
}

/// Returns string containing useWindowSize hook
/// JS Version
fn return_usewindowsize_js() -> String {
    return String::from(
        "import { useState, useEffect } from 'react';

// Hook
function useWindowSize() {
    const [windowSize, setWindowSize] = useState({
        width: undefined,
        height: undefined,
    });
    useEffect(() => {
        // Handler to call on window resize
        function handleResize() {
            // Set window width/height to state
            setWindowSize({
                width: window.innerWidth,
                height: window.innerHeight,
            });
        }
        // Add event listener
        window.addEventListener('resize', handleResize);
        // Call handler right away so state gets updated with initial window size
        handleResize();
        // Remove event listener on cleanup
        return () => window.removeEventListener('resize', handleResize);
    }, []); // Empty array ensures that effect is only run on mount
    return windowSize;
}
    
export default useWindowSize;",
    );
}

// Returns SCSS version on my style config for containers, fonts &colors
fn return_style_variables() -> String {
    return String::from(
        "/*
We put the ant default breackpoint values ​​in the variables 
to be able to use them easily in our components style files 
*/

// smallest break point is $screen-md
// $screen-xs: 480px;
$screen-xs: 480px;
$screen-xs-min: $screen-xs;
// 👆 Extra small screen / phone

// 👇 Small screen / tablet
// $screen-sm: 576px;
$screen-sm: 576px;
$screen-sm-min: $screen-sm;

// Medium screen / desktop
// $screen-md: 768px;
$screen-md: 766px;
$screen-md-min: $screen-md;

// Large screen / wide desktop
$screen-lg: 992px;
$screen-lg-min: $screen-lg;

// Extra large screen / full hd
$screen-xl: 1200px;
$screen-xl-min: $screen-xl;

// Extra extra large screen / large desktop
$screen-xxl: 1600px;
$screen-xxl-min: $screen-xxl;

// provides a maximum
$screen-xs-max: (
    $screen-sm-min - 1px
);
$screen-sm-max: (
    $screen-md-min - 1px
);
$screen-md-max: (
    $screen-lg-min - 1px
);
$screen-lg-max: (
    $screen-xl-min - 1px
);
$screen-xl-max: (
    $screen-xxl-min - 1px
);

// colors
$primaryColor: #071448;
$secondaryColor: #BC002E;

// fonts
$primaryFont: 'Poppins', sans-serif;
$secondaryFont: 'Montserrat', sans-serif;",
    );
}

/// Returns string for config variables, screen sizes, etc...
/// TS Version
fn return_config_ts() -> String {
    return String::from(
        "export const MOBILE_SIZE: number = 768;
export const TABLET_SIZE: number = 992;
export const DESKTOP_SIZE: number = 1200;
export const BIG_DESKTOP_SIZE: number = 1366;

// Switch between those two when dev or local
export const BASE_URL: string = 'http://localhost:5173/';
// export const BASE_URL: string = 'https://www.mywebsite.com';",
    );
}

/// Returns string for config variables, screen sizes, etc...
/// JS Version
fn return_config_js() -> String {
    return String::from(
        "export const MOBILE_SIZE = 768;
export const TABLET_SIZE = 992;
export const DESKTOP_SIZE = 1200;
export const BIG_DESKTOP_SIZE = 1366;

// Switch between those two when dev or local
export const BASE_URL: string = 'http://localhost:5173/';
// export const BASE_URL: string = 'https://www.mywebsite.com';",
    );
}

/// Returns string to replace src/index.css content with clean init
fn return_index_css() -> String {
    return String::from(
        "*, ::before, ::after {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
}",
    );
}

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
    pub path: String,
    pub name: String,
    pub injection: String,
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
