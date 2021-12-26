use std::collections::*;
use std::cmp::Reverse;

fn readln<T: std::str::FromStr>() -> T {
  let mut tmp = String::new();
  std::io::stdin().read_line(&mut tmp).ok();
  tmp.trim().parse().ok().unwrap()
}
fn readvec<T: std::str::FromStr>() -> Vec<T> {
  readln::<String>()
      .split_whitespace()
      .map(|x| x.parse().ok().unwrap())
      .collect()
}

fn main() {
  let _dict:Vec<usize> = readvec();
  let dict:Vec<isize> = readvec();
  let mut max = 0isize;
  for v in dict {
    if v <= 0 { continue }
    max += v;
  }

  println!("{}", max);
}