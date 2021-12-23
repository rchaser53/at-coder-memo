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
  let dict2:Vec<isize> = readvec();
  let mut memo = vec![-1;m+1];
  memo[0] = 0;
  
  for i in 0..n {
    let mut new_memo = memo.clone();
    for j in 0..m {
      if memo[j] == -1 { continue }
      let ni = j + dict[i];
      if m < ni { continue }
      new_memo[ni] = std::cmp::max(memo[ni], memo[j]+dict2[i]);
    }
    memo = new_memo;
  }

  let mut result = 0;
  for v in memo {
    result = std::cmp::max(result, v);
  }
  println!("{}", result);
}