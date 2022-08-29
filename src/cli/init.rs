use std::{fs, io::ErrorKind};

pub fn init() {
    let current_directory = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(error) => {
            eprintln!("Could not retrieve current directory due to error: {:?}", error);
            std::process::exit(1);
        },
    };

    match fs::create_dir(format!("{}/.svcs", current_directory.to_string_lossy())) {
        Ok(..) => println!("Initialized empty project under {}", current_directory.to_string_lossy()),
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => {
                eprintln!("Could not initialize project because it's already initialized.");
                std::process::exit(1);
            },
            unkown => {
                eprintln!("Could not initialize project due to error: {:?}", unkown);
                std::process::exit(1);
            },
        },
    };
}
