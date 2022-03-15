use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{fmt::format, fs, time::Duration};

const PG_HOST: &str = "localhost";
const PG_BASE: &str = "todomvc";
const PG_USER: &str = "admin";
const PG_PSWD: &str = "podman";

pub type Db = Pool<Postgres>;

pub async fn init_db() -> Result<Db, sqlx::Error> {
    new_db_pool(PG_HOST, PG_BASE, PG_USER, PG_PSWD, 1).await
}

pub async fn pexec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
    let content = fs::read_to_string(file).map_err(|ex| {
        println!("ERROR reading {} (cause: {:?})", file, ex);
        ex
    })?;
    Ok(())
}

async fn new_db_pool(
    host: &str,
    db: &str,
    user: &str,
    pwd: &str,
    max_conn: u32,
) -> Result<Db, sqlx::Error> {
    let conn_string = format!("postgres://{}:{}@{}/{}", user, pwd, host, db);
    PgPoolOptions::new()
        .max_connections(max_conn)
        .connect_timeout(Duration::from_millis(500))
        .connect(&conn_string)
        .await
}

#[cfg(test)]
#[path = "../_tests/model_db.rs"]
mod tests;
