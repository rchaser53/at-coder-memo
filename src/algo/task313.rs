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
  let mut dict:Vec<Vec<usize>> = vec![];
  for _ in 0..n {
    dict.push(readvec());
  }

  let mut memo = vec![false;m+1];
  memo[0] = true;

  for v in dict {
    let (a,b) = (v[0], v[1]);
    let mut new_memo = memo.clone();
    for i in 0..m {
      if !memo[i] { continue }
      for j in 1..=b {
        let ni = i + a * j;
        if m < ni { break }
        new_memo[ni] = true;
      }

      if new_memo[m] {
        println!("Yes");
        return
      }
    }

    memo = new_memo;

  }
  println!("No");
}