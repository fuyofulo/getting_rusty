fn main() {

    let s = String::from("Zaid Khan");

    let response1 = length1(&s);
    println!("\nlength of {} is {}", s, response1);

    let (response2, s) = length2(s);
    println!("\nlength of {} is {}\n", s, response2);

}

// this function borrows the ownership
fn length1 (s1: &String) -> usize {
    return s1.len();
}

// this function takes the ownership and returns it back
fn length2 (s2: String) -> (usize, String) {
    return (s2.len(), s2);
}


