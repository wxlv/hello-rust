#![allow(dead_code)]
pub fn while_test() {
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }
}

pub fn for_test() {
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

pub fn for_iter_test() {
    let mut names = vec!["Alice", "Bob", "Charlie", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "This is a rustacrean among us!",
            _ => "Hello",
        }
    }
    println!("{:?}", names);
}
