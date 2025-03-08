fn main() {
    println!("\nHello, world!\n");

    let result: u32 = sum(30000, 5);
    println!("{}\n", result);

    let answer: bool = is_even(35);
    println!("{}\n", answer);

    let mut name: String = String::from("Zaid Khan");
    println!("Name -> {}\n", name);

    name.push_str(" Khan");

    let vector1: Vec<i32> = vec![1,2,3,4,5];
    println!("vector -> {:?}\n", vector1);

    for i in 0..10 {
        println!("{}", i);
    }
    println!();
}

fn sum(a: u32, b:u32) -> u32 {
    return a+b;
}

fn is_even(n: u32) -> bool {
    return n % 2 == 0;
}

