use std::collections::HashMap;

// Livraria 2.0: Augmente a livraria feita anteriormente com estruturas de dados para eficientemente encontrar um livro pelo seu titulo e encontrar os livros escritos por um autor. Adicionar ISBN e palavras chave a cada livro. Introduzir procura por palavras chave eficiente com a capacidade de fazer procura por interseção de palavras chave ou união de palavras chave.

pub fn exercise_library() {
    let mut library = Library::new();
}

#[derive(Eq, PartialEq, Debug)]
struct Book {
    title: String,
    author: String,
    borrowed: bool,
}

impl Book {
    fn new(title: String, author: String, borrowed: bool) -> Self {
        Self { title, author, borrowed }
    }
}

#[derive(Debug)]
struct Library {
    books: HashMap<&'static str, Book>,
}

impl Library {
    fn new() -> Self {
        Self {}
    }
    // fn add_book(&mut self, book: Book) {
    //     self.books.push(book);
    // }
    // fn remove_book(&mut self) {
    //     let mut dd: HashMap<Book, String> = HashMap::new();
    //     self.books.pop();
    // }
    // fn borrow_book(&mut self, title: &str) {
    //     for book in &mut self.books {
    //         if book.title == title {
    //             book.borrowed = true;
    //         } else {
    //             println!("Book not found {}", title);
    //         }
    //     }
    // }
    // fn return_book(&mut self, title: &str) {
    //     for book in &mut self.books {
    //         if book.title == title {
    //             book.borrowed = false;
    //         } else {
    //             println!("Book not found {}", title);
    //         }
    //     }
    // }
}
