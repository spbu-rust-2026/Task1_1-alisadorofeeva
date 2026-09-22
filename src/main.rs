use std::io;

fn main() {
    let mut string = String::new();

    io::stdin().read_line(&mut string).expect("error");

    let mut numbers = string.split_whitespace();

    let a: i64 = numbers.next().expect("error").parse().expect("error");

    let b: i64 = numbers.next().expect("error").parse().expect("error");

    println!("{}", a + b);
}
