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
  let mut memo = vec![false;m+1];
  memo[0] = true;
  
  for j in 0..m {
    if !memo[j] { continue }
    for &v in &dict {
      let nv = j + v;
      if m < nv { continue }
      memo[nv] = true;
    }
  }
  
  if memo[m] {
    println!("Yes");
  } else {
    println!("No");
  }
}