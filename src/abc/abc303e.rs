/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    uv:[(Usize1,Usize1);n-1]
  }

  let mut g = vec![vec![];n];
  for (u, v) in uv {
    g[u].push(v);
    g[v].push(u);
  }

  g.sort_by(|a,b| a.len().cmp(&b.len()));
  g.reverse();

  let mut ni = n as isize;
  let mut result = vec![];
  for arr in g {
    let v = arr.len() as isize;
    ni -= v + 1;
    result.push(v);
    if ni <= 0 {
      break
    }
  }
  result.sort();

  println!("{}", result.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}