use rusqlite::Connection;

pub struct ThreadPool<'a> {
    conn: &'a Connection
}

pub struct Thread {
    pub id: i32,
    pub uri: String,
    pub title: String,
}

impl<'a> ThreadPool<'a> {
    pub fn new(conn: &'a Connection) -> ThreadPool {
        conn.execute("CREATE TABLE IF NOT EXISTS threads (id INTEGER PRIMARY KEY, uri VARCHAR(256)
                      UNIQUE, title VARCHAR(256))", &[]);
        ThreadPool { conn: conn }
    }

    pub fn insert(&self, uri: &str, title: &str) -> i32 {
        self.conn.execute("INSERT INTO threads (uri, title) VALUES (?, ?)", &[&uri, &title]).unwrap()
    }

    pub fn get(&self, uri: &str) -> Result<Thread, ()> {
        let mut stmt = self.conn.prepare("SELECT * FROM threads WHERE uri=?1").unwrap();
        let thread_iter = stmt.query_map(&[&uri], |row| {
            Thread {
                id: row.get(0),
                uri: row.get(1),
                title: row.get(2)
            }
        }).unwrap();

        for t in thread_iter {
            return Ok(t.unwrap());
        }
        Err(())
    }
}