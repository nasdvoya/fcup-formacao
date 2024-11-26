use std::collections::{HashMap, HashSet};

use uuid::Uuid;

// Livraria 2.0: Augmente a livraria feita anteriormente com estruturas de dados para eficientemente encontrar um livro pelo seu titulo e encontrar os livros escritos por um autor. Adicionar ISBN e palavras chave a cada livro. Introduzir procura por palavras chave eficiente com a capacidade de fazer procura por interseção de palavras chave ou união de palavras chave.

pub fn exercise_library() {
    let mut library = Library::new();
}

// TODO: Why did I need to add Hash here?
#[derive(Eq, PartialEq, Debug, Hash)]
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
    fn new(isbn: &Isbn, title: String, author: String, keywords: HashSet<String>, borrowed: bool) -> Self {
        let isbn = Isbn::new();
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
struct Library(HashMap<&Isbn, Book>);

impl Library {
    fn new() -> Self {
        let mut books: HashMap<Isbn, Book> = HashMap::new();
        let book1 = Isbn::new();
        books.insert(
            book1,
            Book::new(
                &book1,
                "The Rust Programming Language".to_string(),
                "Steve Klabnik and Carol Nichols".to_string(),
                HashSet::from(["rust".to_string(), "programming".to_string()]),
                false,
            ),
        );
        Self(books)
    }
}
