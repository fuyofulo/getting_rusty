fn main() {
    let r = Rect{
        width: 10,
        height: 10
    };

    println!("{}", r.area());
}


struct Rect<T> {
    width: T,
    height: T
}

impl <T: std::ops::Mul<Output = T> + Copy> Rect<T> {
    fn area(&self) -> T {
        return self.width * self.height
    }
}



