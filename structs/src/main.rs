struct Rect {
    height: u16,
    width: u16
}

impl Rect {
    fn area(&self) -> u16 {
        self.height * self.width
    }
    fn perimeter(&self) -> u16 {
        2 * self.height + 2 * self.width
    }
    fn debug() -> u8 {
        1
    }
}

fn main() {
    let rect1 = Rect{ width: 20, height: 10};
    println!("Area of the rectangle is {}", rect1.area());
    let rect2 = Rect{width: 15, height: 33};
    println!("Perimeter of the rectangle is {}", rect2.perimeter());
    println!("Call static function: {}", Rect::debug());
}
