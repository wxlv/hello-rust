#![allow(dead_code)]

// 拥有隐式辨别值（implicit discriminator，从0开始）的枚举
enum Number {
    Zero,
    One,
    Two,
}

// 拥有显式辨别值（explicit discriminator）的枚举
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

pub fn enum_c_style() {
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);
    println!("two is {}", Number::Two as i32);

    // {:06x} 表示以十六进制打印，且至少占6位，不足的用0填充
    // {:06o} 表示以八进制打印，且至少占6位，不足的用0填充
    println!("roses are #{:#0b}", Color::Red as i32);
    println!("roses are #{:#0x}", Color::Red as i32);
    println!("roses are #{:#0X}", Color::Red as i32);
    println!("roses are #{:#0o}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}
