use rand::{random, Rng};

fn main() {
    calculator();
    endless_guess_game();
    one_try_guess_game();
}

fn tupulo() {
    let tuple = (500, 100);
    let (x, y) = tuple;
    println!("Isto vai destrui o tupulo, Valor {x} , {y}");
}

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

fn calculator() -> () {
    println!("Chose an operation, add, sub, mul, div");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed while chosing operation");

    if input.trim().to_string() == "add" {
        cal_add();
    } else if input.trim().to_string() == "sub" {
        cal_sub();
    } else if input.trim().to_string() == "mul" {
        cal_mul();
    } else if input.trim().to_string() == "div" {
        cal_div();
    }
}

fn cal_add() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("First input:");
    std::io::stdin().read_line(&mut input1).expect("Failed to read line for input1");
    println!("Second input:");
    std::io::stdin().read_line(&mut input2).expect("Failed to read line for input2");

    let first_int = input1.trim().parse::<u8>().expect("Failed to parse the input 1 to u8");
    let second_int = input2.trim().parse::<u8>().expect("Failed to parse the input 1 to u8");

    println!("The result of sum is: {}", first_int + second_int);
}
fn cal_sub() {}
fn cal_mul() {}
fn cal_div() {}
