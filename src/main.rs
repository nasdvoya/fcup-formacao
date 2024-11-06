use rand::{random, Rng};

fn main() {
    //one_try_guess_game();
    endless_guess_game();
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
    let number_to_guess: u8 = rand::thread_rng().gen_range(1..=2);
    let mut input = String::new();

    println!("I'm thinking of a number between 1 and 6");

    while input.trim() != number_to_guess.to_string() {
        println!("input is {}", input);
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        println!("Input is {}.ddd", input);

        if input.trim() == number_to_guess.to_string() {
            println!("You guessed it!")
        } else {
            println!("Wrong! Try again.")
        }
    }
}
