mod fmt_display;
mod list_for_display;
mod fmt_of_args;

fn main() {
    println!("Hello, Rust!");
    fmt_display::test_fmt_display();
    list_for_display::list_display();
    fmt_of_args::write_args_fmt();
}