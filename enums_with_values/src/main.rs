enum Shape {
    Square(f32),
    Circle(f32),
    Rectangle(f32, f32)
}

fn main() {
    let shape1 = Shape::Square(10.0);
    let shape2 = Shape::Circle(7.0);
    let shape3 = Shape::Rectangle(4.0, 5.0);

    let area_shape1 = calculate_area(shape1);
    println!("\n{}\n",area_shape1);
    
    let area_shape2 = calculate_area(shape2);
    println!("\n{}\n",area_shape2);

    let area_shape3 = calculate_area(shape3);
    println!("\n{}\n",area_shape3);

}

fn calculate_area (s: Shape) -> f32 {
    match s{
        Shape::Circle(radius) => 3.14*radius*radius,
        Shape::Square(side) => side*side,
        Shape::Rectangle(length, height) => length*height,
    }
}
