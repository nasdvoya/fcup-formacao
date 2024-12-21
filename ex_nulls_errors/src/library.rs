use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use uuid::Uuid;

pub fn exercise_library() {
    let mut library = Library::new();

    let programming_book = Book::new(
        Isbn::new(),
        Details::new("Some cool Language".to_string(), "Someone somewhere".to_string()),
        HashSet::from(["programming".to_string(), "cool".to_string(), "comscience".to_string()]),
        false,
    );
    let novel_book = Book::new(
        Isbn::new(),
        Details::new("The Ship".to_string(), "Jane".to_string()),
        HashSet::from(["fiction".to_string(), "novel".to_string(), "bestseller".to_string()]),
        false,
    );
    let audio_course = AudioBook::new(
        Isbn::new(),
        Details::new("Learn Rust".to_string(), "Rust Expert Guy".to_string()),
        HashSet::from(["programming".to_string(), "rust".to_string(), "audio course".to_string()]),
        false,
        Duration::from_secs(3600), 
    );

    let statue = Statue {
        details: Details::new("Ancient Warrior".to_string(), "Classical Artist".to_string()),
        size: (200, 100, 300),
    };

    library.add_item(LibraryItem::Book(programming_book));
    library.add_item(LibraryItem::Book(novel_book));
    library.add_item(LibraryItem::AudioBook(audio_course));
    library.add_item(LibraryItem::Statue(statue));

    println!("\n--- Testing find by title (Option return) ---");
    match library.find_by_title("Learn Rust") {
        Some(item) => println!("Found item: {:?}", item),
        None => println!("Item not found"),
    }

    println!("\n--- Testing find by author (Option return) ---");
    match library.find_by_author("Rust Expert Guy") {
        Some(items) => {
            println!("Found items by author:");
            for item in items {
                println!("{:?}", item);
            }
        }
        None => println!("No items found by author"),
    }

    println!("\n--- Testing find by type ---");
    match library.find_by_type(ItemType::AudioBook) {
        Some(items) => {
            println!("Found audio books:");
            for item in items {
                println!("{:?}", item);
            }
        }
        None => println!("No audio books found"),
    }

    println!("\n--- Testing get audio duration ---");
    match library.get_item_dimensions("Ancient Warrior") {
        Ok(dimensions) => println!("Statue dimensions: {:?}", dimensions),
        Err(LibraryError::InvalidOperation(msg)) => println!("Operation error: {}", msg),
        Err(LibraryError::ItemNotFound) => println!("Item not found"),
    }

    // Try to get dimensions of a book (should fail)
    match library.get_item_dimensions("Some cool Language") {
        Ok(dimensions) => println!("Dimensions: {:?}", dimensions),
        Err(LibraryError::InvalidOperation(msg)) => println!("Operation error: {}", msg),
        Err(LibraryError::ItemNotFound) => println!("Item not found"),
    }

    println!("\n--- Testing borrow/return functionality ---");
    library.borrow_book("Learn Rust");
    library.borrow_book("Learn Rust"); 
    library.return_book("Learn Rust");
    library.return_book("Learn Rust"); 

    // Test keyword search
    println!("\n--- Testing keyword search ---");
    library.search_intersection_keywords(vec!["programming".to_string(), "rust".to_string()]);
    library.search_union_keywords(vec!["fiction".to_string(), "programming".to_string()]);
}
#[derive(Debug)]
pub enum LibraryError {
    InvalidOperation(String),
    ItemNotFound,
}

#[derive(Debug, PartialEq)]
pub enum ItemType {
    Book,
    AudioBook,
    Statue,
    Painting,
}

#[derive(Debug)]
enum LibraryItem {
    Painting(Painting),
    Statue(Statue),
    Book(Book),
    AudioBook(AudioBook),
}

#[derive(Eq, PartialEq, Debug)]
struct Book {
    isbn: Isbn,
    details: Details,
    keywords: HashSet<String>,
    borrowed: bool,
}

#[derive(Eq, PartialEq, Debug)]
struct AudioBook {
    isbn: Isbn,
    details: Details,
    keywords: HashSet<String>,
    borrowed: bool,
    duration: Duration,
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
struct Library(HashMap<Isbn, LibraryItem>);

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

impl AudioBook {
    fn new(isbn: Isbn, details: Details, keywords: HashSet<String>, borrowed: bool, duration: Duration) -> Self {
        Self {
            isbn,
            details,
            keywords,
            borrowed,
            duration,
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
        let mut books: HashMap<Isbn, LibraryItem> = HashMap::new();
        let book2_isbn = Isbn::new();
        let book = Book::new(
            book2_isbn.clone(),
            Details::new("".to_string(), "".to_string()),
            HashSet::from(["something".to_string(), "something".to_string()]),
            false,
        );
        books.insert(book2_isbn, LibraryItem::Book(book));
        Self(books)
    }

    pub fn get_item_dimensions(&self, title: &str) -> Result<(usize, usize, usize), LibraryError> {
        match self.find_by_title(title) {
            Some(LibraryItem::Statue(statue)) => Ok(statue.size),
            Some(_) => Err(LibraryError::InvalidOperation(
                "This item doesn't have dimensions".to_string(),
            )),
            None => Err(LibraryError::ItemNotFound),
        }
    }

    pub fn find_by_author(&self, author: &str) -> Option<Vec<&LibraryItem>> {
        let items: Vec<&LibraryItem> = self
            .0
            .values()
            .filter(|item| match item {
                LibraryItem::Book(book) => book.details.author == author,
                LibraryItem::AudioBook(audio) => audio.details.author == author,
                LibraryItem::Statue(statue) => statue.details.author == author,
                LibraryItem::Painting(painting) => painting.details.author == author,
            })
            .collect();

        if items.is_empty() {
            None
        } else {
            Some(items)
        }
    }

    pub fn find_by_type(&self, item_type: ItemType) -> Option<Vec<&LibraryItem>> {
        let items: Vec<&LibraryItem> = self
            .0
            .values()
            .filter(|item| match (item, &item_type) {
                (LibraryItem::Book(_), ItemType::Book) => true,
                (LibraryItem::AudioBook(_), ItemType::AudioBook) => true,
                (LibraryItem::Statue(_), ItemType::Statue) => true,
                (LibraryItem::Painting(_), ItemType::Painting) => true,
                _ => false,
            })
            .collect();

        if items.is_empty() {
            None
        } else {
            Some(items)
        }
    }

    pub fn find_by_title(&self, title: &str) -> Option<&LibraryItem> {
        self.0.values().find(|item| match item {
            LibraryItem::Book(book) => book.details.title == title,
            LibraryItem::AudioBook(audio) => audio.details.title == title,
            LibraryItem::Statue(statue) => statue.details.title == title,
            LibraryItem::Painting(painting) => painting.details.title == title,
        })
    }

    fn search_intersection_keywords(&self, keys: Vec<String>) {
        let match_books: Vec<&LibraryItem> = self
            .0
            .values()
            .filter(|item| match item {
                LibraryItem::Book(book) => keys.iter().all(|key| book.keywords.contains(key)),
                LibraryItem::AudioBook(audiobook) => keys.iter().all(|key| audiobook.keywords.contains(key)),
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

    fn search_union_keywords(&self, keys: Vec<String>) {
        let match_books: Vec<&LibraryItem> = self
            .0
            .values()
            .filter(|item| match item {
                LibraryItem::Book(book) => keys.iter().any(|key| book.keywords.contains(key)),
                LibraryItem::AudioBook(audiobook) => keys.iter().any(|key| audiobook.keywords.contains(key)),
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

    fn find_books_by_author(&self, author: &str) {
        let books_by_author: Vec<&LibraryItem> = self
            .0
            .values()
            .filter(|item| match item {
                LibraryItem::Book(book) => book.details.author == author,
                LibraryItem::AudioBook(audiobook) => audiobook.details.author == author,
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

    fn find_book_by_title(&self, title: &str) {
        let book = self.0.values().find(|item| match item {
            LibraryItem::Book(book) => book.details.title == title,
            LibraryItem::AudioBook(audiobook) => audiobook.details.title == title,
            _ => false,
        });
        match book {
            Some(book) => println!("Found book by title '{}':\n{:?}", title, book),
            None => println!("No book found with title: {}", title),
        }
    }

    fn borrow_book(&mut self, title: &str) {
        let book = self.0.values_mut().find(|item| match item {
            LibraryItem::Book(book) => book.details.title == title,
            LibraryItem::AudioBook(audiobook) => audiobook.details.title == title,
            _ => false,
        });

        match book {
            Some(LibraryItem::Book(book)) => {
                if book.borrowed {
                    println!("The book {} is already borrowed", title);
                } else {
                    book.borrowed = true;
                    println!("You just borrowed the book {}", title);
                }
            }
            Some(LibraryItem::AudioBook(audiobook)) => {
                if audiobook.borrowed {
                    println!("The audiobook {} is already borrowed", title);
                } else {
                    audiobook.borrowed = true;
                    println!("You just borrowed the audiobook {}", title);
                }
            }
            _ => println!("Book not found: {}", title),
        }
    }

    fn return_book(&mut self, title: &str) {
        let book = self.0.values_mut().find(|item| match item {
            LibraryItem::Book(book) => book.details.title == title,
            LibraryItem::AudioBook(audiobook) => audiobook.details.title == title,
            _ => false,
        });

        match book {
            Some(LibraryItem::Book(book)) => {
                if !book.borrowed {
                    println!("The book '{}' was not borrowed.", title);
                } else {
                    book.borrowed = false;
                    println!("You returned the book '{}'.", title);
                }
            }
            Some(LibraryItem::AudioBook(audiobook)) => {
                if !audiobook.borrowed {
                    println!("The audiobook '{}' was not borrowed.", title);
                } else {
                    audiobook.borrowed = false;
                    println!("You returned the audiobook '{}'.", title);
                }
            }
            _ => println!("Book not found: {}", title),
        }
    }

    fn add_item(&mut self, item: LibraryItem) {
        let isbn = match &item {
            LibraryItem::Book(book) => &book.isbn,
            LibraryItem::AudioBook(audiobook) => &audiobook.isbn,
            _ => return println!("Only books and audiobooks can be added to the library"),
        };

        if self.0.contains_key(isbn) {
            println!("Item with ISBN {} already exists.", isbn.0);
        } else {
            self.0.insert(isbn.clone(), item);
            println!("Item added successfully.");
        }
    }

    fn remove_book(&mut self, title: &str) {
        let book_key = self
            .0
            .iter()
            .find(|(_, item)| match item {
                LibraryItem::Book(book) => book.details.title == title,
                LibraryItem::AudioBook(audiobook) => audiobook.details.title == title,
                LibraryItem::Painting(painting) => painting.details.title == title,
                LibraryItem::Statue(statue) => statue.details.title == title,
            })
            .map(|(isbn, _)| isbn.clone());

        match book_key {
            Some(isbn) => {
                self.0.remove(&isbn);
                println!("The item '{}' was removed from the library.", title);
            }
            None => println!("Item not found: {}", title),
        }
    }
}
