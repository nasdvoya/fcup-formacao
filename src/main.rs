mod ex_concepts;
mod ex_data_structures;
mod ex_ownership;
mod ex_structs;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("this is not a exception");
    match input.trim() {
        "concepts" => ex_concepts::exercise_concepts(),
        "ownership" => ex_ownership::exercise_ownership(),
        "structs_store" => ex_structs::exercise_store(),
        "structs_library" => ex_structs::exercise_library(),
        "data_structures" => ex_data_structures::exercise_library(),
        _ => println!("Invalid input"),
    }
}
