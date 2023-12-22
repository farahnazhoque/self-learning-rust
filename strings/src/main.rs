#[allow(unused_variables)] // to avoid getting the warning for unused variables

fn main() {
    // string slices:
    let cat = "Happy";
    println!("Normal cat: {}", cat); // "Happy"

    let cat:&'static str = "Fluffy";
    println!("Static cat: {}", cat); // "Fluffy"

    // String objects:
    let dog = String::new(); // this is the unused variable
    let mut dog = String::from("Max");
    println!("{}", dog); // "Max"

    // format macro
    let owner = format!("Hi, I am {}, owner of {}!", "Farah", dog); // creating a sentence
    println!("Formatted string: {}", owner); // "Hi, I am Farah, owner of Max!"

    // get the length of a string
    println!("Length of the dog string: {}", dog.len()); // "3"

    // pushing strings or characters to an existing string (almost like appending) -> the string has to be of type mut
    dog.push(' '); // adding a whitespace character after Max 
    dog.push_str("the dog"); // adding a string
    println!("{}", dog); // "Max the dog"

    // replacing words or anything in an existing string by creating a new string
    let new_dog = dog.replace("the", "is my");
    println!("{}", new_dog); // "Max is my dog"
}
