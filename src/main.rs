use rand::Rng;

fn main() {
    let mut option = String::new();
    println!(
        "Chose exercise: is_prime = 1; edit_string = 2, array_mutation = 3, calculator = 4,
        one_try_guess_game = 5, endless_guess_game=6, fibonacci=7, library=8, vigenere=9"
    );
    std::io::stdin().read_line(&mut option).expect("Pack on exercise choice");
    let option = option.trim();
    if option == "1" {
        is_prime();
    } else if option == "2" {
        edit_string();
    } else if option == "3" {
        array_mutation();
    } else if option == "4 " {
        calculator();
    } else if option == "5 " {
        one_try_guess_game();
    } else if option == "6 " {
        endless_guess_game();
    } else if option == "7 " {
        fibonacci();
    } else if option == "8" {
        let mut lib = Library::new();
        println!("Current lib state: {:?}", lib);
        lib.remove_book();
        println!("Current lib state: {:?}", lib);
        let some_book = Book {
            title: String::from("New book"),
            author: String::from("New author"),
            borrowed: false,
        };
        lib.add_book(some_book);
        println!("Current lib state: {:?}", lib);
        lib.borrow_book("New book");
        println!("Current lib state: {:?}", lib);
        lib.return_book("New book");
        println!("Current lib state: {:?}", lib);
    } else if option == "9" {
    }
}

fn vigenere() {}

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
