use std::fs::create_dir_all;

use rusqlite::Connection;
use thiserror::Error;
use tracing::{debug, info};

use crate::users_dirs::{UsersDirsError, get_config_dir};

#[derive(Error, Debug)]
pub enum AuthenticationDatabaseError {
    #[error("Sqlite database is not initialized.")]
    DatabaseNotInitialized,
    #[error(transparent)]
    SqliteError(#[from] rusqlite::Error),
    #[error(transparent)]
    StdIoError(#[from] std::io::Error),
    #[error(transparent)]
    UsersDirsError(#[from] UsersDirsError),
}

#[derive(Debug)]
pub struct AuthenticationDatabase {
    connection: Connection,
}

impl AuthenticationDatabase {
    #[tracing::instrument]
    pub fn new() -> Result<Self, AuthenticationDatabaseError> {
        Ok(AuthenticationDatabase {
            connection: Self::get_connection()?,
        })
    }

    #[tracing::instrument]
    fn get_database_version(connection: &Connection) -> Result<u32, AuthenticationDatabaseError> {
        info!("Selecting rows from sqlite_master table.");
        let mut stmt: rusqlite::Statement<'_> = connection.prepare(
            "SELECT name FROM sqlite_master WHERE type='table' AND name IN ('Credentials', 'Metadata');")?;
        let rows = stmt
            .query_map([], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .collect::<Vec<String>>();

        if rows.is_empty() {
            return Ok(0);
        }

        let mut stmt: rusqlite::Statement<'_> =
            connection.prepare("SELECT value FROM Metadata WHERE key='version'")?;
        let rows = stmt
            .query_map([], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .map(|r: String| r.parse::<u32>())
            .filter_map(|r| r.ok())
            .collect::<Vec<u32>>();

        if rows.is_empty() {
            return Ok(0);
        }

        let version = rows.first().expect("Query should return at least 1 item.");

        Ok(version.to_owned())
    }

    #[tracing::instrument]
    fn get_connection() -> Result<Connection, AuthenticationDatabaseError> {
        // Get the path to the credential database
        let mut path = get_config_dir()?;
        path.push("credential_store.db");
        let sqlite_path = path.as_path();

        // Create the folder if it doesn't already exist.
        if !sqlite_path.parent().expect("No parent").exists() {
            debug!("Creating directories for sqlite database.");
            create_dir_all(sqlite_path.parent().expect("No parent"))?;
        }

        debug!("Creating sqlite database connection.");
        let connection = Connection::open(sqlite_path)?;

        // Create the tables if they don't already exist.
        Self::upgrade_database(&connection)?;
        Ok(connection)
    }

    #[tracing::instrument]
    fn upgrade_database(connection: &Connection) -> Result<(), AuthenticationDatabaseError> {
        let version = Self::get_database_version(connection)?;

        info!("Current database version is {}.", version);

        if version == 0 {
            info!("Upgrading database to version 1.");

            connection.execute(
                "CREATE TABLE IF NOT EXISTS Credentials (
                        id                      INTEGER PRIMARY KEY,
                        url                     TEXT NOT NULL,
                        user                    TEXT NOT NULL,
                        device_id               TEXT NOT NULL
                    )",
                (), // empty list of parameters.
            )?;

            connection.execute(
                "CREATE TABLE IF NOT EXISTS Metadata (
                        id                      INTEGER PRIMARY KEY,
                        key                     TEXT NOT NULL,
                        value                   TEXT NOT NULL
                    )",
                (), // empty list of parameters.
            )?;

            connection.execute(
                "INSERT INTO Metadata (key, value) VALUES (?1, ?2)",
                ("version", 1.to_string()),
            )?;
        }

        Ok(())
    }
}
