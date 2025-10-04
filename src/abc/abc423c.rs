/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    r:usize,
    l:[usize;n]
  }

  let mut left = (&l[0..r]).into_iter().map(|x| *x).collect::<Vec<usize>>();
  let mut right = l[r..n].into_iter().map(|x| *x).collect::<Vec<usize>>();
  // println!("{:?} {:?}", left, right);
  left.reverse();

  while !left.is_empty() {
    let li = left.len()-1;
    if left[li] == 1 {
      left.pop();
    } else {
      break;
    }
  }

  while !right.is_empty() {
    let ri = right.len()-1;
    if right[ri] == 1 {
      right.pop();
    } else {
      break;
    }
  }

  let mut result = 0;
  for v in left {
    if v == 1 {
      result += 1;    
    }
    result += 1;
  }

  for v in right {
    if v == 1 {
      result += 1;
    }
    result += 1;
  }

  println!("{}", result);
}