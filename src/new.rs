use std::{
    fs::OpenOptions,
    io::Write,
};

use clap::ArgMatches;
use home::home_dir;

pub fn match_new(new_args: Option<&ArgMatches>) {
    if let Some(args) = new_args {
        // initialize required variables
        let task_name = args.get_one::<String>("task-name-input").unwrap();
        let database_name = args.get_one::<String>("database-name-input").unwrap();

        // Get the home directory
        if let Some(home) = home_dir() {
            // Create the path to the database file to be accessed
            let database_path = home
                .join("Documents")
                .join(format!("{}_todo.txt", database_name));

            // Open database file and error handling for it
            match OpenOptions::new().write(true).append(true).open(database_path) {
                Ok(mut database_file) => {
                    match database_file.write_fmt(format_args!("\n{}", task_name)) {
                        Ok(_) => println!("Wrote new task into database!"),
                        Err(e) => eprintln!("Failed to write task into database file: {:?}", e),
                    }
                }
                Err(e) => eprintln!("Failed to open database file: {:?}", e),
            }
        } else {
            eprintln!("Could not determine the home directory.");
        }
    }
}
