use sqlite::Connection;
use std::{env, path::Path};

#[derive(Debug)]
struct Todo {
    id: String,
    name: String,
    done: bool,
}

impl Default for Todo {
    fn default() -> Self {
        Todo {
            id: String::from("0"),
            name: String::from("0"),
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
            CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY, name TEXT, done BOOLEAN);
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
                    "id" => todo.id = String::from(value.unwrap()),
                    "name" => todo.name = String::from(value.unwrap()),
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

fn add(connection: &Connection, name: String) {
    connection
        .execute(format!(
            "INSERT INTO todos (name, done) VALUES ('{}', false)",
            name
        ))
        .unwrap();
    println!("The to-do has been added!");
}

fn done(connection: &Connection, id: u128) {
    connection
        .execute(format!("UPDATE todos SET done='true' WHERE id={}", id))
        .unwrap();
    println!("The to-do is done!");
}

fn delete(connection: &Connection, id: u128) {
    connection
        .execute(format!("DELETE FROM todos WHERE id={}", id))
        .unwrap();
    println!("The to-do has been deleted!");
}

fn main() {
    let action: String = env::args().nth(1).expect("No action provided");
    let connection: Connection = get_database_connection();
    create_todos_table(&connection);

    match action.as_str() {
        "list" => list(&connection),
        "add" => {
            let name: String = env::args().nth(2).expect("No name provided");
            add(&connection, name);
        }
        "done" => {
            let id: u128 = env::args()
                .nth(2)
                .expect("ID not provided")
                .parse()
                .unwrap();
            done(&connection, id);
        }
        "delete" => {
            let id: u128 = env::args()
                .nth(2)
                .expect("ID not provided")
                .parse()
                .unwrap();
            delete(&connection, id);
        }
        _ => println!("Wrong action"),
    };
}
