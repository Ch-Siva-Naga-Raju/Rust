fn main() {
    enum Shape {
        Rectangle(f32,f32),
        Square(f32),
        Circle(f32)
    }

    fn area(shape:Shape) -> f32{
        let ans = match shape{
            Shape::Rectangle(length,width, ) => length * width ,
            Shape::Square(side) => return side * side,
            Shape::Circle(radius) => return 3.14 * radius * radius
        };
        return ans;
    }
    let rect:Shape = Shape::Rectangle(32.0,23.0);
    let sqr: Shape = Shape::Square(12.0);
    let circle: Shape = Shape::Circle(15.0);
    println!("Area is: ", );

    let circle_area: f32 = area(circle);
    let square_area: f32 = area(sqr);
    let rect_area: f32 = area(rect);
    println!("Circle area is: {}", circle_area);
    println!("Square area is: {}", square_area);
    println!("Rectangle area is: {}", rect_area);
    struct Rect {
        width: u32,
        height:u32
    }

    impl Rect{
        fn area(&self) -> u32{
            return self.width * self.height
        }
    }
    let rect = Rect{ width: 43, height:21};
    println!("Area of the rectangle is: {}", rect.area());

    // Pass string as mutable Reference
    let mut str = String::from("Hello");
    change_str(&mut str);
}

fn change_str(str: &mut String) {
    str.push_str(" World");
    print!("The modified String is : {str}");
}