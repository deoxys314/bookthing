extern crate rusqlite;
extern crate xdg;

use crate::book::{Book, BookID, BookIdentifier};
use rusqlite::{Connection, Result, NO_PARAMS};
use std::fmt;
use std::path::PathBuf;
use xdg::BaseDirectories;

#[derive(Debug)]
pub struct BookConnection {
    connection: Connection,
}

impl Default for BookConnection {
    fn default() -> BookConnection {
        BookConnection::new()
    }
}

// instantiating the struct
impl BookConnection {
    pub fn new() -> BookConnection {
        BookConnection::establish_connection(None)
    }

    pub fn from_path(path: PathBuf) -> BookConnection {
        BookConnection::establish_connection(Some(path))
    }

    fn establish_connection(path: Option<PathBuf>) -> BookConnection {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("bookthing").unwrap();

        let database_path = match path {
            Some(p) => p,
            None => xdg_dirs
                .place_data_file("database.db")
                .expect("Connot find path to database."),
        };

        let conn =
            Connection::open(database_path).expect("Failed to establish database connection");

        conn.execute(
            r#"CREATE TABLE IF NOT EXISTS "books" (
        "id"    INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
        "title" TEXT,
        "author"        TEXT,
        "bookid"        TEXT,
        "idtype"        TEXT,
        "secondary_authors"     TEXT,
        "publication_year"      INTEGER,
        "publisher"     TEXT,
        "cover" BLOB,
        "copies"        INTEGER NOT NULL DEFAULT 1
        )"#,
            NO_PARAMS,
        );
        conn.execute(
            r#"CREATE TABLE IF NOT EXISTS "lending" (
        "id"    INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        "book_id"       INTEGER NOT NULL,
        "date"  TEXT,
        "action"        TEXT
        )"#,
            NO_PARAMS,
        );
        conn.execute(
            r#"CREATE TABLE IF NOT EXISTS "tags" (
        "id"    INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        "book_id"       INTEGER NOT NULL,
        "tag"   TEXT NOT NULL
        )"#,
            NO_PARAMS,
        );

        BookConnection { connection: conn }
    }
}

// database operations
impl BookConnection {
    // pub fn insert_book(&self, book: Book) -> Result<BookID> {
    //     let result = self.connection.execute(
    //         r#"INSERT INTO books (title, author) VALUES (?1, ?2)"#,
    //         &[&book.title, &book.author.unwrap()],
    //     );

    //     match result {
    //         Ok(_) => Ok(self.connection.last_insert_rowid() as BookID),
    //         Err(_) => Ok(3), // TODO: change this!
    //     }
    // }

    pub fn list(&self) -> Vec<Book> {
        let mut statement = self.connection.prepare(
            r#"SELECT id, title, author, bookid, idtype, secondary_authors, publication_year, publisher, copies
               FROM books
               ORDER BY title DESC"#)
            .expect("Failure in reading database.");

        let rows = statement.query_map(NO_PARAMS, |row| {
            Ok(Book {
                id: row.get(0).unwrap(),
                title: row.get(1).unwrap(),
                author: row.get(2).unwrap(),
                bookid: row.get(3).unwrap(),

                ..Book::new()
            })
        });

        let mut books = Vec::<Book>::new();

        for book in rows.unwrap() {
            books.push(book.unwrap());
        }

        books
    }
}
