use rusqlite::Connection;

pub struct Preference {
    key: String,
    value: String
}

impl Preference {
    fn init(conn: &Connection) {
        conn.execute("CREATE TABLE IF NOT EXISTS preferences
                      (key VARCHAR PRIMARY KEY, value VARCHAR);")
    }
}