/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    a:usize,
    b:usize,
    s:Chars
  }

  let mut s = s.into_iter().collect::<VecDeque<char>>();
  let mut result = 10usize.pow(16);
  for x in 0..=n {
    let mut temp = a*x;
    for i in 0..n/2 {
      if s[i] != s[n-1-i] {
        temp += b;
      }
    }
    result = result.min(temp);
    let c = s.pop_front().unwrap();
    s.push_back(c);
  }

  println!("{}", result);
}