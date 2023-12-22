fn main() {
    println!("Hello, world!"); // regular  
    println!("My name is {} and my age is {}", "Farahnaz Hoque", 21); // default positional 
    println!("a + b = {}", 3 + 6); // can even print out mathematical expressions
    println!("{0} has a {2} and {0} has a {1}", "Farah", "dog", "cat"); // setting positions
    println!("first name: {fname}, last name: {lname}", fname="Farahnaz", lname="Hoque"); // assigning values
    println!{"binary: {:b}, hex: {:x}, oct: {:o}", 50, 50, 50} // converting 
    println!("array: {:?}", [1, 2, 3]); // converting complex things like array into string requires the debug function
}
