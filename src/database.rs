use clap::ArgMatches;
use home::home_dir;
use std::{fs::File, io::Write};

pub fn match_database(database_args: Option<&ArgMatches>) {
    if let Some(args) = database_args {
        // Initialize option variables
        let file_name = args.get_one::<String>("file-name-input");

        // Get the home directory
        if let Some(home) = home_dir() {
            // if file name option is used
            if let Some(file_name) = file_name {
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
            } else {
                eprintln!("Could not determine the home directory.");
            }
        }
    }
}
