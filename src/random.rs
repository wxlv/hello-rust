use rand::distributions::Standard;
use rand::{
    distributions::Alphanumeric, distributions::Uniform, prelude::Distribution, thread_rng, Rng,
};
use rand_distr::Normal;

/// Prints 5 random numbers to the console. The numbers are of
/// u8, u16, u32, i32, and f64 types.
pub fn random_number() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}

fn random_range_number() {
    let mut rng = rand::thread_rng();

    println!("Random range Integer: {}", rng.gen_range(0..10));
    println!("Random range Float: {}", rng.gen_range(0.0..10.0));
}

fn random_range_uniform_number() {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }
}

fn random_normal_number() {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(2.0, 3.0).unwrap();
    let v = normal.sample(&mut rng);
    println!("Random normal number: {}", v);
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

pub fn random_distribution_number() {
    let mut rng = rand::thread_rng();
    let rng_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();
    println!("Random distribution number: {:?}", rng_tuple);
    println!("Random distribution number: {:?}", rand_point);
}

pub fn random_sample_iter() {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(60)
        .map(char::from)
        .collect();
    println!("Random string: {}", rand_string);
}

pub fn random_custom_iter() {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_LENGTH: usize = 30;
    let mut rng = rand::thread_rng();
    let password: String = (0..PASSWORD_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    println!("Random custom password: {:?}", password);
}
