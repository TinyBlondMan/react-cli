# react-cli ![](https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB) ![](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white) ![](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)


An attempt at creating a basic and tailored react cli to boost productivity. 
Using Rust + Clap v4.

## :rotating_light: Disclaimer
This tool is tailored for my needs. Source files are accessible and easily 
customizable to suit to your needs.  Only works on Linux, I didn't try on Mac 
nor Windows.

This is basically my first side project in Rust *(I'm initially a front 
React dev, but I like this language)*. Thus, the code is probably ugly 
for Rust developpers, please be kind. Any advice is greatly appreciated.

Don't hesitate to change files structure, routing system, installation, etc...
 *(in commands, not in project of course)*.
You can also add commands I wouldn't have thought of.
## :computer: Installation

:warning: For MacOS and Windows, it probably won't work due to terminal 
not being the same. :warning:

To make your own binary, clone repo and compile with Cargo.

```
git clone https://github.com/TinyBlondMan/react-cli.git
cd react-cli
cargo run -- --version
```

Otherwise you can copy the binary from the repo and add it to PATH.

    
## :notebook: Documentation

### Project

```
### From your general working directory (ex: home/projects/)
react-cli project project_name
```

Default: will create a new React project with Vite, Typescript, and Yarn.

| Short   | Long | Command |
| ------------- |-------------|-------------|
| -h| --help     | Help for params  |
| -j| --javascript     | Changes installation to JS  |
| -n| --npm     | Changes package manager to NPM  |


Also creates folders: pages, components, layout, utils, styles, data, 
hooks & styles. It then populates folders with various more or less useful 
files like basic pages and layout components.

### Component

```
### From inside project directory (ex: home/projects/todolist)
react-cli component ComponentName
```

Default: will create a component in /src/components/common. Component has 
an index.tsx and a style.scss files. Index file is a Typescript Export Default 
React Pure Function Component.

| Short| Long | Command |
| ------------- |-------------|-------------|
| -h | --help     | Help for params.  |
| -f | --format     | Changes component function type (see --help for options).  |
| -t | --type     | Changes component type directory. Can be anything.  |
| -j | --javascript     | Changes index from TSX to JSX.  |
| -n | --no-style     | Removes style file from component folder  |
| -c | --css     | Creates style file with CSS instead of SCSS.  |

:warning: :warning: For now, style file import in index file is not handled :warning: :warning:

## :bulb: Current work & coming upgrades

I should probably publish a git repo of the result you have when launching 
this command.

For now there are absolutely no security except those imposed by Rust.
Component command can launch anywhere so be careful where you are when you
fire it. I plan to add securities for directories. This should be the next 
step.

I also plan to make a router command to add react-router-dom, routes 
folder and files accorrdingly with a few basic and nested routes.

In later stages of this project I would like to refactor the code with more
structs, options, and things like that for readability, but my level in 
Rust is not high enough for now and I work 10 hours a day in JS... Thus, not
for today.

## :link: Links
[Clap Documentation for the version used in this project (4.0.32)](https://docs.rs/clap/4.0.32/clap)

[My starting point for CLI tools dev in Rust](https://blog.ediri.io/lets-build-a-cli-in-rust)

[The Rustlang Book](https://doc.rust-lang.org/book)

## :bookmark_tabs: License

[MIT License](https://choosealicense.com/licenses/mit/)

## :tada: Special thanks
My dear wife-to-be who supports me every day. And thank you to my friend Ulrich who 
often helps me in realizing how good developers can be with enough motivation.