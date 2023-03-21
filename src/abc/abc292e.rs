/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Reverse;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    l:usize,
    n1:usize,
    n2:usize,
    vl1:[(usize,usize);n1],
    vl2:[(usize,usize);n2],
  }
  let mut que1 = vl1.into_iter().collect::<VecDeque<(usize,usize)>>();
  let mut que2 = vl2.into_iter().collect::<VecDeque<(usize,usize)>>();
  let mut result = 0usize;

  while !que1.is_empty() && !que2.is_empty() {
    let (v1, mut num1) = que1.pop_front().unwrap();

    while 0 < num1 && !que2.is_empty() {
      let (v2, num2) = que2[0];
      if num1 > num2 {
        num1 -= num2;
        if v1 == v2 {
          result += num2;
        }
        que2.pop_front();
      } else if num1 == num2 {
        if v1 == v2 {
          result += num1;
        }
        que2.pop_front();
        break
      } else {
        if v1 == v2 {
          result += num1;
        }

        que2[0].1 -= num1;
        break
      }
    }
  }
  println!("{}", result);
}