use std::collections::{HashMap, HashSet};

use uuid::Uuid;

pub fn exercise_library() {
    let mut library = Library::new();
    library.search_intersection_keywords(vec!["test1".to_string(), "test2".to_string()]);
    library.search_union_keywords(vec!["test1".to_string(), "test2".to_string()]);

    library.find_books_by_author("Klabnik");
    library.find_book_by_title("The Rust Programming Language");

    let book = LibraryItems::Book {
        details: Details::new("".to_string(), "".to_string()),
        keywords: HashSet::from(["something".to_string(), "something".to_string()]),
        borrowed: false,
    };

    let new_book = Book::new(
        Isbn::new(),
        Details::new("Some cool book".to_string(), "JohnyDoes".to_string()),
        HashSet::from(["something".to_string(), "learning".to_string()]),
        false,
    );
    library.add_book(new_book);
    println!("{:#?}", library);
    library.borrow_book("Some book");
    println!("{:#?}", library);
    library.return_book("Some book");
    println!("{:#?}", library);
    library.remove_book("Some book");
}

#[derive(Eq, PartialEq, Debug)]
struct Book {
    isbn: Isbn,
    details: Details,
    keywords: HashSet<String>,
    borrowed: bool,
}

#[derive(Eq, PartialEq, Debug)]
struct Statue {
    details: Details,
    size: (usize, usize, usize),
}
#[derive(Eq, PartialEq, Debug)]
struct Painting {
    details: Details,
}

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
struct Isbn(String);

#[derive(Debug, Eq, PartialEq)]
struct Details {
    title: String,
    author: String,
}

#[derive(Debug)]
enum LibraryItems {
    Panting {
        details: Details,
    },
    Statue {
        details: Details,
        size: (u32, u32, u32),
    },
    Book {
        details: Details,
        keywords: HashSet<String>,
        borrowed: bool,
    },
    AudioBook {
        details: Details,
        keywords: HashSet<String>,
        borrowed: bool,
    },
}

#[derive(Debug)]
struct Library(HashMap<Isbn, LibraryItems>);

impl Book {
    fn new(isbn: Isbn, details: Details, keywords: HashSet<String>, borrowed: bool) -> Self {
        Self {
            isbn,
            details,
            keywords,
            borrowed,
        }
    }
}

impl Isbn {
    fn new() -> Self {
        let isbn = Uuid::new_v4();
        Self(isbn.to_string())
    }
}

impl Details {
    fn new(title: String, author: String) -> Self {
        Self { title, author }
    }
}

impl Library {
    fn new() -> Self {
        let mut books: HashMap<Isbn, LibraryItems> = HashMap::new();
        // TODO: Fazer sem cloning?

        let book2_isbn = Isbn::new();
        let booky = LibraryItems::Book {
            details: Details::new("".to_string(), "".to_string()),
            keywords: HashSet::from(["something".to_string(), "something".to_string()]),
            borrowed: false,
        };

        books.insert(book2_isbn, booky);
        Self(books)
    }

    // Search for book or audiobooks that contain instersection of keys.
    fn search_intersection_keywords(&self, keys: Vec<String>) {
        let match_books: Vec<&LibraryItems> = self
            .0
            .values()
            .filter(|lib_item| match lib_item {
                LibraryItems::Book { keywords, .. } | LibraryItems::AudioBook { keywords, .. } => {
                    keys.iter().all(|key| keywords.contains(key))
                }
                _ => false,
            })
            .collect();

        if match_books.is_empty() {
            println!("No books contain specified keywords: {:?}", keys);
        } else {
            for book in match_books {
                println!("{:?}", book);
            }
        }
    }

    // Search for book or audiobooks that contain union of keys.
    fn search_union_keywords(&self, keys: Vec<String>) {
        let match_books: Vec<&LibraryItems> = self
            .0
            .values()
            .filter(|lib_item| match lib_item {
                LibraryItems::Book { keywords, .. } | LibraryItems::AudioBook { keywords, .. } => {
                    keys.iter().any(|key| keywords.contains(key))
                }
                _ => false,
            })
            .collect();

        if match_books.is_empty() {
            println!("No books contain specified keywords: {:?}", keys);
        } else {
            for book in match_books {
                println!("{:?}", book);
            }
        }
    }

    /// Find book or audio book by author.
    fn find_books_by_author(&self, author: &str) {
        let books_by_author: Vec<&LibraryItems> = self
            .0
            .values()
            .filter(|lib_item| match lib_item {
                LibraryItems::Book { details, .. } | LibraryItems::AudioBook { details, .. } => author == details.author,
                _ => false,
            })
            .collect();

        if books_by_author.is_empty() {
            println!("No books found by author: {}", author);
        } else {
            println!("Books by author '{}':", author);
            for book in books_by_author {
                println!("{:?}", book);
            }
        }
    }

    /// Find book or audio book by title.
    fn find_book_by_title(&self, title: &str) {
        let book = self.0.values().find(|lib_item| match lib_item {
            LibraryItems::Book { details, .. } | LibraryItems::AudioBook { details, .. } => details.title == title,
            _ => false,
        });
        match book {
            Some(book) => println!("Found book by title '{}':\n{:?}", title, book),
            None => println!("No book found with title: {}", title),
        }
    }

    /// Borrow book or audiobook. I guess we are not allowed to borrow statues...
    fn borrow_book(&mut self, title: &str) {
        // let book = self.0.values_mut().find(|book| book.details.title == title);
        let book = self.0.values_mut().find(|lib_item| match lib_item {
            LibraryItems::Book { details, .. } | LibraryItems::AudioBook { details, .. } => details.title == title,
            _ => false,
        });

        match book {
            Some(LibraryItems::Book { borrowed, .. } | LibraryItems::AudioBook { borrowed, .. }) => {
                if *borrowed {
                    println!("The book/audiobook {} is already borrowed", title);
                } else {
                    *borrowed = true;
                    println!("You just borrowed the book {}", title);
                }
            }
            _ => println!("Book not found: {}", title),
        }
    }

    /// Return the borrowed book
    fn return_book(&mut self, title: &str) {
        // let book = self.0.values_mut().find(|book| book.details.title == title);

        let book = self.0.values_mut().find(|lib_item| match lib_item {
            LibraryItems::Book { details, .. } | LibraryItems::AudioBook { details, .. } => details.title == title,
            _ => false,
        });

        match book {
            Some(LibraryItems::Book { borrowed, .. } | LibraryItems::AudioBook { borrowed, .. }) => {
                if !*borrowed {
                    println!("The book '{}' was not borrowed.", title);
                } else {
                    *borrowed = false;
                    println!("You returned the book '{}'.", title);
                }
            }
            _ => println!("Book not found: {}", title),
        }
    }

    fn add_book(&mut self, book: Book) {
        if self.0.contains_key(&book.isbn) {
            println!("Book with ISBN {} already exists.", book.isbn.0);
        } else {
            self.0.insert(book.isbn.clone(), book);
            println!("Book added successfully.");
        }
    }

    fn remove_book(&mut self, title: &str) {
        let book_key = self
            .0
            .iter()
            .find(|(_, library_item)| match library_item {
                LibraryItems::Panting { details, .. }
                | LibraryItems::Statue { details, .. }
                | LibraryItems::Book { details, .. }
                | LibraryItems::AudioBook { details, .. } => details.title == title,
            })
            .map(|(isbn, _)| isbn.clone());

        match book_key {
            Some(isbn) => {
                self.0.remove(&isbn);
                println!("The book '{}' was removed from the library.", title);
            }
            None => println!("Book not found: {}", title),
        }
    }
}
