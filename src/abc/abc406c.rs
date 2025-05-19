/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    a:[usize;n]
  }
  
  let mut vec = vec![];
  let mut rev = '<';
  let mut count = 0;
  for i in 0..n-1 {
    if a[i] < a[i+1] {
      if rev == '>' {
        vec.push(count);
        rev = '<';
        count = 0;
      }
    } else {
      if rev == '<' {
        vec.push(count);
        rev = '>';
        count = 0;
      }
    }
    count += 1;
  }

  vec.push(count);
  if rev == '>' {
    vec.push(0);
  }
  
  let mut result = 0usize;
  for i in (2..vec.len()).step_by(2) {
    result += vec[i] * vec[i-2];
  }

  println!("{}", result);
}