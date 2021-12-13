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
  
  let inf = 1_000_000_000;
  let mut memo = vec![inf;n+1];
  memo[0] = 0;
  memo[1] = 1;
  
  for i in 1..n {
    if i * 3 <= n {
      memo[i*3] = std::cmp::min(memo[i*3], memo[i] + 1);
    }
    memo[i+1] = std::cmp::min(memo[i+1], memo[i] + 1);
  }
  
  println!("{}", memo[n]);
}

fn another() {
  let mut n:usize = readln();
  
  let mut i = 0;
  while 0 < n {
    i += 1;
    if n % 3 == 0 {
      n /= 3;
    } else {
      n -= 1;
    }
  }
  
  println!("{}", i);
}