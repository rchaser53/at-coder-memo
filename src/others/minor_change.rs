use proconio::input;

fn main() {
    input! {
        a: String,
        b: String,
    }
    let b: Vec<char> = b.chars().collect();
 	let mut count = 0;
    for (i, c) in a.chars().enumerate() {
       if b[i] != c {
         count += 1;
       }
    }
    println!("{}", count);
}