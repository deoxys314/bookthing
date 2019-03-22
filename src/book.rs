pub type BookID = usize;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BookIdentifier {
    ISBN(String),
    None,
}

impl Default for BookIdentifier {
    fn default() -> Self { BookIdentifier::None }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Book {
    pub title: String,
    pub author: Option<String>,
    pub id: u32,
    pub bookid: Option<String>,
    pub secondary_authors: Option<Vec<String>>,
    pub publication_year: Option<u16>,
    pub publisher: Option<String>,
    pub cover: Option<Vec<u8>>,
    pub copies: Option<usize>,
}

impl Book {
    pub fn new() -> Book {
        Book::default()
    }

    pub fn author(mut self, author: String) -> Book {
        self.author = Some(author);
        self
    }

    pub fn isbn(mut self, isbn: String) -> Book {
        // TODO: validate isbns here
        self.author = Some(isbn);
        self
    }

    // functions: author(), title(), isbn, etc etc
    // everything needed for the "by parts" section
    //
    //
    // from_isbn()
    // from_LoC()
    //
    // from_parts(author: Option<String> . . .
    //   this one is not going to be used, probably, but it's good to have
}
