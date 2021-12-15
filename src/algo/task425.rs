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

fn helper(memo: &mut Vec<Option<usize>>, x:usize) -> usize {
  if let Some(v) = memo[x] {
    v
  } else {
    let v1 = helper(memo, x - 1);
    let v2 = helper(memo, x - 2);
    memo[x] = Some(v1+v2);
    v1+v2
  }
}

fn main() {
  let ab: Vec<usize> = readvec();
  let n = ab[0];
  
  let mut memo = vec![None;n+1];
  memo[0] = Some(0);
  memo[1] = Some(1);

  println!("{}", helper(&mut memo, n));
}