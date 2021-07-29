/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    x:usize
  }

  let a = 2 * n - 1;
  if x == 1 || x == a {
    println!("No");
    return
  }
  println!("Yes");
  let mut temp = (1..=a).collect::<Vec<usize>>();
  temp.remove(&x-2);
  temp.remove(&x-2);
  temp.remove(&x-2);
  let mut result = VecDeque::new();
  result.push_back(x-1);
  result.push_back(x);
  result.push_back(x+1);

  for i in 0..temp.len() {
    if i % 2 == 0 {
      result.push_front(temp[i]);
    } else {
      result.push_back(temp[i]);
    }
  }

  for v in result {
    println!("{}", v);
  }
}