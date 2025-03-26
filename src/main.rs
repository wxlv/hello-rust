mod conversion;
fn main() {
    let _rs = conversion::base::num_to_base36(103807307673765);
    println!("{}", _rs);

    let _rs1 = conversion::base::base36_to_num("2QFz1zaG4");
    println!("{}", _rs1);

    let digits = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_";

    println!("Hello, world! {}", digits.len());
}