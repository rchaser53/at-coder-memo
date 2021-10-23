/** OUTPUT **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input!{
    n:usize,
    s:Chars,
  }

  let mut memo = vec![];
  for i in 0..n {
    if s[i] == '0' {
      memo.push(i);
    }
  }

  if memo.len() == 1 {
    println!("-1");
    return
  } else if memo.is_empty() {
    println!(
      "{}",
      (1..=n).into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" ")
    );
    return
  }

  let first = memo.remove(0);
  memo.push(first);

  let mut ci = 0;
  let mut result = vec![];
  for i in 0..n {
    if s[i] == '1' {
      result.push(format!("{}", i + 1));
    } else {
      result.push(format!("{}", memo[ci] + 1));
      ci += 1;
    }
  }

  println!("{}", result.join(" "));
}