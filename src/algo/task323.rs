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
  let mut vals:Vec<usize> = readvec();
  vals.sort();
  
  let mut memo = vec![false;n+1];
  memo[0] = true;

  for i in 0..=n {
    if memo[i] {
      for &j in &vals {
        let ni = i + j;
        if n < ni { continue }
        memo[ni] = true;
      }
    }
  }

  if memo[n] {
    println!("Yes");
  } else {
    println!("No");
  }
}