use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let input: Vec<&str> = s.split("\n").collect();

    let first = input[0].parse::<i32>().unwrap();
    let second: Vec<i32> = input[1]
        .split_whitespace()
        .map(|i| i.parse::<i32>().unwrap())
        .collect();
    let a = first + second[0] + second[1];
    let b = input[2].clone();

    println!("{} {}", a, b);
}
