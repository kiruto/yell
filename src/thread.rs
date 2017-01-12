use rusqlite::Connection;

pub struct Thread {
    id: i32,
    uri: String,
    title: String
}

impl Thread {
    fn init(conn: &Connection) {
        conn.execute("CREATE TABLE IF NOT EXISTS threads (id INTEGER PRIMARY KEY, uri VARCHAR(256)
                      UNIQUE, title VARCHAR(256))");
    }
}