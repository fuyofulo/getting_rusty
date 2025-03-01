fn main() {
    display_stuff(1, 2);
    display_stuff(String::from("hello"), String::from("World!"));
}

fn display_stuff<T: std::fmt::Display> (a: T, b: T) {
    println!("{a}");
    println!("{b}");
}
