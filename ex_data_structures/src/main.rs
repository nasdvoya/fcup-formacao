use std::collections::{HashMap, HashSet};
use uuid::Uuid;

// Livraria 2.0: Augmente a livraria feita anteriormente com estruturas de dados para eficientemente encontrar um livro pelo seu titulo e encontrar os livros escritos por um autor. Adicionar ISBN e palavras chave a cada livro. Introduzir procura por palavras chave eficiente com a capacidade de fazer procura por interseção de palavras chave ou união de palavras chave.
// Mercearia 2.0: Augmente a Merceria para poder mos encontrar eficientemente os produtos dentro de uma fileira, prateleira e zona. Devemos também ser capazes de encontrar eficientemente um produto e a sua posição.

fn main() {
    exercise_store();
    exercise_library();
    exercise_array_mutation();
}
// --------------------------------------------------------------------------------------------------------------
// --------------------- Library 2.0 ----------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------
fn exercise_library() {
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

// --------------------------------------------------------------------------------------------------------------
// --------------------- Store 2.0 ------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------

fn exercise_store() {
    let mut ashop = Shop::new();
    println!("Shop: {:#?}", ashop);
    let mut soda_product = Product::new(String::from("ID234"), String::from("Pepsi"), 33, 2.45, 200);
    soda_product.change_price(2.55);
    ashop.add_product(soda_product, 2, "Soda");
    println!("Add product: {:#?}", ashop);
    ashop.restock(String::from("Pepsi"), 100);
    println!("Restock: {:#?}", ashop);
    ashop.move_product(1, "Sweets", String::from("ID234"));
    println!("Move: {:#?}", ashop);

    if let Some((row, zone)) = ashop.locate_product("ID234") {
        println!("Product ID234 is located in row {} and zone {}", row, zone);
    } else {
        println!("Product ID234 was not found.");
    }
}

#[derive(Debug)]
struct Shop {
    rows: Vec<Row>,
}

#[derive(Debug)]
struct Row {
    number: u8,
    zones: Vec<Zone>,
}

#[derive(Debug)]
struct Zone {
    name: &'static str,
    products: Vec<Product>,
}

#[derive(Debug)]
struct Product {
    id: String,
    name: String,
    exp_date: u16,
    price: f64,
    stock: u16,
}

impl Shop {
    fn new() -> Self {
        let mut rows = Vec::new();
        rows.push(Row::new(
            1,
            vec![Zone::new("Salty"), Zone::new("Sweets"), Zone::new("Drinks")],
        ));
        rows.push(Row::new(2, vec![Zone::new("Soda"), Zone::new("Water"), Zone::new("Alcohol")]));
        rows.push(Row::new(
            3,
            vec![Zone::new("Yogurt"), Zone::new("Butter"), Zone::new("Salad")],
        ));

        Self { rows }
    }

    fn locate_product(&self, product_id: &str) -> Option<(u8, &str)> {
        for row in &self.rows {
            for zone in &row.zones {
                if zone.products.iter().any(|p| p.id == product_id) {
                    return Some((row.number, zone.name));
                }
            }
        }
        None
    }

    fn restock(&mut self, product: String, amount: u16) {
        for row in &mut self.rows {
            for zone in &mut row.zones {
                if let Some(product) = zone.products.iter_mut().find(|p| p.name == product) {
                    product.stock += amount;
                    return;
                }
            }
        }
    }

    fn add_product(&mut self, new_product: Product, add_row: u8, add_zone: &str) {
        if let Some(row) = self.rows.iter_mut().find(|r| r.number == add_row) {
            if let Some(zone) = row.zones.iter_mut().find(|z| z.name == add_zone) {
                zone.products.push(new_product);
            } else {
                println!("Adding product to store: The zone:{} was not found", add_zone);
            }
        } else {
            println!("Adding product to store: The row:{} was not found", add_row);
        }
    }

    fn move_product(&mut self, to_row: u8, to_zone: &str, id_to_move: String) {
        let mut product_to_move: Option<Product> = None;
        for row in &mut self.rows {
            for zone in &mut row.zones {
                println!("a222");
                println!("{}a", id_to_move);
                if let Some(pos) = zone.products.iter().position(|p| p.id == id_to_move) {
                    println!("222");
                    product_to_move = Some(zone.products.remove(pos));
                    break;
                }
            }
            if product_to_move.is_some() {
                break;
            }
        }
        if let Some(product) = product_to_move {
            for row in &mut self.rows {
                if row.number == to_row {
                    if let Some(zone) = row.zones.iter_mut().find(|z| z.name == to_zone) {
                        zone.products.push(product);
                        println!("Product moved.");
                        return;
                    } else {
                        println!("Product not moved.")
                    }
                }
            }
        } else {
            println!("Product with ID {} was not found", id_to_move);
        }
    }
}

impl Row {
    fn new(number: u8, zones: Vec<Zone>) -> Self {
        Row { number, zones }
    }
}

impl Zone {
    // TODO: Review life cycle
    fn new(name: &'static str) -> Self {
        Zone {
            name,
            products: Vec::new(),
        }
    }
}

impl Product {
    fn new(id: String, name: String, exp_date: u16, price: f64, stock: u16) -> Self {
        Self {
            name,
            id,
            exp_date,
            price,
            stock,
        }
    }

    fn change_price(&mut self, new_price: f64) {
        self.price = new_price;
    }
}

// --------------------------------------------------------------------------------------------------------------
// --------------------- Mutation e random arrays ---------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------

fn exercise_array_mutation() {
    let mut some_array: Vec<i32> = vec![1, 5, 6, 22, 54, 3];
    println!("The array is {:?}", some_array);

    some_array = some_array.into_iter().map(|x| x % 2).collect();
    println!("Altered array is: {:?}", some_array);

    some_array = some_array.into_iter().map(|x| x * 2).collect();
    println!("Altered array is: {:?}", some_array);

    some_array = some_array.into_iter().map(|x| x - 2).collect();
    println!("Altered array is: {:?}", some_array);

    some_array = some_array.into_iter().map(|x| x + 2).collect();
    println!("Altered array is: {:?}", some_array);
}
