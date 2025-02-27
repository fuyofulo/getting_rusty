pub enum Shape {
    Square(f32),
    Circle(f32),
    Rectangle(f32, f32)
}

impl Shape {

    pub fn area(&self) -> f32 {
        match &self{
            Shape::Square(side) => side*side,
            Shape::Circle(radius) => 3.14*2.0*radius,
            Shape::Rectangle(length, width) => length*width
        }
    }
}
