/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    lr:[(isize,isize);n]
  }

  let mut temp = 0;
  for &v in &lr {
    temp += v.1;
  }
  if temp < 0 {
    println!("No");
    return
  }

  let mut result = vec![0;n];

  for i in 0..n {
    let (min, max) = lr[i];
    let diff = max - min;
    if 0 < temp {
      if diff <= temp {
        result[i] = min;
        temp -= diff;
      } else {
        result[i] = max - temp;
        temp = 0;
      }
    } else {
      result[i] = max;
    }
  }

  if 0 < temp {
    println!("No");
    return
  }

  println!("Yes");
  println!("{}", result.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}