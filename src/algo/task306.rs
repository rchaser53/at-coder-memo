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
  let line:Vec<usize> = readvec();
  let (n, m) = (line[0], line[1]);
  let vals:Vec<usize> = readvec();
  
  let inf = 1_000_000_000;
  let mut memo = vec![inf;n];
  memo[0] = 0;

  for i in 0..n {
    for j in 1..=m {
      let k = i + j;
      if n <= k {
        break
      }
      memo[k] = std::cmp::min(memo[k], memo[k-j] + j * vals[k]);
    }
  }
  
  println!("{}", memo[n-1]);
}