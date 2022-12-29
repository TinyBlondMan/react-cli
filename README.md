# react-cli
An attempt at creating a basic and tailored react cli to boost productivity. 
Using Rust + Clap v4.

## Disclaimer
This tool is tailored for my needs. Source files are accessible and easily customizable to suit to your needs.  Only works on Linux, I didn't try on Mac nor Windows.

Don't hesitate to change files structure, routing system, installation, etc... *(in commands, not in project of course)*.
You can also add commands I wouldn't have thought of.

## Documentation
1. Clone project
2. Run with cargo

```
cargo run -- project project_name
```

By default, will create a new React project with Vite, Typescript, and Yarn.

| Short   | Long | Command |
| ------------- |-------------|-------------|
| -h      | --help     | Displays help for main command or individual commands  |
| -j      | --javascript     | Changes default installation to JS  |
| -n      | --npm     | Changes package manager to NPM  |


Creates folders: pages, components, layout, utils, styles, data, hooks & styles.

Populates folders with various more or less useful files like basic pages and layout components.

## Current work & coming upgrades
Files are empty for now. Working on it, already working in dev. Once I'm done with project command I'll start working on component command. Component command is made to create individual component folder, index.tsx/jsx and style.scss.

I also plan to make a router command to add react-router-dom, routes folder and files accorrdingly.

After component and router commands are done I will make a binary out of this project. And include documentation to create it and add it to PATH yourself.

## Links
Clap: https://docs.rs/clap/4.0.32/clap

My starting point for CLI tools: https://blog.ediri.io/lets-build-a-cli-in-rust

The Rustlang Book: https://doc.rust-lang.org/book

## Special thanks
My dear wife-to-be who supports me every day.