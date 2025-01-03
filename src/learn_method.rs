#![allow(dead_code)]
struct Point {
    x: f64,
    y: f64,
}
// 实现代码块，`Point`的所有方法都在这里给出
impl Point {
    //这是一个静态方法(static method)
    //静态方法不需要被实例调用
    //这类方法一般用作构造器(constructor)
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
    //另外一个静态方法，需要两个参数
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}
impl Rectangle {
    // 这是一个实例方法（instance method）
    // `&self`是`self:&Self`的语法糖（sugar），其中`Self`是方法调用者的类型。在这个实例中`Self` = `Rectangle`
    fn area(&self) -> f64 {
        // `self`通过点运算符来访问结构体字段
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        // `abs`是一个`f64`类型的方法，返回调用者的绝对值
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    //这个方法要去调用者是可变的，`&mut self`是`self:&mut Self`的语法糖
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;
        self.p1.y += y;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second);
    }
}

pub fn test_methods() {
    let rectangle = Rectangle { p1: Point::origin(), p2: Point::new(3.0, 4.0) };
    //实例方法通过点运算符来调用，注意第一个参数`&self`是隐式传递。
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle { p1: Point::origin(), p2: Point::new(2.0, 111.0) };
    square.translate(1.0, 14.0);
    println!("Square area: {}", square.area());
    println!("Square perimeter: {}", square.perimeter());

    let pair = Pair(Box::new(144), Box::new(2));
    pair.destroy();

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    //
    println!("2 to vec1 : {}", vec1.iter().any(|&x| x == 2));

    println!("2 to vec2 : {}", vec2.into_iter().any(|x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    println!("2 to vec1 : {}", array1.iter().any(|&x| x == 2));
    println!("2 to vec2 : {}", array2.into_iter().any(|x| x == 2));

    println!("array1 is {:?}, array2 is {:?}", array1, array2);
}
