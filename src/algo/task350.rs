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
  let dict:Vec<usize> = readvec();
  let (n, m) = (dict[0], dict[1]);
  let dict:Vec<usize> = readvec();
  let inf = 1_000_000;
  let mut memo = vec![inf;m+1];
  memo[0] = 0;
  
  for i in 0..n {
    let mut new_memo = memo.clone();
    for j in 0..m {
      if memo[j] == inf { continue }
      let ni = j + dict[i];
      if m < ni { continue }
      new_memo[ni] = std::cmp::min(memo[ni], memo[j]+1);
    }
    memo = new_memo;
  }

  if memo[m] == inf {
    println!("-1");
  } else {
    println!("{}", memo[m]);
  }
}