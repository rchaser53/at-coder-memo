/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;


fn main() {
  input!{
    n:usize,
    m:usize,
  }

  let mut memo = vec![0;m];
  for i in 0..m {
    input! {
      c:usize,
      a:[Usize1;c]
    }
    let mut temp = 0;
    for v in a {
      temp += 1 << v;
    }
    memo[i] = temp;
  }

  let mut result = 0;
  let need = (1 << n)-1;
  let limit = 1 << m;
  for i in 0..limit {
    let mut temp = 0;
    for j in 0..m {
      if i >> j & 1 == 1 {
        temp |= memo[j];
      }
    }
    if need == temp {
      result += 1;
    }
  }
  
  println!("{}", result);
}