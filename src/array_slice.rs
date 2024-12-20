fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

pub fn array_slice() {
    let _xs: [i32; 5] = [1, 2, 3, 4, 5];
    let _ys: [i32; 500] = [0; 500];

    println!("first element of the array: {}", _xs[0]);
    println!("second element of the array: {}", _xs[1]);

    println!("array _xs size: {}", _xs.len());

    println!("array occupies {} bytes", std::mem::size_of_val(&_xs));

    println!("borrow the whole array as a slice");
    analyze_slice(&_xs);

    println!("borrow a section of the array as a slice");
    analyze_slice(&_ys[1..4]);

    // println!("_xs[5] is {}", _xs[5]);
}
