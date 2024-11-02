use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, Write},
    path::Path,
};

use clap::ArgMatches;
use home::home_dir;

pub fn match_clear(clear_args: Option<&ArgMatches>) {
    if let Some(args) = clear_args {
        // Initialize required variables
        let database_name = args.get_one::<String>("database-name-input").unwrap();

        // Get the home directory
        if let Some(home) = home_dir() {
            // Create the path to the database file to be accessed
            let database_path = home
                .join("Documents")
                .join(format!("{}_todo.txt", database_name));

            // Open database file and error handling for it
            match read_lines(&database_path) {
                Ok(lines) => {
                    let mut updated_lines = Vec::new();
                    let mut counter = 0;

                    for entry in lines {
                        match entry {
                            Ok(line) => {
                                if counter == 0 {
                                    updated_lines.push(line.clone()); // Assuming the first line is a header
                                    counter += 1;
                                } else if line.is_empty() {
                                    continue;
                                } else {
                                    if !line.contains("\x1B[9m") {
                                        updated_lines.push(line.clone());
                                    }
                                }
                                counter += 1;
                            }
                            Err(e) => eprintln!("Error reading line entry: {:?}", e),
                        }
                    }
                    if counter == 1 {
                        println!("You have no tasks");
                    }

                    // Write the updated lines back to the file
                    if let Err(e) = write_lines(&database_path, updated_lines) {
                        eprintln!("Failed to write to file: {:?}", e);
                    }
                    println!("Cleared all tasks marked as done!");
                }
                Err(e) => eprintln!("Failed to read file content: {:?}", e),
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

fn write_lines<P>(filename: P, lines: Vec<String>) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(filename)?;

    for line in lines {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}