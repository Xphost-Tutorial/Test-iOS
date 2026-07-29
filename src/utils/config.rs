use rusqlite::Connection;

pub struct Config {}

impl Config {
    pub fn new() -> Self {
        Self {}
    }
    pub fn load(&mut self) -> Result<(), rusqlite::Error> {
        let path = crate::utils::path::CONFIG_LOCAL_DIR.get().unwrap();
        if !path.exists() {
            let _ = std::fs::create_dir_all(&path);
        }
        let mut conn = Connection::open(path.join("data.db"))?;
        conn.execute(
            r#"
CREATE TABLE IF NOT EXISTS config (
    name TEXT PRIMARY KEY,
    value TEXT
);
        "#,
            [],
        )?;
        let tx = conn.transaction()?;
        tx.commit()?;
        Ok(())
    }

    pub fn save(&mut self) -> Result<(), rusqlite::Error> {
        let path = crate::utils::path::CONFIG_LOCAL_DIR.get().unwrap();
        if !path.exists() {
            let _ = std::fs::create_dir_all(&path);
        }
        let mut conn = Connection::open(path.join("data.db"))?;
        conn.execute(
            r#"
CREATE TABLE IF NOT EXISTS config (
    name TEXT PRIMARY KEY,
    value TEXT
);
        "#,
            [],
        )?;
        let tx = conn.transaction()?;
        tx.commit()?;
        Ok(())
    }
}
