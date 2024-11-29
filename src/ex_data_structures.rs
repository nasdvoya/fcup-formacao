use std::collections::{HashMap, HashSet};

use uuid::Uuid;

// Livraria 2.0: Augmente a livraria feita anteriormente com estruturas de dados para eficientemente encontrar um livro pelo seu titulo e encontrar os livros escritos por um autor. Adicionar ISBN e palavras chave a cada livro. Introduzir procura por palavras chave eficiente com a capacidade de fazer procura por interseção de palavras chave ou união de palavras chave.

pub fn exercise_library() {
    let library = Library::new();
    library.search_intersection_keywords(vec!["rust".to_string()]);
    println!("This is a library {:#?}", library);
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
            HashSet::from(["rust".to_string(), "steve".to_string()]),
            false,
        );
        let book2_isbn = Isbn::new();
        let book2 = Book::new(
            book2_isbn.clone(),
            "Programming".to_string(),
            "Klabnik".to_string(),
            HashSet::from(["rust".to_string(), "programming".to_string()]),
            false,
        );
        books.insert(book1_isbn, book1);
        books.insert(book2_isbn, book2);
        Self(books)
    }

    // And
    fn search_intersection_keywords(&self, keys: Vec<String>) /* -> Vec<&Book> */
    {
        let books = self.0.values().filter(|book| keys.iter().all(|key| book.keywords.contains(key)));
        // for bk in books {}
        println!("{:?}", books);
        // books.collect()
    }

    // Or
    fn search_union_keywords(&self, keys: Vec<String>) -> Vec<Book> {
        todo!();
    }
}
