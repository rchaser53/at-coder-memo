/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
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
  let n:usize = readln();
  let mut vals = vec![];
  for _ in 0..n {
    let a:Vec<usize> = readvec();
    vals.push((a[0], a[1]));
  }

  vals.sort_by(|a,b| a.1.cmp(&b.1));
  vals.reverse();

  let mut result = 0;
  let mut now = 0;

  while let Some((a,b)) = vals.pop() {
    if now <= a {
      result += 1;
      now = b;
    }
  }
  println!("{}", result);
}