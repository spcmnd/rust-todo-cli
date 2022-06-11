use sqlite::Connection;
use std::{env, path::Path};

struct Todo<'a> {
    id: &'a str,
    name: &'a str,
    done: bool,
}

impl Default for Todo<'_> {
    fn default() -> Self {
        Todo {
            id: "0",
            name: "0",
            done: false,
        }
    }
}

fn check_database() {
    match Path::new("todo.db").exists() {
        true => println!("To-Do database does exist, loading it..."),
        false => println!("To-Do database does not exist, creating it..."),
    }
}

fn get_database_connection() -> Connection {
    check_database();
    println!("---");
    sqlite::open("todo.db").unwrap()
}

fn create_todos_table(connection: &Connection) {
    connection
        .execute(
            "
            CREATE TABLE IF NOT EXISTS todos (id INTEGER, name TEXT, done BOOLEAN);
            ",
        )
        .unwrap();
}

fn list(connection: &Connection) {
    println!("List action");
    println!("---");
    connection
        .iterate("SELECT * FROM todos", |todo_row| {
            let mut todo: Todo = Todo::default();

            for &(column, value) in todo_row.iter() {
                match column {
                    "id" => todo.id = value.unwrap(),
                    "name" => todo.name = value.unwrap(),
                    "done" => {
                        todo.done = match value.unwrap() {
                            "true" => true,
                            "false" => false,
                            _ => false,
                        }
                    }
                    _ => (),
                };
            }

            println!(
                "[{}] {} - {}",
                todo.id,
                todo.name,
                match todo.done {
                    true => "Done",
                    false => "Not done",
                }
            );

            true
        })
        .unwrap();
}

fn main() {
    let action = env::args().nth(1).expect("No action provided");
    let connection: Connection = get_database_connection();
    create_todos_table(&connection);

    match action.as_str() {
        "list" => list(&connection),
        _ => println!("Wrong action"),
    };
}
