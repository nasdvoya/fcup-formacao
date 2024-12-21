use std::collections::{HashMap, HashSet};

use uuid::Uuid;

pub fn exercise_library() {
    let mut library = Library::new();
    library.search_intersection_keywords(vec!["test1".to_string(), "test2".to_string()]);
    library.search_union_keywords(vec!["test1".to_string(), "test2".to_string()]);

    library.find_books_by_author("Klabnik");
    library.find_book_by_title("The Rust Programming Language");

    let new_book = Book::new(
        Isbn::new(),
        "Some book".to_string(),
        "John Doe".to_string(),
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

// TODO: Why did I need to add Hash here?
#[derive(Eq, PartialEq, Debug, Hash, Clone)]
struct Isbn(String);

impl Isbn {
    fn new() -> Self {
        let isbn = Uuid::new_v4();
        Self(isbn.to_string())
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Book {
    isbn: Isbn,
    title: String,
    author: String,
    keywords: HashSet<String>,
    borrowed: bool,
}

impl Book {
    fn new(isbn: Isbn, title: String, author: String, keywords: HashSet<String>, borrowed: bool) -> Self {
        Self {
            isbn,
            title,
            author,
            keywords,
            borrowed,
        }
    }
}

#[derive(Debug)]
struct Library(HashMap<Isbn, Book>);

// TODO: Reference on hashmap?
impl Library {
    fn new() -> Self {
        let mut books: HashMap<Isbn, Book> = HashMap::new();
        // TODO: Fazer sem cloning?
        let book1_isbn = Isbn::new();
        let book1 = Book::new(
            book1_isbn.clone(),
            "The Rust Programming Language".to_string(),
            "Steve Klabnik and Carol Nichols".to_string(),
            HashSet::from(["test1".to_string(), "test2".to_string()]),
            false,
        );
        let book2_isbn = Isbn::new();
        let book2 = Book::new(
            book2_isbn.clone(),
            "Programming".to_string(),
            "Klabnik".to_string(),
            HashSet::from(["test3".to_string(), "test4".to_string()]),
            false,
        );
        books.insert(book1_isbn, book1);
        books.insert(book2_isbn, book2);
        Self(books)
    }

    fn search_intersection_keywords(&self, keys: Vec<String>) {
        let match_books: Vec<&Book> = self
            .0
            .values()
            .filter(|book| keys.iter().all(|key| book.keywords.contains(key)))
            .collect();

        if match_books.is_empty() {
            println!("No books contain specified keywords: {:?}", keys);
        } else {
            for book in match_books {
                println!("{:?}", book);
            }
        }
    }

    fn search_union_keywords(&self, keys: Vec<String>) {
        let match_books: Vec<&Book> = self
            .0
            .values()
            .filter(|book| keys.iter().any(|key| book.keywords.contains(key)))
            .collect();

        if match_books.is_empty() {
            println!("No books contain specified keywords: {:?}", keys);
        } else {
            for book in match_books {
                println!("{:?}", book);
            }
        }
    }

    fn find_books_by_author(&self, author: &str) {
        let books_by_author: Vec<&Book> = self.0.values().filter(|book| book.author == author).collect();

        if books_by_author.is_empty() {
            println!("No books found by author: {}", author);
        } else {
            println!("Books by author '{}':", author);
            for book in books_by_author {
                println!("{:?}", book);
            }
        }
    }

    fn find_book_by_title(&self, title: &str) {
        let book = self.0.values().find(|book| book.title == title);

        match book {
            Some(book) => println!("Found book by title '{}':\n{:?}", title, book),
            None => println!("No book found with title: {}", title),
        }
    }

    fn borrow_book(&mut self, title: &str) {
        let book = self.0.values_mut().find(|book| book.title == title);

        match book {
            Some(book) => {
                if book.borrowed {
                    println!("The book '{}' is already borrowed.", title);
                } else {
                    book.borrowed = true;
                    println!("You borrowed the book '{}'.", title);
                }
            }
            None => println!("Book not found: {}", title),
        }
    }

    fn return_book(&mut self, title: &str) {
        let book = self.0.values_mut().find(|book| book.title == title);

        match book {
            Some(book) => {
                if !book.borrowed {
                    println!("The book '{}' was not borrowed.", title);
                } else {
                    book.borrowed = false;
                    println!("You returned the book '{}'.", title);
                }
            }
            None => println!("Book not found: {}", title),
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
            .find(|(_, book)| book.title == title)
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
