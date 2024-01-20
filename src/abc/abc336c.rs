/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    mut n:usize
  }
  n -= 1;

  let mut memo = vec![5usize];
  for _ in 0..25 {
    let li = memo.len()-1;
    memo.push(memo[li] * 5);
  }

  let mut result = vec![0;25+1];
  for i in (0..25).rev() {
    if memo[i] <= n {
      let v = n / memo[i];
      result[i+1] = v * 2;
      n %= memo[i];
    }
  }
  if n > 0 {
    result[0] = n * 2;
  }
  
  while !result.is_empty() && result[result.len()-1] == 0 {
    result.pop();
  }

  if result.is_empty() {
    println!("{}", 0);
  } else {
    result.reverse();
    println!("{}", result.into_iter().map(|v| v.to_string()).collect::<String>());
  } 
}