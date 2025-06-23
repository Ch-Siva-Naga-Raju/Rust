use std::fs::read_to_string;

fn main() {
    let result = read_to_string("./check.txt");
    match result {
        Ok(val) => println!("Success: {}", val),
        Err(e) => println!("Error: {}", e)
    }
}
