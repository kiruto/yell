use rusqlite::Connection;
use rand;

pub struct PreferencePool<'a> {
    conn: &'a Connection
}

pub struct Preference {
    key: String,
    value: String
}

impl<'a> PreferencePool<'a> {
    fn gen_session_key() -> String {
        let mut result = String::new();
        for i in 0..3 {
            let num = rand::random::<u64>();
            let r = format!("{:x}", &num);
            result += r.as_str();
        }
        result
    }

    pub fn new(conn: &'a Connection) -> PreferencePool {
        conn.execute("CREATE TABLE IF NOT EXISTS preferences
                      (key VARCHAR PRIMARY KEY, value VARCHAR);", &[]).unwrap();
        PreferencePool::gen_session_key();
        PreferencePool { conn: conn }
    }

    pub fn get(&self, key: &str, default: Option<&str>) {
        let mut stmt = self.conn.prepare("SELECT value FROM preferences WHERE key=?1").unwrap();
        let thread_iter = stmt.query_map(&[&key], |row| { row[0] });
        match thread_iter {
            Ok(rows) => {
                for i in thread_iter.unwrap() {
                    return Ok(i.unwrap());
                }
                return default;
            },
            _ => {
                default
            }
        }
//        for i in thread_iter {
//            return Ok(i.unwrap());
//        }
//        default
    }

    macro_rules! get {
        ($a: expr) => { get($a, None) };
        ($a: expr, $b: expr) => { get($a, $b) };
    }

}