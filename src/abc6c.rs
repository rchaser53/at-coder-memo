use proconio::{input, fastout};

fn main() {
  input!{
    n: isize,
    m: isize,
  }
  
  if m < n * 2
    || n * 4 < m
  {
    println!("-1 -1 -1");
    return
  }
  
  for c in 0..=n {
    if m < 4 * c { break }
    let b = m - 2 * (n + c);
    if b < 0 { continue }
    let a = n - b - c;
    if a < 0 { continue }
    
    println!("{} {} {}", a, b, c);
    return
  }
  
  println!("-1 -1 -1");
}