pub fn exercise_ownership() {
    edit_string();
    array_mutation();

    let key = "key";
    let text = "Hello world!";
    let encrypted = vigenere(key, text);

    println!("Encrypted text: {}", encrypted);
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

fn array_mutation() {
    // TODO: Aceder string como uma array?
    let mut some_array: [i32; 6] = [1, 5, 6, 22, 54, 3];
    println!("They array is {:?}", some_array);

    for i in 0..some_array.len() {
        some_array[i] %= 2;
    }
    println!("Altered array is: {:?}", some_array);
    for i in 0..some_array.len() {
        some_array[i] *= 2;
    }
    println!("Altered array is: {:?}", some_array);
    for i in 0..some_array.len() {
        some_array[i] -= 2;
    }
    println!("Altered array is: {:?}", some_array);
    for i in 0..some_array.len() {
        some_array[i] += 2;
    }
    println!("Altered array is: {:?}", some_array);
}
// TODO: Something broke
fn vigenere(key: &str, text: &str) -> String {
    let key_bytes = key.as_bytes();
    let mut encrypted = String::new();

    for (i, &byte) in text.as_bytes().iter().enumerate() {
        let key_byte: u8 = key_bytes[i % key_bytes.len()] - b'a';
        let encrypted_byte = ((byte - b'a' + key_byte) % 26) + b'a';
        encrypted.push(encrypted_byte as char);
    }

    encrypted
}
