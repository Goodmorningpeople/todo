use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, Write},
    path::Path,
};

use clap::ArgMatches;
use home::home_dir;

pub fn match_mark(mark_args: Option<&ArgMatches>) {
    if let Some(args) = mark_args {
        // Initialize required variables
        let database_name = args.get_one::<String>("database-name-input").unwrap();
        let task_index = args.get_one::<i32>("task-index-input").unwrap();

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
                    let mut task_found = false;

                    for entry in lines {
                        match entry {
                            Ok(line) => {
                                if counter == 0 {
                                    updated_lines.push(line.clone()); // Assuming the first line is a header
                                } else if line.is_empty() {
                                    continue;
                                } else if &(counter) == task_index {
                                    // Replace the task with its crossed-out version
                                    println!("{}", convert_to_crossed_out(line.trim()));
                                    updated_lines.push(convert_to_crossed_out(line.trim()));
                                    task_found = true;
                                } else {
                                    updated_lines.push(line.clone());
                                }
                                counter += 1;
                            }
                            Err(e) => eprintln!("Error reading line entry: {:?}", e),
                        }
                    }

                    if task_found {
                        // Write the updated lines back to the file
                        if let Err(e) = write_lines(&database_path, updated_lines) {
                            eprintln!("Failed to write to file: {:?}", e);
                        }
                    } else {
                        eprintln!("Task index {} not found.", task_index);
                    }
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

pub fn convert_to_crossed_out(text: &str) -> String {
    format!("\x1B[9m{}\x1B[0m", text)
}
