use clap::ArgMatches;
use home::home_dir;
use std::{
    fs::{self, File},
    io::Write,
};

pub fn match_database(database_args: Option<&ArgMatches>) {
    if let Some(args) = database_args {
        // Initialize option variables
        let new = args.get_one::<String>("new-option");
        let list = args.get_flag("list-option");
        let remove = args.get_one::<String>("remove-option");

        // Get the home directory
        if let Some(home) = home_dir() {
            // if file name option is used
            if let Some(file_name) = new {
                // Create the path to the new file
                let file_path = home
                    .join("Documents")
                    .join(format!("{}_todo.txt", file_name));

                // Create new file and error handling for it
                match File::create(&file_path) {
                    Ok(mut file) => {
                        println!("Created {}_todo.txt in ~/Documents!", file_name);
                        match file.write_all(b"TODO-LIST FILE, NOT TO BE TOUCHED") {
                            Ok(_) => {}
                            Err(e) => {
                                eprintln!("Failed to print validating statement to file: {:?}", e)
                            }
                        }
                    }
                    Err(e) => eprintln!(
                        "Failed to create {}_todo.txt in ~/Documents: {:?}",
                        file_name, e
                    ),
                }
            } else if list {
                match fs::read_dir(home.join("Documents")) {
                    Ok(paths) => {
                        let mut counter = 0;
                        for entry in paths {
                            match entry {
                                Ok(path) => {
                                    let name = path.path().display().to_string();
                                    if name.ends_with("_todo.txt") {
                                        counter += 1;
                                        println!(
                                            "{}",
                                            name.strip_suffix("_todo.txt")
                                                .unwrap()
                                                .strip_prefix(
                                                    home.join("Documents/").to_str().unwrap()
                                                )
                                                .unwrap()
                                        )
                                    }
                                }
                                Err(e) => eprintln!("Failed to get path entry: {:?}", e),
                            }
                        }
                        if counter == 0 {
                            println!("You have no database files");
                        }
                    }
                    Err(e) => eprintln!("Failed to read ~/Documents directory: {:?}", e),
                }
            }
            if let Some(database_name) = remove {
                match fs::read_dir(home.join("Documents")) {
                    Ok(paths) => {
                        for entry in paths {
                            match entry {
                                Ok(path) => {
                                    let name = path.path().strip_prefix(home.join("Documents/")).unwrap().display().to_string();
                                    if name.contains(database_name) && name.contains("_todo.txt") {
                                        match fs::remove_file(
                                            home.join("Documents/")
                                                .join(name)
                                        ) {
                                            Ok(_) => println!("Removed database file!"),
                                            Err(e) => {
                                                eprintln!("Failed to remove database file: {:?}", e)
                                            }
                                        }
                                    }
                                }
                                Err(e) => eprintln!("Failed to get path entry: {:?}", e),
                            }
                        }
                    }
                    Err(e) => eprintln!("Failed to read ~/Documents directory: {:?}", e),
                }
            }
        } else {
            eprintln!("Could not determine the home directory.");
        }
    }
}