use sqlite;

fn list(connection: sqlite::Connection) {
    println!("List action");
    connection
        .iterate("SELECT * FROM todos", |todos| {
            println!("{:?}", todos);
            true
        })
        .unwrap();
}

fn main() {
    let action = std::env::args().nth(1).expect("No action provided");
    let connection = sqlite::open("todo.db").unwrap();
    connection
        .execute(
            "
        CREATE TABLE IF NOT EXISTS todos (id INTEGER, name TEXT);
        INSERT INTO todos VALUES (1, 'First');
        INSERT INTO todos VALUES (2, 'Second');
        ",
        )
        .unwrap();

    match action.as_str() {
        "list" => list(connection),
        _ => println!("Wrong action"),
    };
}
