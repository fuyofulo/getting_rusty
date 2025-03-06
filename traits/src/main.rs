fn main() {
    let r = Rect {
        height: 10.0,
        width: 10.0
    };

    print_area_of_shape(r);

    let c = Circle {
        radius: 4.0
    };

    print_area_of_shape(c);
}

trait Shape {
    fn area(&self) -> f32;
}

fn print_area_of_shape<T: Shape>(s: T) {
    println!("{}", s.area());
}

struct Rect {
    height: f32,
    width: f32
}

struct Circle {
    radius: f32
}

impl Shape for Rect {
    fn area(&self) -> f32 {
        return self.height*self.width
    }
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        return self.radius*self.radius
    }
}






