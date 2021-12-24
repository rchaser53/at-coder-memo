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
  let _dict:Vec<usize> = readvec();
  let dict:Vec<isize> = readvec();

  let limit = 12000;
  let hlimit = limit/2;
  let mut memo = vec![false;limit];
  memo[hlimit+dict[0] as usize] = true;
  memo[hlimit-dict[0] as usize] = true;
  for i in 1..dict.len() {
    let v = dict[i] as usize;
    let mut new_memo = vec![false;limit];
    for j in 0..memo.len() {
      if !memo[j] { continue }
      if v <= j {
        new_memo[j-v] = true;
      }

      if j+v < limit {
        new_memo[j+v] = true;
      }
    }
    memo = new_memo;
  }

  let mut min = 1_000_000_000;
  for i in 0..memo.len() {
    if memo[i] {
      let v = (i as isize - hlimit as isize).abs();
      min = std::cmp::min(v, min);
    }
  }
  println!("{}", min);
}