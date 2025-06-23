enum Shape{
    Rectangle(f32, f32),
    Square(f32),
    Circle(f32)
}

fn main() {
    let rect = Shape::Rectangle(15.5, 10.2);
    let sqr = Shape::Square(4.4);
    let crcl = Shape::Circle(13.2);
    println!("Area of the rectangle is : {}", calc_area(rect));
    println!("Area of the square is : {}", calc_area(sqr));
    println!("Area of the circle is : {}", calc_area(crcl));

}

fn calc_area(shape: Shape) -> f32 {
    let area = match shape {
        Shape::Rectangle(length, width) => length * width,
        Shape::Square(side) => side * side,
        Shape::Circle(r) => 3.14 * r * r,
    };
    area
}