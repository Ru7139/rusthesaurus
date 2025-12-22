fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = rusqlite::Connection::open_in_memory()?;

    conn.execute(
        "
        CREATE TABLE person (
            id      INTEGER PRIMARY KEY,
            name    TEXT NOT NULL,
            yob     INTEGER,
            data    BLOB
        ) STRICT
        ",
        (),
    )?;

    Ok(())
}
