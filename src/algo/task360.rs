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
  let mut n:usize = readln();
  let vals:Vec<usize> = readvec();
  let mut result = 0;
  let dict = [50,10,5,1];

  for i in 0..vals.len() {
      let d = dict[i];
      let v = std::cmp::min(n / d, vals[i]);
      n -= v * d;
      result += v;
  }
  println!("{}", result);
}
  