use std::{
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf}
};

use clap::builder::Str;
use heck::ToTitleCase;

pub struct Command {
    pub name: String,
    pub description: String,
    pub category: String,
    pub path: String
}
#[derive(serde::Deserialize)]
struct CommandConfig {
    pub name: String,
    pub category: String,
    pub description: String,
}

impl Command {
    pub fn from_path(path: &str) -> Vec<Command> {
        // Get all the subdirectories of the commands directory
        let directories = fs::read_dir(path)
        .expect(format!("Could not read {}", path).as_str())
        .filter_map(Result::ok)
        .filter_map(|f| match f.metadata() {
            Ok(_) => Some(f),
            Err(_) => None
        })
        .filter(|f| f.metadata().unwrap().is_dir())
        .collect::<Vec<DirEntry>>();

        let mut commands = vec![];

        for folder in directories.iter() {
            // See if we can find a config.toml in the directory
            let config = get_command_config(&folder.path());
            let mut command_path = folder.path().clone();
            command_path.push("command.sh");

            // If we have a config we will save the data to the command list
            if let Some(cfg) = config {
                commands.push(Command {
                    name: cfg.name,
                    description: cfg.description,
                    category: cfg.category,
                    path: folder.path().display().to_string()
                })
            } else {
                // If we did not find a config, try to derive the command name from
                // the folder name and set description and category to empty strings.
                let name = command_name_from_path(folder);

                commands.push(Command {
                    name: name,
                    description: String::new(),
                    category: String::new(),
                    path: command_path.to_owned().into_os_string().into_string().unwrap()
                })
            }
        }

        commands
    }
}

fn command_name_from_path(path: &DirEntry) -> String {
    let name = path
        .file_name()
        .to_str()
        .unwrap_or_default()
        .replace('-', &" ")
        .to_title_case()
        .to_owned();

    name
}

fn get_command_config(path: &PathBuf) -> Option<CommandConfig> {
    let mut p = path.clone();
    p.push("info.toml");
    
    let content: String;
    match fs::read_to_string(p) {
        Ok(c) => content = c,
        Err(_) => return None, 
    }
    
    let config: Result<CommandConfig, toml::de::Error> = toml::from_str(String::as_str(&content));

    match config {
        Ok(c) => return Some(c),
        Err(e) => panic!("{}", e)
    }
}