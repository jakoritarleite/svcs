use std::{fs, process};


#[derive(Debug)]
pub struct NodeConfig {
    pub nodes_folder: String,
}

impl NodeConfig {
    pub fn create() {
       match fs::create_dir(".svcs/nodes") {
            Ok(..) => (),
            Err(error) => {
                eprintln!("Could not create nodes config folder due to error: {:?}", error);
                process::exit(1);
            }
        }
    }
}
