/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;

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
  
  let mut memo = vec![0;n+1];
  memo[0] = 1;
  memo[1] = 1;
  
  for i in 2..=n {
    let mut new_memo = memo.clone();
    new_memo[i] += memo[i-2] + memo[i-1];
    memo = new_memo;
  }
  
  println!("{}", memo[n]);
}