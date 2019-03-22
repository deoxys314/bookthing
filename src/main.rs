#[macro_use]
extern crate clap;

extern crate ansi_term;

use ansi_term::Style;

use clap::{Arg, App, SubCommand, ArgGroup};

use booklib::book;
use booklib::database;

fn main() -> Result<(), usize> {
    let matches = App::new("bookthing")
        .version(crate_version!())
        .author("Cameron Rossington <cameron.rossington@gmail.com>")
        .about("A suite of tools to help manage a personal library of books.")
        .long_about("A suite of tools to manage a personal library of books. Where relevant, commands will accept any unambiguous identifier to operate on a book (or books), such as book id, ISBN number, or title (fuzzy matching will be used).")
        .subcommand(SubCommand::with_name("add")
                    .about("Add a book to your personal database.")
                    .arg(Arg::with_name("id")
                         .help("The book to add, by ISBN or Library of Congress number.")
                         .value_name("ID")
                         .required(true)))
        .subcommand(SubCommand::with_name("add-parts")
                    .about("Add a book by parts.")
                    .arg(Arg::with_name("title")
                         .short("t")
                         .long("title")
                         .takes_value(true)
                         .help("The title of the book to enter. (Remember to use quotes.)"))
                    .arg(Arg::with_name("isbn")
                         .short("i")
                         .long("isbn")
                         .takes_value(true)
                         .help("The ISBN of the book to add. (Note that booktool can look up books by ISBN using the \"add\" subcommand.)"))
                    .arg(Arg::with_name("author")
                         .short("a")
                         .long("author")
                         .takes_value(true)
                         .help("The name of the primary author of the book to add. For consistency with automatically added books, be sure to quote the author as \"Lastname, Firstname\". E.g.: `--author \"Doe, John\"`"))
                    .arg(Arg::with_name("secondary-author")
                         .long("secondary-author")
                         .takes_value(true)
                         .multiple(true)
                         .number_of_values(1)
                         .help("The name of a secondary author of the book to add. For consistency with automatically added books, be sure to quote the author as \"Lastname, Firstname\". E.g.: `--secondary-author \"Doe, John\"` This value must be specified multiple times to specify multiple authors."))
                    .arg(Arg::with_name("publish-date")
                         .short("d")
                         .long("publish-date")
                         .takes_value(true)
                         .help("The date of publishing for the book to be added. For consistency with automatically added books, be sure to use dates in YYYY-MM-DD format, to as high a level of precision as possible."))
                    .group(ArgGroup::with_name("parts")
                           .args(&["title", "isbn", "author", "secondary-author", "publish-date"])
                           .required(true)   // these two lines say that we must have at least one of these args
                           .multiple(true))) // and than more than one is okay (in fact, it's preferable!)
        .subcommand(SubCommand::with_name("remove")
                    .about("Remove a book from your personal database.")
                    .arg(Arg::with_name("id")
                         .help("The book to remove, by book identifier.")
                         .value_name("ID")
                         .required(true)))
        .subcommand(SubCommand::with_name("list")
                    .about("Lists all books in your personal library. By default, shows book id, title, author and ISBN, and sorts results by title.")
                    .arg(Arg::with_name("header")
                         .short("h")
                         .long("header")
                         .takes_value(false)
                         .help("Print headers along with listed information."))
                    .arg(Arg::with_name("complete")
                         .short("c")
                         .long("complete")
                         .takes_value(false)
                         .help("Show all information available for each entry."))
                    .arg(Arg::with_name("id")
                         .long("id")
                         .takes_value(false)
                         .help("Sort results by book id."))
                    .arg(Arg::with_name("title")
                         .long("title")
                         .takes_value(false)
                         .help("Sort results by title. (Note: this is the default.)"))
                    .arg(Arg::with_name("isbn")
                         .long("isbn")
                         .takes_value(false)
                         .help("Sort results by isbn."))
                    .arg(Arg::with_name("copies")
                         .long("copies")
                         .takes_value(false)
                         .help("Sort results by the number of copies in your library, then by title."))
                    .arg(Arg::with_name("lending")
                         .long("lending")
                         .takes_value(false)
                         .help("Sort results by lending status, then by title."))
                    .group(ArgGroup::with_name("sorting")
                           .args(&["id", "title", "isbn", "copies", "lending"])))
        .subcommand(SubCommand::with_name("lend")
                    .about("Track lending status of a book in your library.")
                    .arg(Arg::with_name("in")
                         .long("in")
                         .takes_value(false)
                         .help("Check a book back in to your library."))
                    .arg(Arg::with_name("out")
                         .long("out")
                         .takes_value(true)
                         .number_of_values(2)
                         .help("Check a book back in to your library."))
                    .group(ArgGroup::with_name("lending")
                           .args(&["in", "out"])))
        .subcommand(SubCommand::with_name("history")
                    .about("Show the lending history of a book from your library.")
                    .arg(Arg::with_name("book")
                         .takes_value(true)
                         .help("A book identifier to show the history of.")))
        .subcommand(SubCommand::with_name("tag")
                    .about("Add a tag to a book by book identifier.")
                    .arg(Arg::with_name("book")
                         .takes_value(true)
                         .index(1)
                         .required(true))
                         .help("A book identifier.")
                    .arg(Arg::with_name("tag")
                         .takes_value(true)
                         .index(2)
                         .required(true)
                         .help("The tag to add to the book. Can be any string.")))

        .get_matches();


    // once we have gotten here, clap has already validated arguments

    match matches.subcommand() {
        ("list", Some(m)) => subcommand_list(m),
        _ => {
            println!("You have entered a valid subcommand, but which hasn't been implemented yet.");
            Ok(())
        }
    }
}


fn subcommand_list(matches: &clap::ArgMatches) -> Result<(), usize> {
    let books = database::BookConnection::new().list();

    if matches.is_present("header") {
        // print the headers here
        println!("{}", Style::new().bold().paint("ID\tTITLE\tAUTHOR\tISBN"));
    }
    for book in books {
        // print each book here
        println!("{}\t{}\t{}\t{}",
                 book.id,
                 Style::new().underline().paint(book.title),
                 book.author.unwrap_or("[Author Unknown]".to_string()),
                 book.bookid.unwrap_or("[no bookid]".to_string()));
    }
    // println!("{:?}", matches);
    Ok(())
}

