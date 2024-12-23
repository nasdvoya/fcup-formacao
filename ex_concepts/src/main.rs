use rand::Rng;

fn main() {
    exercise_concepts();
}

pub fn exercise_concepts() {
    one_try_guess_game();
    endless_guess_game();
    fibonacci();
    calculator();
    is_prime();
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
