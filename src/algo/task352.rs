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
  let (n, a, b) = (dict[0], dict[1], dict[2]);
  let mut dict:Vec<usize> = readvec();

  for i in 0..dict.len() {
    dict[i] %= a;
  }

  let mut memo = vec![false;a];
  memo[0] = true;
  
  for i in 0..n {
    let mut new_memo = memo.clone();
    for j in 0..a {
      if !memo[j] { continue }
      let ni = (j + dict[i]) % a;
      new_memo[ni] = true;
      
    }
    memo = new_memo;
  }

  if memo[b] {
    println!("Yes");
  } else {
    println!("No");
  }
}