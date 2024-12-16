use clap::{App, Arg};

pub fn create_matcher() {
    let matches = App::new("My Test Program")
        .version("1.0")
        .author("John Doe")
        .about("Does awesome things")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .help("The file to read")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("num")
                .short("n")
                .long("number")
                .takes_value(true)
                .help("Five less than your favorite number"),
        )
        .get_matches();
    let my_file = matches.value_of("file").unwrap_or("input.txt");
    println!("The file passed is: {}", my_file);

    let num_str = matches.value_of("num");
    match num_str {
        None => println!("No number provided"),
        Some(s) => match s.parse::<i32>() {
            Ok(n) => {
                println!("Five less than {} is {}", n, n - 5);
            }
            Err(_) => {
                println!("{} is not a number", s);
            }
        },
    }
}
