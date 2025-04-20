#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area (rect: &Rectangle) -> f32 {
    let Rectangle { top_left: Point{x: x1, y: y1}, 
        bottom_right: Point {x: x2, y: y2},
    } = rect;
    (x2 - x1) * (y2 - y1)
}
fn main () {
    let rect = Rectangle {
        top_left: Point{x: 2., y: 3.},
        bottom_right: Point {x: 8., y: 4.},
    };
    println!(" Area of rectangle {:?} is: { }",rect, rect_area(&rect));
}