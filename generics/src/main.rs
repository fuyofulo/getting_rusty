fn main() {
    display_stuff(1, 2);
    display_stuff(String::from("hello"), String::from("World!"));

    println!("{}", sum(1, 2));
    println!("{}", sum(5.0, 6.0)); 
}

fn display_stuff<T: std::fmt::Display> (a: T, b: T) {
    println!("{a}");
    println!("{b}");
}

fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    return a + b;
}
