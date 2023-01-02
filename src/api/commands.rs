use clap::Args;

#[derive(Args)]
pub struct Project {
    /// Project name
    pub project_name: Option<String>,

    /// TS by default, changes to JS if true
    #[arg(short = 'j', long = "javascript")]
    pub with_javascript: bool,

    /// YARN by default, changes to NPM if true
    #[arg(short = 'n', long = "npm")]
    pub with_npm: bool,
}

#[derive(Args)]
pub struct Router {
    /// Routes folder name
    pub routes_folder_name: Option<String>,

    /// TS by default, changes to JS if true
    #[arg(short = 'j', long = "javascript")]
    pub with_javascript: bool,

    /// YARN by default, changes to NPM if true
    #[arg(short = 'n', long = "npm")]
    pub with_npm: bool,

    /// Adds some basic links and <routes_folder>/links.ts|js file
    #[arg(short = 'l', long = "links")]
    pub with_links: bool,

    /// Adds some basic implementation of a router in routes folder
    #[arg(short = 'r', long = "routing")]
    pub with_routing_system: bool,
}

#[derive(Args)]
pub struct Component {
    /// Component name
    pub component_name: Option<String>,

    /// Fills component with const, arrow function, class component, etc...
    ///
    /// Default value:
    /// tsdrpfc: Typescript Export Default React Pure Function Component;
    /// Other values:
    /// rafce: React Arrow Function Export component;
    /// rfce: React Functional Export component;
    /// rce: React Class Export component;
    /// rcredux: React Class component with Redux;
    /// rfceredux: React Functional Export component with Redux;
    /// tsrcredux: Typescript React Class component with Redux;
    #[arg(short = 'f', long = "format")]
    pub comp_format: String,

    /// Defines if component is common, form, section, etc...
    ///
    /// Default value: common;
    /// Other values: forms, sections, pages (in pages folder)
    #[arg(short = 't', long = "type")]
    pub comp_type: String,

    /// TS by default, changes to JS if true
    #[arg(short = 'j', long = "javascript")]
    pub with_javascript: bool,

    /// Prevents from creating stylesheet
    #[arg(short = 'n', long = "no-style")]
    pub without_style: bool,

    /// SCSS by default, changes to CSS if true
    #[arg(short = 'c', long = "css")]
    pub with_css: bool,
}
