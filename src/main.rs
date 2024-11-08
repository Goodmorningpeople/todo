use clap::{command, value_parser, Arg, ArgAction, Command};
use todo::{
    add::match_add, clear::match_clear, database::match_database, list::match_list,
    mark::match_mark, remove::match_remove, sort::match_sort,
};

fn main() {
    let match_result = command!()
        .about("Simple, efficient and useful cli application for keeping track of your tasks")
        .subcommand(Command::new("database").about("database [options]: Commands to manipulate database files, database files are stored in (~/Documents/<file-name>_todo.txt)
        --new: Create a new database and assign it a name
        --list: List the current database files
        --remove: Remove a database file")
            .arg(
                Arg::new("new-option")
                    .short('n')
                    .long("new")
                    .conflicts_with_all(["list-option", "remove-option"])
            )
            .arg(
                Arg::new("list-option")
                    .short('l')
                    .long("list")
                    .action(ArgAction::SetTrue)
                    .conflicts_with_all(["new-option", "remove-option"])
            )
            .arg(
                Arg::new("remove-option")
                    .short('r')
                    .long("remove")
                    .conflicts_with_all(["list-option", "new-option"])
            )
        )
        .subcommand(Command::new("add").about("add [task-name] [database-name]: Create a new todo-list task file in a selected database file")
            .arg(
                Arg::new("task-name-input")
                    .required(true)
            )
            .arg(
                Arg::new("database-name-input")
                    .required(true)
            )
        )
        .subcommand(Command::new("list").about("list [database-name]: List all the todos stored in a selected database file")
            .arg(
                Arg::new("database-name-input")
                    .required(true)
            )
        )
        .subcommand(Command::new("mark").about("mark [task-index] [database-name]: Input the task number of a task in a selected database file to mark the task as done")
            .arg(
                Arg::new("task-index-input")
                    .required(true)
                    .value_parser(value_parser!(i32))
            )
            .arg(
                Arg::new("database-name-input")
                    .required(true)
            )
        )
        .subcommand(Command::new("remove").about("remove [task-index] [database-name]: Input the task number of a task in selected database file to remove the task")
            .arg(
                Arg::new("task-index-input")
                    .required(true)
                    .value_parser(value_parser!(i32))
        )
            .arg(
                Arg::new("database-name-input")
                    .required(true)
            )
    )
        .subcommand(Command::new("sort").about("sort [database-name]: Sorts todo-list, placing tasks marked as done at the bottom and tasks not completed yet at the top in a selected database file")
            .arg(
                Arg::new("database-name-input")
                    .required(true)
            )
    )
        .subcommand(Command::new("clear").about("clear [database-name]: Clears all tasks marked as done in a selected database file")
            .arg(
                Arg::new("database-name-input")
                    .required(true)
            )
    )
        .get_matches();

    // Match arguments with match functions
    let database_args = match_result.subcommand_matches("database");
    match_database(database_args);

    let add_args = match_result.subcommand_matches("add");
    match_add(add_args);

    let list_args = match_result.subcommand_matches("list");
    match_list(list_args);

    let mark_args = match_result.subcommand_matches("mark");
    match_mark(mark_args);

    let remove_args = match_result.subcommand_matches("remove");
    match_remove(remove_args);

    let sort_args = match_result.subcommand_matches("sort");
    match_sort(sort_args);

    let clear_args = match_result.subcommand_matches("clear");
    match_clear(clear_args);
}
