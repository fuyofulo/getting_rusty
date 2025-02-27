use std::fs;

fn main() {
    
    let contents = fs::read_to_string("a.txt");

    match contents {
        Ok(contents) => println!("\n{}\n", contents),
        Err(_e) => println!("\nError while reading file !!!\n")
    }
}
