use std::path::Path;
use time::Timespec;
use rusqlite::Connection;
//Hopefully DB-independend SQL to store, modify and retrieve all
//comment-related actions.  Here's a short scheme overview:
//
//| tid (thread id) | id (comment id) | parent | ... | voters | remote_addr |
//+-----------------+-----------------+--------+-----+--------+-------------+
//| 1               | 1               | null   | ... | BLOB   | 127.0.0.0   |
//| 1               | 2               | 1      | ... | BLOB   | 127.0.0.0   |
//+-----------------+-----------------+--------+-----+--------+-------------+
//
//The tuple (tid, id) is unique and thus primary key.

pub struct Comment {
    tid: i32,
    id: i32,
    parent: i32,
    created: Timespec,
    modified: f32,
    mode: i32,
    remote_addr: String,
    text: String,
    author: String,
    email: String,
    website: String,
    likes: i32,
    dislikes: i32,
    voters: Option<Vec<u8>>
}

impl Comment {
    pub fn init(conn: &Connection) {

        conn.execute("CREATE TABLE IF NOT EXISTS comments (
                     tid REFERENCES threads(id),
                     id INTEGER PRIMARY KEY,
                     parent INTEGER,
                     created FLOAT NOT NULL,
                     modified FLOAT,
                     mode INTEGER,
                     remote_addr VARCHAR,
                     text VARCHAR,
                     author VARCHAR,
                     email VARCHAR,
                     website VARCHAR,
                     likes INTEGER DEFAULT 0,
                     dislikes INTEGER DEFAULT 0,
                     voters BLOB NOT NULL);", &[]).unwrap();
    }
}