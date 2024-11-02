use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use clap::ArgMatches;
use home::home_dir;

pub fn match_list(list_args: Option<&ArgMatches>) {
    if let Some(args) = list_args {
        // initialize required variables
        let database_name = args.get_one::<String>("database-name-input").unwrap();

        // Get the home directory
        if let Some(home) = home_dir() {
            // Create the path to the database file to be accessed
            let database_path = home
                .join("Documents")
                .join(format!("{}_todo.txt", database_name));

            // Open database file and error handling for it
            match read_lines(database_path) {
                Ok(mut lines) => {
                    let mut counter = 0;
                    while let Some(entry) = lines.next() {
                        match entry {
                            Ok(line) => {
                                if counter == 0 {
                                    counter += 1;
                                    continue;
                                }
                                if line.is_empty() {
                                    continue;
                                }
                                counter += 1;
                                println!("{}. {}", counter - 1, line.trim())

                            }
                            Err(e) => eprintln!("Error reading line entry: {:?}", e),
                        }
                    }
                    if counter == 1 {
                        println!("You have no tasks");
                    }
                }
                Err(e) => eprintln!("Failed to read file content:{:?}", e),
            }
        } else {
            eprintln!("Could not determine the home directory.");
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
