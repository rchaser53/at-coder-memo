/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn dfs(memo: &mut HashMap<usize,usize>, n:usize) -> usize {
  if n == 0 {
    return 1
  }

  if let Some(&v) = memo.get(&n) {
    v
  } else {
    let v1 = dfs(memo, n/2);
    let v2 = dfs(memo, n/3);
    memo.insert(n, v1+v2);
    v1+v2
  }
}

fn main() {
  input! {
    n:usize,
  }

  println!("{}", dfs(&mut HashMap::new(), n));
}