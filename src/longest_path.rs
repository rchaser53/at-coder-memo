use proconio::input;
use proconio::marker::*;

fn helper(
  list: &Vec<Vec<usize>>,
  dp: &mut Vec<usize>,
  seen: &mut Vec<bool>,
  v: usize
) -> usize {
  if seen[v] {
    return dp[v];
  }
  seen[v] = true;
  let mut fans = 0;
  for &to in list[v].iter() {
    let vv = helper(list, dp, seen, to) + 1;
    fans = std::cmp::max(fans, vv);
  }
  dp[v] = fans;
  fans
}

fn main() {
  input!{
    n: usize,
    m: usize,
    vals: [(Usize1, Usize1);m]
  }
  
  let mut dp = vec![0;n+1];
  let mut seen = vec![false;n];
  let mut list = vec![vec![];n];
  for (from, to) in vals {
    list[from].push(to);
  }
  
  let mut result = 0;
  for i in 0..n {
    let v = helper(&list, &mut dp, &mut seen, i);
    result = std::cmp::max(result, v);
  }
  
  println!("{}", result);
}