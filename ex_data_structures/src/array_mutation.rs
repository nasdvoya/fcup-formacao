pub fn exercise_array_mutation() {
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
