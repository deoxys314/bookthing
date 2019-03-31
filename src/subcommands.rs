extern crate ansi_term;
extern crate clap;
extern crate rusqlite;
extern crate tabwriter;

use ansi_term::Style;
use booklib::database;
use rusqlite::Result;
use std::io::{self, Write};
use tabwriter::TabWriter;

pub fn list(matches: &clap::ArgMatches) -> Result<()> {

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
            book.author.unwrap_or("[Author Unknown]".to_string()),
            book.bookid.unwrap_or("[no bookid]".to_string())
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
                    .unwrap_or("[publisher unknown]".to_string()),
                completebook.copies.unwrap_or(1)
            ).unwrap();
        }

        write!(&mut tw, "\n").unwrap();
    }

    tw.flush().unwrap(); // This must be called to ensure that the tabwriter actally writes.
    Ok(())
}
