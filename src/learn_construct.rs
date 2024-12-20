use std::fmt::Display;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

fn matrix_reverse(matrix: Matrix) -> Matrix {
    let _matrix = matrix;
    Matrix(_matrix.0, _matrix.2, _matrix.1, _matrix.3)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})\r\n({},{})", self.0, self.1, self.2, self.3)
    }
}

pub fn construct_test() {
    let long_tuple = (1u8, 2u32, 3u64, -1i8, -2i32, -3i64, 0.1f32, 0.2f64, 'a', true);

    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);

    println!("tuple of tuples:{:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is :{:?}", pair);

    println!("pair reverse is :{:?}", reverse(pair));

    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("tuple deconstruct : {:?},{:?},{:?},{:?}", a, b, c, d);

    let _matrix = Matrix(1.1, 1.6, 12.34, 99.33);
    println!("struct Matrix: {:?},{:?},{:?},{:?}", _matrix.0, _matrix.1, _matrix.2, _matrix.3);
    println!("struct Matrix Display: \r\n{}", _matrix);
    println!("struct Matrix Display Reverse: \r\n{}", matrix_reverse(_matrix));
}
