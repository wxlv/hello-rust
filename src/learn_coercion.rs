
// 不显示类型转换产生的溢出警告
#![allow(overflowing_literals)]
pub fn coercion_test(){
    let decimal = 65.4321_f32;

    // 错误！不提供隐式转换
    // let integer:u8 = decimal;

    let integer = decimal as u8;
    let chatacter = integer as char;

    println!("Casting: {} -> {} -> {}",decimal,integer,chatacter);

    // 当把任何类型转换为无符号类型T时，会不断的加上或减去（std::T::MAX + 1）直到位于新类型T的范围内
    // 1000已经在u16范围内
    println!("1000 as u16 is:{}", 1000u16);
    // 1000 - 256 - 256 -256 = 232
    // 事实上的处理方式是：从最低有效位（LSB,last significant bits）开始保留8位，然后剩余位置，直到最高有效位（MSB,most significant bits）都被抛弃。
    // MSB是二进制的最高位，LSB就是二进制最低位，按日常书写习惯就是最左边一位和最右边一位。
    println!("1000 as u8 is:{}", 1000u8);
    // -1 + 256 = 255
    println!("-1 as u8 is:{}",(-1i8) as u8);
    // 对正数，就和取模一样，得到232
    println!("1000 mod 256 is:{}", 1000 % 256);

    // 当转换到有符号类型时，（位操作的）结果就和“先转换到无符号类型，如果MSB是1，则该值为负”是一样的。
    // 当然如果数值已经在目标类型的范围，直接放进去就是了
    println!("128 as a i16 is:{}", 128i16);

    println!("128 as a i8 is:{}",128 as i8);

    println!("232 as a i8 is:{}",232 as i8);

    let parsed:i32 = "5".parse().unwrap();
    let turbo_parsed =  "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!("parsed + turbo_parsed is {}", sum);

}