#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rusqlite;

#[cfg(test)] mod tests;

use std::sync::Mutex;

use rocket::{Rocket, State, response::Debug};

use rusqlite::{Connection, Error};

type DbConn = Mutex<Connection>;

fn init_database(conn: &Connection) {
    conn.execute("CREATE TABLE entries (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL
                  )", &[])
        .expect("create entries table");

    conn.execute("INSERT INTO entries (id, name) VALUES ($1, $2)",
            &[&0, &"Rocketeer"])
        .expect("insert single entry into entries table");
}

#[get("/")]
fn hello(db_conn: State<DbConn>) -> Result<String, Debug<Error>>  {
    db_conn.lock()
        .expect("db connection lock")
        .query_row("SELECT name FROM entries WHERE id = 0",
                   &[], |row| { row.get(0) })
        .map_err(Debug)
}

fn rocket() -> Rocket {
    // Open a new in-memory SQLite database.
    let conn = Connection::open_in_memory().expect("in memory db");

    // Initialize the `entries` table in the in-memory database.
    init_database(&conn);

    // Have Rocket manage the database pool.
    rocket::ignite()
        .manage(Mutex::new(conn))
        .mount("/", routes![hello])
}

fn main() {
    rocket().launch();
}
