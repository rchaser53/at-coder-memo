/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    a:[Usize1;n]
  }

  let mut memo = vec![];
  let mut seen = vec![false;n];
  
  let mut ci = 0;
  while !seen[ci] {
    seen[ci] = true;
    memo.push(ci);
    ci = a[ci];
  }
  memo.push(ci);
  
  let mut result = VecDeque::new();
  let mut seen = HashSet::new();

  for i in memo.into_iter().rev() {
    if seen.contains(&i) { break }
    seen.insert(i);
    result.push_front(i+1);
  }

  println!("{}", result.len());
  println!("{}", result.into_iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" "));
}