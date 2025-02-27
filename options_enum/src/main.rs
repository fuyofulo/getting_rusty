fn main() {
    
    let s1 = String::from("Zaid");

    let ans = find_first_a(s1);

    match ans {
        None => println!("\nValue not found\n"),
        Some(val) => println!("\na found at index {}\n", val)
    }
}

fn find_first_a(str: String) -> Option<u32> {

    let mut index = 0;
    for c in str.chars() {
        if c == 'a' {
            return Some(index);
        }

        index = index + 1;

    }

    None




}







