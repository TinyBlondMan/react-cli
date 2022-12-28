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
pub struct Reverse {
    /// The string to reverse
    pub string: Option<String>,
}

#[derive(Args)]
pub struct Inspect {
    /// The string to inspect
    pub string: Option<String>,

    #[arg(short = 'd', long = "digits")]
    pub only_digits: bool,
}
