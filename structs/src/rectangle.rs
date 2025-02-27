pub struct Rectangle {
    pub length: u32,
    pub width: u32
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        return self.length*self.width;
    }

    pub fn perimeter(&self) -> u32 {
        return 2*(self.length+self.width);
    }

    pub fn static_function() -> String {
    let s1 = String::from("static function");
    return s1;
    }
}
