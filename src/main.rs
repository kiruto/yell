extern crate rusqlite;
extern crate time;

use rusqlite::Connection;
use std::path::Path;

mod comment;
mod thread;
mod preference;

const DB_FILE_PATH: &'static str = "./yell.sqlite";

fn main() {
    let db_file = Path::new(DB_FILE_PATH);
    let conn = Connection::open(db_file).unwrap();
    comment::Comment::init(&conn);
    thread::Thread::init(&conn);
    preference::Preference::init(&conn);
}
