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
  let (n, m, k) = (dict[0], dict[1], dict[2]);
  let dict:Vec<usize> = readvec();
  let inf = 1_000_000;
  let mut memo = vec![inf;m+1];
  memo[0] = 0;

  for v in dict {
    let mut new_memo = memo.clone();
    for i in 0..m {
      let ni = i + v;
      if m < ni {
        continue
      }
      new_memo[ni] = std::cmp::min(memo[i] + 1, new_memo[ni]);
    }
    memo = new_memo;
  }
  
  if k < memo[m] {
    println!("No");
  } else {
    println!("Yes");
  }
}