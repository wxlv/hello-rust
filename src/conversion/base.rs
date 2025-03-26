const DIGITS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_";

fn decimal_to_base(num: i128, base: u32) -> String {
    if base < 2 || base > 72 {
        return String::from("不支持的进制");
    }

    let mut result = String::new();
    let is_nagative = num < 0;
    let mut abs_num = num.abs() as u128;

    if abs_num == 0 {
        return String::from("0");
    }

    while abs_num > 0 {
        let digit = (abs_num % base as u128) as usize;
        result.insert(0, DIGITS.chars().nth(digit).unwrap());
        abs_num /= base as u128;
    }

    if is_nagative {
        result.insert(0, '-');
    }
    result
}

fn base_to_decimal(num: &str, base: u32) -> i128 {
    if base < 2 || base > 72 {
        return 0;
    }

    let mut result = 0;
    let mut is_nagative = false;
    let mut chars = num.chars();

    if num.starts_with('-') {
        is_nagative = true;
        chars.next();
    }

    for c in chars {
        let digit = DIGITS.find(c).unwrap();
        result = result * base as i128 + digit as i128;
    }

    if is_nagative {
        -result
    } else {
        result
    }
}

pub fn num_to_base36(num: i128) -> String {
    decimal_to_base(num, 36)
}

pub fn base36_to_num(num: &str) -> i128 {
    base_to_decimal(num, 36)
}
