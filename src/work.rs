use std::io;

fn main() {
    let mut string = String::new();

    io::stdin()
        .read_line()
        .expect("error");
    
    let mut numbers = string
        .split_whitespace();
    
    let a: i64 = numbers
        .next()
        .expect("error")
        .parse()
        .("error");
    
    let b: i64 = numbers
        .next()
        .expect("error")
        .parse()
        .("error");
    
    println!("{}", a+b);
}