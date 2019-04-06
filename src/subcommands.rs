extern crate ansi_term;
extern crate clap;
extern crate reqwest;
extern crate rusqlite;
extern crate serde_json;
extern crate tabwriter;

use ansi_term::Style;
use booklib::database;
use serde_json::Value;
use std::io::{self, Write};
use tabwriter::TabWriter;

#[derive(Debug)]
pub enum SubCommandError {
    ReqwestError(reqwest::Error),
    RusqliteError(rusqlite::Error),
    SerdeError(serde_json::Error),
}

impl From<serde_json::Error> for SubCommandError {
    fn from(error: serde_json::Error) -> Self {
        SubCommandError::SerdeError(error)
    }
}

impl From<reqwest::Error> for SubCommandError {
    fn from(error: reqwest::Error) -> Self {
        SubCommandError::ReqwestError(error)
    }
}

impl From<rusqlite::Error> for SubCommandError {
    fn from(error: rusqlite::Error) -> Self {
        SubCommandError::RusqliteError(error)
    }
}

impl From<SubCommandError> for usize {
    fn from(error: SubCommandError) -> Self {
        100 + match error {
            SubCommandError::ReqwestError(_) => 1,
            SubCommandError::RusqliteError(_) => 2,
            SubCommandError::SerdeError(_) => 3,
        }
    }
}

pub fn add(matches: &clap::ArgMatches) -> Result<(), SubCommandError> {
    // TODO: feed in crap data, see what happens
    let googleurl = format!(
        "https://www.googleapis.com/books/v1/volumes?q=isbn:{}",
        matches.value_of("id").unwrap()
    );

    let result = reqwest::get(&googleurl)?.text()?;

    let v: Value = serde_json::from_str(&result)?;

    println!("{:?}", v);
    Ok(())
}

pub fn list(matches: &clap::ArgMatches) -> Result<(), SubCommandError> {
    let order = if matches.is_present("id") {
        database::BookOrder::Id
    } else if matches.is_present("isbn") {
        database::BookOrder::ISBN
    } else if matches.is_present("copies") {
        database::BookOrder::Copies
    } else if matches.is_present("author") {
        database::BookOrder::Author
    } else {
        database::BookOrder::Title
    };

    let books = database::BookConnection::new()?.list(order);

    let mut tw = TabWriter::new(io::stdout()).padding(1);

    if matches.is_present("header") {
        let bold = Style::new().bold();
        // print the headers here
        write!(&mut tw, "{}", bold.paint("ID\tTITLE\tAUTHOR\tISBN")).unwrap();

        if matches.is_present("complete") {
            write!(
                &mut tw,
                "{}",
                bold.paint("\tSECONDARY AUTHORS\tPUBLICATION YEAR\tPUBLISHER\tCOPIES")
            ).unwrap();
        }

        write!(&mut tw, "\n").unwrap();
    }

    for book in books {
        let completebook = book.clone();
        write!(
            &mut tw,
            "{}\t{}\t{}\t{}",
            book.id,
            Style::new().underline().paint(book.title),
            book.author.unwrap_or_else(|| "[Author Unknown]".to_string()),
            book.bookid.unwrap_or_else(|| "[no bookid]".to_string())
        ).unwrap();
        // print each book here
        if matches.is_present("complete") {
            write!(
                &mut tw,
                "\t{}\t{}\t{}\t{}",
                match completebook.secondary_authors {
                    Some(a) => a.join(", "),
                    None => "-".to_string(),
                },
                match completebook.publication_year {
                    Some(n) => n.to_string(),
                    None => "[unknown]".to_string(),
                },
                completebook
                    .publisher
                    .unwrap_or_else(|| "[publisher unknown]".to_string()),
                completebook.copies.unwrap_or(1)
            ).unwrap();
        }

        write!(&mut tw, "\n").unwrap();
    }

    tw.flush().unwrap(); // This must be called to ensure that the tabwriter actally writes.
    Ok(())
}
