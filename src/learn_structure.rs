#[derive(Debug)]
#[allow(dead_code)]
struct Person {
    name: String,
    age: u8,
}
#[allow(dead_code)]
struct Unit;
#[allow(dead_code)]
struct Pair(i32, f32);

#[derive(Debug, Clone)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle { top_left, bottom_right } = rect;
    let Point { x: top_x, y: top_y } = top_left;
    let Point { x: bottom_x, y: bottom_y } = bottom_right;
    (top_x - bottom_x).abs() * (top_y - bottom_y).abs()
}

fn square(point: Point, len: f32) -> Rectangle {
    let Point { x, y } = point;
    Rectangle {
        top_left: Point { x, y: y + len },
        bottom_right: Point { x: x + len, y },
    }
}

pub fn structure_test() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point = Point { x: 10.3, y: 0.4 };

    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..point };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point { x: left_edge, y: top_edge } = point;
    let _rectangle = Rectangle { top_left: Point { x: left_edge, y: top_edge }, bottom_right: bottom_right };
    println!("{:?}", _rectangle);

    let _unit = Unit;
    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    let rect = Rectangle { top_left: Point { x: 0.0, y: 10.0 }, bottom_right: Point { x: 10.0, y: 0.0 } };
    println!("rectangle area: {}", rect_area(rect));

    let _square = square(point.clone(), 5.0);
    println!("square: {:?}", _square);
    println!("square area: {}", rect_area(square(point, 5.0)));
}
