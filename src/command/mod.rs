use std::{
    fs,
    io,
    path::{Path, PathBuf}
};

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
    pub fn list(path: &String) -> Vec<Command> {
        let folders = get_command_folders(&path);
        let mut commands = vec![];

        for folder in folders.iter() {
            let config = get_command_config(&folder.to_string());
            let mut command_path = PathBuf::from(folder);
            command_path.push("command.sh");

            if let Some(cfg) = config {
                commands.push(Command {
                    name: cfg.name,
                    description: cfg.description,
                    category: cfg.category,
                    path: folder.to_string()
                })
            } else {
                let name = PathBuf::from(folder)
                            .file_name()
                            .unwrap_or_default()
                            .to_os_string()
                            .to_str()
                            .unwrap_or_default()
                            .replace('-', &" ")
                            .to_title_case()
                            .to_owned();

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

fn get_command_config(path: &String) -> Option<CommandConfig> {
    let mut p = PathBuf::from(path.to_owned());
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

fn get_command_folders(path: &String) -> Vec<String> {
    let files = fs::read_dir(path).expect(format!("Could not read {}", path).as_str());
    let folders = files
        .filter_map(Result::ok)
        .filter_map(|f| match f.metadata() {
            Ok(_) => Some(f),
            Err(_) => None
        })
        .filter(|f| f.metadata().unwrap().is_dir())
        .filter_map(|f| match f.path().into_os_string().into_string() {
            Ok(s) => Some(s),
            Err(_) => None
        })
        .collect::<Vec<String>>();
        
    

    folders
}