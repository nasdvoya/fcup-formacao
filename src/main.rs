use mercearia::Product;
use rand::Rng;
mod mercearia;

fn main() {
    let mut product = Product::new(String::from("1223"), String::from(""), 33, 2.45, 1);
    println!("Here is a product {:?}", product);
    product.change_price(3.22);
    println!("Here is a product again {:?}", product);
    product.update_stock(3);
    println!("Product again {:?}", product);
}
fn enums() {
    // sum types
    // sao "tagged unions"
    // Obrigatoriamente ter o tamanho do maior membro do union
    // Podem ter blocos de implementação, funcoes associadas e metodos
    // "_" no ultimo branch do match // WARNING: Evitar
    // valor so eh consumido quando atribuimos valor no match
}

fn test(key: &str, text: &str) -> String {
    let key_bytes = key.as_bytes();
    let mut encrypted = String::new();

    for (i, &byte) in text.as_bytes().iter().enumerate() {
        let key_byte: u8 = key_bytes[i % key_bytes.len()] - b'a';
        let encrypted_byte = ((byte - b'a' + key_byte) % 26) + b'a';
        encrypted.push(encrypted_byte as char);
    }

    encrypted
}

fn vigenere(key: &str, input: &str) {
    let alphabet: Vec<char> = ('a'..'z').collect();
    let password_collection: Vec<char> = key.chars().collect();
    let mut cipher: Vec<char> = Vec::new();
    // iterate over input
    for (iteration, input_char) in input.chars().enumerate() {
        let cipher_char: &char = &password_collection[iteration];
        if let Some(index) = alphabet.iter().position(|&c| c == *cipher_char) {
            cipher.push(alphabet[1]);
            // gives number of positions to move
        }
        for (key_i, key_char) in key.chars().enumerate() {
            if key_i == iteration {
                for (alpha_i, alpha_char) in alphabet.iter().enumerate() {
                    if key_char == alpha_char.clone() {}
                }
            }
        }
    }
    // use the position of key to shift
    println!("Alpha: {:?}", alphabet);
}

fn new_vigenere(key: &str, input: &str) {
    let alphabet: Vec<char> = ('a'..'z').collect();
}

#[derive(Eq, PartialEq, Debug)]
struct Book {
    title: String,
    author: String,
    borrowed: bool,
}

// TODO: What is debug under the hood
#[derive(Debug)]
struct Library {
    books: Vec<Book>,
}

impl Library {
    fn new() -> Self {
        let mut books = Vec::new();
        books.push(Book {
            title: String::from("Book1"),
            author: String::from("Author1"),
            borrowed: false,
        });
        books.push(Book {
            title: String::from("Book2"),
            author: String::from("Author2"),
            borrowed: false,
        });
        books.push(Book {
            title: String::from("Book3"),
            author: String::from("Author3"),
            borrowed: false,
        });
        Library { books }
    }
    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }
    fn remove_book(&mut self) {
        // TODO: Isto?
        self.books.pop();
    }
    fn borrow_book(&mut self, title: &str) {
        for book in &mut self.books {
            if book.title == title {
                book.borrowed = true;
            } else {
                println!("Book not found {}", title);
            }
        }
    }
    fn return_book(&mut self, title: &str) {
        for book in &mut self.books {
            if book.title == title {
                book.borrowed = false;
            } else {
                println!("Book not found {}", title);
            }
        }
    }
}

fn edit_string() -> () {
    // Get input
    println!("Please enter the text you want to work on:");
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed");
    println!("You have entered: {:?}", input.trim());

    // Chose an option
    let mut action: String = String::new();
    loop {
        println!("Edit your input [del: char/word; upcase: char/word; locase: char/word;]: ");
        std::io::stdin().read_line(&mut action).expect("Weird");
        let pr_action: &str = action.trim();
        if pr_action.starts_with("del:") {
            let to_delete: String = pr_action.replace("del:", "");
            println!("Result: {:?}", input.replace(to_delete.as_str(), ""));
        } else if pr_action.starts_with("upcase:") {
            println!("Result: {:?}", input.to_uppercase());
        } else if pr_action.starts_with("locase:") {
            println!("Result: {:?}", input.to_lowercase());
        } else {
            return;
        }
        input.clear();
        action.clear();
        println!("Please enter a NEW input the text you want to work on:");
        std::io::stdin().read_line(&mut input).expect("Failed");
    }
}

// fn tupulo() {
//     // TODO: Check this
//     // let str: String = String::new();
//     // let str_ref: &String = &str;
//     // let str_2: String = *str_ref;
//     let tuple = (500, 100);
//     let (x, y) = tuple;
//     println!("Isto vai destrui o tupulo, Valor {x} , {y}");
// }
//
fn one_try_guess_game() -> () {
    println!("Computer says what?");
    let computer_says: u8 = rand::thread_rng().gen_range(1..=3);

    let mut input = String::new();
    let _answer = std::io::stdin().read_line(&mut input).expect("Failed to read line");

    if input.trim() == computer_says.to_string() {
        println!("You guessed right! {} is the correct answer", computer_says)
    } else {
        println!("Computers says no.. {} is the correct answer", computer_says)
    }
}

fn endless_guess_game() -> () {
    let number_to_guess: u8 = rand::thread_rng().gen_range(1..=6);
    let mut input = String::new();

    println!("I'm thinking of a number between 1 and 3");

    while input.trim().to_string() != number_to_guess.to_string() {
        input.clear();

        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim() == number_to_guess.to_string() {
            println!("You guessed it!")
        } else {
            println!("Wrong! Try again.")
        }
    }
}

fn fibonacci() -> () {
    println!("Input fibonacci limit: ");
    let mut input = String::new();
    let mut op1: u8 = 1;
    let mut op2: u8 = 0;
    let mut result: u8 = 0;
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let limit: u8 = input.trim().parse().expect("Please enter a valid number");

    while result < limit {
        result = op2 + op1;
        op1 = op2;
        op2 = result;
        println!("Fibonacci: {}", result);
    }
}

fn calculator() -> () {
    println!("Ugh, math.");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("this is not a exception");

    if input.trim().to_string().contains("+") {
        cal_add(input);
    } else if input.trim().to_string().contains("-") {
        cal_sub(input);
    } else if input.trim().to_string().contains("*") {
        cal_mul(input);
    } else if input.trim().to_string().contains("/") {
        cal_div(input);
    }
}

fn cal_add(input: String) {
    let foo: Vec<&str> = input.trim().split("+").collect();
    let input1: u8 = foo[0].parse().expect("Invalid first operand");
    let input2: u8 = foo[1].parse().expect("Invalid second operand");

    println!("The result of sum is: {}", input1 + input2);
}
fn cal_sub(input: String) {
    let foo: Vec<&str> = input.trim().split("-").collect();
    let input1: u8 = foo[0].parse().expect("Invalid first operand");
    let input2: u8 = foo[1].parse().expect("Invalid second operand");

    println!("The result of sub is: {}", input1 - input2);
}
fn cal_mul(input: String) {
    let foo: Vec<&str> = input.trim().split("*").collect();
    let input1: u8 = foo[0].parse().expect("Invalid first operand");
    let input2: u8 = foo[1].parse().expect("Invalid second operand");

    println!("The result of mul is: {}", input1 * input2);
}
fn cal_div(input: String) {
    let foo: Vec<&str> = input.trim().split("/").collect();
    let input1: u8 = foo[0].parse().expect("Invalid first operand");
    let input2: u8 = foo[1].parse().expect("Invalid second operand");

    println!("The result of div is: {}", input1 % input2);
}

fn is_prime() -> bool {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).expect("Weird");

    let number: u32 = input.trim().parse().expect("Could not parse");

    if number <= 1 {
        println!("Is not a prime");
    }

    for i in 2..number {
        if number % 2 == 0 {
            println!("Is not a prime {:?}", i);
            return false;
        }
    }

    println!("{:?} is a prime number", number);
    true
}

fn array_mutation() {
    // TODO: Aceder string como uma array?
    let mut some_array: [i32; 6] = [1, 5, 6, 22, 54, 3];
    println!("They array is {:?}", some_array);

    for i in 0..some_array.len() {
        // add, sub, div, mult
        some_array[i] %= 2;
    }
    println!("Altered array is: {:?}", some_array);
    for i in 0..some_array.len() {
        // add, sub, div, mult
        some_array[i] *= 2;
    }
    println!("Altered array is: {:?}", some_array);
    for i in 0..some_array.len() {
        // add, sub, div, mult
        some_array[i] -= 2;
    }
    println!("Altered array is: {:?}", some_array);
    for i in 0..some_array.len() {
        // add, sub, div, mult
        some_array[i] += 2;
    }
    println!("Altered array is: {:?}", some_array);
}
