use std::io;

fn main() {
    let mut input:String = String::new();
    println!("Please write your message:");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Your message was: {}", input);
        }
        Err(e) => {
            println!("Something went wrong: {}", e);
        }
    }
}
