/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    a: [i32;n]
  }

  let mut a = a.into_iter().collect::<VecDeque<i32>>();
  let mut flag = true;
  while !a.is_empty() {
    if flag {
      if a[a.len()-1] == 0 {
        a.pop_back();
      } else {
        if a[0] == 1 {
          println!("No");
          return
        }
        a.pop_front();
        flag = !flag;
      }
    } else {
      if a[a.len()-1] == 1 {
        a.pop_back();
      } else {
        if a[0] == 0 {
          println!("No");
          return
        }
        a.pop_front();
        flag = !flag;
      }
    }
  }

  println!("Yes");
}